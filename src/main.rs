use poem::{listener::TcpListener, Route};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};
use subxt::{Config, OnlineClient, PolkadotConfig};

mod system;

#[subxt::subxt(runtime_metadata_path = "artifacts/metadata.scale")]
pub mod substrate_node {}

#[derive(Debug)]
struct Api<T: Config> {
    pub client: OnlineClient<T>,
}

impl<T: Config> Api<T> {
    pub async fn new() -> anyhow::Result<Self> {
        let api = OnlineClient::<T>::new().await?;

        Ok(Self { client: api })
    }
}

#[OpenApi]
impl<T: Config> Api<T> {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }

    #[oai(path = "/block_hash", method = "get")]
    async fn block_hash(&self, block_number: Query<Option<u32>>) -> PlainText<String> {
        match block_number.0 {
            Some(block_number) => {
                let block_hash = self
                    .client
                    .rpc()
                    .block_hash(Some(block_number.into()))
                    .await
                    .unwrap();

                PlainText(format!("{:?}", block_hash))
            }
            None => PlainText("block_hash".to_string()),
        }
    }

    #[oai(path = "/system/version", method = "get")]
    async fn version(&self) -> PlainText<String> {
        match self.system_version().await {
            Ok(version) => PlainText(format!("{}", version)),
            Err(e) => PlainText(format!("{:?}", e)),
        }
    }

}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // init api
    let api = Api::<PolkadotConfig>::new().await?;

    let api_service = OpenApiService::new(api, "substrate node rpc node", "1.0")
        .server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/", ui);

    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
        .map_err(|e| anyhow::anyhow!("{:?}", e))
}
