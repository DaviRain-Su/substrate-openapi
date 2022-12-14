use crate::Api;
use sp_runtime::ApplyExtrinsicResult;
use subxt::Error;
use subxt::{rpc_params, Config};

impl<T: Config> Api<T> {
    // accountNextIndex(accountId: AccountId): Index
    /// retrieves the next accountIndex as available on the node
    pub async fn account_next_index(&self, account_id: String) -> u32 {
        todo!()
    }

    // addLogFilter(directives: Text): Null
    /// Adds the supplied directives to the current log filter
    pub async fn add_log_filter(&self, directives: String) -> Option<()> {
        todo!()
    }

    // addReservedPeer(peer: Text): Text
    /// Adds a reserved peer
    pub async fn system_add_reserved_peer(&self, peer: String) -> Result<String, Error> {
        let params = rpc_params![peer];
        let result: String = self
            .client
            .rpc()
            .request("system_addReservedPeer", params)
            .await?;
        Ok(result)
    }

    // chain(): Text
    /// Retrieves the chain
    pub async fn system_chain(&self) -> Result<String, Error> {
        let params = rpc_params![];
        let result: String = self.client.rpc().request("system_chain", params).await?;
        Ok(result)
    }
    // chainType(): ChainType
    /// Retrieves the chain type
    pub async fn chain_type(&self) -> String {
        todo!()
    }
    // dryRun(extrinsic: Bytes, at?: BlockHash): ApplyExtrinsicResult
    /// Dry run an extrinsic at a given block
    pub async fn system_dry_run(
        &self,
        encoded_signed: &[u8],
        at: Option<T::Hash>,
    ) -> Result<ApplyExtrinsicResult, Error> {
        self.client.rpc().dry_run(encoded_signed, at).await
    }
    // health(): Health
    /// Return health status of the node
    pub async fn health(&self) -> String {
        todo!()
    }
    // localListenAddresses(): Vec<Text>
    /// The addresses include a trailing /p2p/ with the local PeerId, and are thus suitable to be passed to addReservedPeer or as a bootnode address for example
    pub async fn system_local_listen_addresses(&self) -> Result<Vec<String>, Error> {
        let params = rpc_params![];
        let result: Vec<String> = self
            .client
            .rpc()
            .request("system_localListenAddresses", params)
            .await?;
        Ok(result)
    }
    // localPeerId(): Text
    /// Returns the base58-encoded PeerId of the node
    pub async fn system_local_peer_id(&self) -> Result<String, Error> {
        let params = rpc_params![];
        let result: String = self
            .client
            .rpc()
            .request("system_localPeerId", params)
            .await?;
        Ok(result)
    }

    // name(): Text
    /// Retrieves the node name
    pub async fn system_name(&self) -> Result<String, Error> {
        let params = rpc_params![];
        let result: String = self.client.rpc().request("system_name", params).await?;
        Ok(result)
    }
    // networkState(): NetworkState
    // Returns current state of the network
    pub async fn network_state(&self) -> String {
        todo!()
    }
    // nodeRoles(): Vec<NodeRole>
    // Returns the roles the node is running as
    pub async fn node_roles(&self) -> Vec<String> {
        todo!()
    }
    // peers(): Vec<PeerInfo>
    /// Returns the currently connected peers
    pub async fn peers(&self) -> Vec<String> {
        todo!()
    }
    // properties(): ChainProperties
    /// Get a custom set of properties as a JSON object, defined in the chain spec
    pub async fn system_properties(&self) -> Result<String, Error> {
        todo!()
    }
    // todo peer_id need to define, give a corrent type
    // removeReservedPeer(peerId: Text): Text
    /// Remove a reserved peer
    pub async fn system_remove_reserved_peer(&self, peer_id: String) -> Result<String, Error> {
        let params = rpc_params![peer_id];
        let result: String = self
            .client
            .rpc()
            .request("system_removeReservedPeer", params)
            .await?;
        Ok(result)
    }
    // reservedPeers(): Vec<Text>
    /// Returns the list of reserved peers
    pub async fn system_reserved_peers(&self) -> Result<Vec<String>, Error> {
        let params = rpc_params![];
        let result: Vec<String> = self
            .client
            .rpc()
            .request("system_reservedPeers", params)
            .await?;
        Ok(result)
    }
    // resetLogFilter(): Null
    /// Resets the log filter to Substrate defaults
    pub async fn reset_log_filter(&self) -> Option<()> {
        todo!()
    }
    // syncState(): SyncState
    /// Returns the state of the syncing of the node
    pub async fn system_sync_state(&self) -> Result<(), Error> {
        todo!()
    }

    // version(): Text
    /// Retrieves the version of the node
    pub async fn system_version(&self) -> Result<String, Error> {
        let params = rpc_params![];
        let result: String = self.client.rpc().request("system_version", params).await?;
        Ok(result)
    }
}
