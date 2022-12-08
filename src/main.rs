use poem::{listener::TcpListener, Route};
use poem_openapi::{
    param::Query,
    payload::{Base64, Json, PlainText},
    OpenApi, OpenApiService,
};
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

    #[oai(path = "/blockHash", method = "get")]
    async fn block_hash(&self, block_number: Query<u32>) -> Json<String> {
        match self
            .client
            .rpc()
            .block_hash(Some(block_number.0.into()))
            .await
        {
            Ok(Some(block_number)) => Json(format!("{:?}", block_number)),
            Ok(None) => Json(format!("None")),
            Err(e) => Json(format!("{:?}", e)),
        }
    }

    /******************** System *****************************/

    // system_add_reserved_peer
    #[oai(path = "/system/addReservedPeer", method = "get")]
    async fn add_reserved_peer(&self, peer: Query<String>) -> Json<String> {
        match self.system_add_reserved_peer(peer.0).await {
            Ok(result) => Json(result),
            Err(e) => Json(format!("{:?}", e)),
        }
    }

    #[oai(path = "/system/chain", method = "get")]
    async fn chain(&self) -> Json<String> {
        match self.system_chain().await {
            Ok(result) => Json(result),
            Err(e) => Json(format!("{:?}", e)),
        }
    }

    //  // system_local_listen_addresses
    //  #[oai(path = "/system/dryRun", method = "get")]
    //  async fn dry_run(&self, encoded_signed: Query<Vec<u8>>, at: Query<Option<T::Hash>>) -> Result<Json<String>, String> {
    //      match self.system_dry_run(&encoded_signed.0, at.0).await {
    //          Ok(result) => Ok(Json(format!("{:?}", result))),
    //          Err(e) => Err(format!("{:?}", e)),
    //      }
    // }

    // system_local_listen_addresses
    #[oai(path = "/system/localListenAddresses", method = "get")]
    async fn local_listen_addresses(&self) -> Json<Vec<String>> {
        match self.system_local_listen_addresses().await {
            Ok(result) => Json(result),
            Err(e) => Json(vec![format!("{:?}", e)]),
        }
    }

    #[oai(path = "/system/localPeerId", method = "get")]
    async fn local_peer_id(&self) -> Json<String> {
        match self.system_local_peer_id().await {
            Ok(result) => Json(result),
            Err(e) => Json(format!("{:?}", e)),
        }
    }

    #[oai(path = "/system/name", method = "get")]
    async fn name(&self) -> Json<String> {
        match self.system_name().await {
            Ok(result) => Json(result),
            Err(e) => Json(format!("{:?}", e)),
        }
    }

    // #[oai(path = "/system/remove_reserved_peer", method = "get")]
    // async fn remove_reserved_peer(&self, peer_id: Query<String>) -> Json<String> {
    //     match self.system_remove_reserved_peer(peer_id.0).await {
    //         Ok(result) => Json(result),
    //         Err(e) => Json(format!("{:?}", e)),
    //     }
    // }

    #[oai(path = "/system/reservedPeers", method = "get")]
    async fn reserved_peers(&self) -> Json<Vec<String>> {
        match self.system_reserved_peers().await {
            Ok(result) => Json(result),
            Err(e) => Json(vec![format!("{:?}", e)]),
        }
    }

    #[oai(path = "/system/version", method = "get")]
    async fn version(&self) -> Json<String> {
        match self.system_version().await {
            Ok(result) => Json(result),
            Err(e) => Json(format!("{:?}", e)),
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
