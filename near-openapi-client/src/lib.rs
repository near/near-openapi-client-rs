//! NEAR Protocol JSON RPC API Client
//!
//! Generated from the NEAR OpenAPI specification.
pub use near_openapi_types as types;
#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[derive(Clone, Debug)]
///Client for NEAR Protocol JSON RPC API
///
///Version: 1.2.2
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}
impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "1.2.2"
    }
    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }
    fn client(&self) -> &reqwest::Client {
        &self.client
    }
    fn inner(&self) -> &() {
        &()
    }
}
impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    ///[Deprecated] Returns changes for a given account, contract or contract
    /// code for given block height or hash. Consider using changes instead.
    ///
    ///Sends a `POST` request to `/EXPERIMENTAL_changes`
    pub async fn experimental_changes<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExperimentalChanges,
    ) -> Result<
        ResponseValue<
            types::JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcStateChangesError,
        >,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "experimental_changes",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///[Deprecated] Returns changes in block for given block height or hash
    /// over all transactions for all the types. Includes changes like
    /// account_touched, access_key_touched, data_touched,
    /// contract_code_touched. Consider using block_effects instead
    ///
    ///Sends a `POST` request to `/EXPERIMENTAL_changes_in_block`
    pub async fn experimental_changes_in_block<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExperimentalChangesInBlock,
    ) -> Result<
        ResponseValue<
            types::JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcStateChangesError,
        >,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "experimental_changes_in_block",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Queries the congestion level of a shard. More info about congestion [here](https://near.github.io/nearcore/architecture/how/receipt-congestion.html?highlight=congestion#receipt-congestion)
    ///
    ///Sends a `POST` request to `/EXPERIMENTAL_congestion_level`
    pub async fn experimental_congestion_level<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExperimentalCongestionLevel,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcCongestionLevelResponseAndRpcChunkError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "experimental_congestion_level",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///[Deprecated] Get initial state and parameters for the genesis block.
    /// Consider genesis_config instead.
    ///
    ///Sends a `POST` request to `/EXPERIMENTAL_genesis_config`
    pub async fn experimental_genesis_config<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExperimentalGenesisConfig,
    ) -> Result<ResponseValue<types::JsonRpcResponseForGenesisConfigAndGenesisConfigError>, Error<()>>
    {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "experimental_genesis_config",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Returns the proofs for a transaction execution.
    ///
    ///Sends a `POST` request to `/EXPERIMENTAL_light_client_block_proof`
    pub async fn experimental_light_client_block_proof<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExperimentalLightClientBlockProof,
    ) -> Result<
        ResponseValue<
            types::JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcLightClientProofError,
        >,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "experimental_light_client_block_proof",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Returns the proofs for a transaction execution.
    ///
    ///Sends a `POST` request to `/EXPERIMENTAL_light_client_proof`
    pub async fn experimental_light_client_proof<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExperimentalLightClientProof,
    ) -> Result<
        ResponseValue<
            types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcLightClientProofError,
        >,
        Error<()>,
    >{
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "experimental_light_client_proof",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///[Deprecated] Returns the future windows for maintenance in current epoch
    /// for the specified account. In the maintenance windows, the node will not
    /// be block producer or chunk producer. Consider using maintenance_windows
    /// instead.
    ///
    ///Sends a `POST` request to `/EXPERIMENTAL_maintenance_windows`
    pub async fn experimental_maintenance_windows<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExperimentalMaintenanceWindows,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForArrayOfRangeOfUint64AndRpcMaintenanceWindowsError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "experimental_maintenance_windows",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///A configuration that defines the protocol-level parameters such as
    /// gas/storage costs, limits, feature flags, other settings
    ///
    ///Sends a `POST` request to `/EXPERIMENTAL_protocol_config`
    pub async fn experimental_protocol_config<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExperimentalProtocolConfig,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcProtocolConfigResponseAndRpcProtocolConfigError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "experimental_protocol_config",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Fetches a receipt by its ID (as is, without a status or execution
    /// outcome)
    ///
    ///Sends a `POST` request to `/EXPERIMENTAL_receipt`
    pub async fn experimental_receipt<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExperimentalReceipt,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcReceiptResponseAndRpcReceiptError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "experimental_receipt",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Contains the split storage information. More info on split storage [here](https://near-nodes.io/archival/split-storage-archival)
    ///
    ///Sends a `POST` request to `/EXPERIMENTAL_split_storage_info`
    pub async fn experimental_split_storage_info<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExperimentalSplitStorageInfo,
    ) -> Result<
        ResponseValue<
            types::JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcSplitStorageInfoError,
        >,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "experimental_split_storage_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Queries status of a transaction by hash, returning the final transaction
    /// result and details of all receipts.
    ///
    ///Sends a `POST` request to `/EXPERIMENTAL_tx_status`
    pub async fn experimental_tx_status<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExperimentalTxStatus,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcTransactionResponseAndRpcTransactionError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "experimental_tx_status",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Returns the current epoch validators ordered in the block producer order
    /// with repetition. This endpoint is solely used for bridge currently and
    /// is not intended for other external use cases.
    ///
    ///Sends a `POST` request to `/EXPERIMENTAL_validators_ordered`
    pub async fn experimental_validators_ordered<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForExperimentalValidatorsOrdered,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForArrayOfValidatorStakeViewAndRpcValidatorError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "experimental_validators_ordered",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Returns block details for given height or hash
    ///
    ///Sends a `POST` request to `/block`
    pub async fn block<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForBlock,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcBlockResponseAndRpcBlockError>, Error<()>>
    {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "block",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Returns changes in block for given block height or hash over all
    /// transactions for all the types. Includes changes like account_touched,
    /// access_key_touched, data_touched, contract_code_touched.
    ///
    ///Sends a `POST` request to `/block_effects`
    pub async fn block_effects<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForBlockEffects,
    ) -> Result<
        ResponseValue<
            types::JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcStateChangesError,
        >,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "block_effects",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///[Deprecated] Sends a transaction and immediately returns transaction
    /// hash. Consider using send_tx instead.
    ///
    ///Sends a `POST` request to `/broadcast_tx_async`
    pub async fn broadcast_tx_async<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForBroadcastTxAsync,
    ) -> Result<ResponseValue<types::JsonRpcResponseForCryptoHashAndRpcTransactionError>, Error<()>>
    {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "broadcast_tx_async",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///[Deprecated] Sends a transaction and waits until transaction is fully
    /// complete. (Has a 10 second timeout). Consider using send_tx instead.
    ///
    ///Sends a `POST` request to `/broadcast_tx_commit`
    pub async fn broadcast_tx_commit<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForBroadcastTxCommit,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcTransactionResponseAndRpcTransactionError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "broadcast_tx_commit",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Returns changes for a given account, contract or contract code for given
    /// block height or hash.
    ///
    ///Sends a `POST` request to `/changes`
    pub async fn changes<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForChanges,
    ) -> Result<
        ResponseValue<
            types::JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcStateChangesError,
        >,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "changes",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Returns details of a specific chunk. You can run a block details query
    /// to get a valid chunk hash.
    ///
    ///Sends a `POST` request to `/chunk`
    pub async fn chunk<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForChunk,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcChunkResponseAndRpcChunkError>, Error<()>>
    {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "chunk",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Queries client node configuration
    ///
    ///Sends a `POST` request to `/client_config`
    pub async fn client_config<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForClientConfig,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcClientConfigResponseAndRpcClientConfigError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "client_config",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Returns gas price for a specific block_height or block_hash. Using
    /// [null] will return the most recent block's gas price.
    ///
    ///Sends a `POST` request to `/gas_price`
    pub async fn gas_price<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForGasPrice,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcGasPriceResponseAndRpcGasPriceError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "gas_price",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Get initial state and parameters for the genesis block
    ///
    ///Sends a `POST` request to `/genesis_config`
    pub async fn genesis_config<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForGenesisConfig,
    ) -> Result<ResponseValue<types::JsonRpcResponseForGenesisConfigAndGenesisConfigError>, Error<()>>
    {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "genesis_config",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Returns the current health status of the RPC node the client connects
    /// to.
    ///
    ///Sends a `POST` request to `/health`
    pub async fn health<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForHealth,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForNullableRpcHealthResponseAndRpcStatusError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "health",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Returns the proofs for a transaction execution.
    ///
    ///Sends a `POST` request to `/light_client_proof`
    pub async fn light_client_proof<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForLightClientProof,
    ) -> Result<
        ResponseValue<
            types::JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcLightClientProofError,
        >,
        Error<()>,
    >{
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "light_client_proof",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Returns the future windows for maintenance in current epoch for the
    /// specified account. In the maintenance windows, the node will not be
    /// block producer or chunk producer.
    ///
    ///Sends a `POST` request to `/maintenance_windows`
    pub async fn maintenance_windows<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForMaintenanceWindows,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForArrayOfRangeOfUint64AndRpcMaintenanceWindowsError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "maintenance_windows",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Queries the current state of node network connections. This includes
    /// information about active peers, transmitted data, known producers, etc.
    ///
    ///Sends a `POST` request to `/network_info`
    pub async fn network_info<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForNetworkInfo,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcNetworkInfoResponseAndRpcNetworkInfoError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "network_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Returns the next light client block.
    ///
    ///Sends a `POST` request to `/next_light_client_block`
    pub async fn next_light_client_block<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForNextLightClientBlock,
    ) -> Result<
        ResponseValue<
            types::JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcLightClientNextBlockError,
        >,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "next_light_client_block",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///This module allows you to make generic requests to the network.
    ///
    ///The `RpcQueryRequest` struct takes in a [`BlockReference`](https://docs.rs/near-primitives/0.12.0/near_primitives/types/enum.BlockReference.html) and a [`QueryRequest`](https://docs.rs/near-primitives/0.12.0/near_primitives/views/enum.QueryRequest.html).
    ///
    ///The `BlockReference` enum allows you to specify a block by `Finality`,
    /// `BlockId` or `SyncCheckpoint`.
    ///
    ///The `QueryRequest` enum provides multiple variants for performing the
    /// following actions:
    /// - View an account's details
    /// - View a contract's code
    /// - View the state of an account
    /// - View the `AccessKey` of an account
    /// - View the `AccessKeyList` of an account
    /// - Call a function in a contract deployed on the network.
    ///
    ///Sends a `POST` request to `/query`
    pub async fn query<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForQuery,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcQueryResponseAndRpcQueryError>, Error<()>>
    {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "query",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Sends transaction. Returns the guaranteed execution status and the
    /// results the blockchain can provide at the moment.
    ///
    ///Sends a `POST` request to `/send_tx`
    pub async fn send_tx<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForSendTx,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcTransactionResponseAndRpcTransactionError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "send_tx",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Requests the status of the connected RPC node. This includes information
    /// about sync status, nearcore node version, protocol version, the current
    /// set of validators, etc.
    ///
    ///Sends a `POST` request to `/status`
    pub async fn status<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForStatus,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcStatusResponseAndRpcStatusError>, Error<()>>
    {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "status",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Queries status of a transaction by hash and returns the final
    /// transaction result.
    ///
    ///Sends a `POST` request to `/tx`
    pub async fn tx<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForTx,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcTransactionResponseAndRpcTransactionError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo { operation_id: "tx" };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Queries active validators on the network. Returns details and the state
    /// of validation on the blockchain.
    ///
    ///Sends a `POST` request to `/validators`
    pub async fn validators<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForValidators,
    ) -> Result<
        ResponseValue<types::JsonRpcResponseForRpcValidatorResponseAndRpcValidatorError>,
        Error<()>,
    > {
        let url = format!("{}/", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "validators",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
