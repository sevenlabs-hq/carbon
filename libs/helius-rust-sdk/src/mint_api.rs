use crate::error::Result;
use crate::types::{MintCompressedNftRequest, MintResponse};
use crate::Helius;

impl Helius {
    /// The easiest way to mint a compressed NFT (cNFT)
    ///
    /// # Arguments
    /// * `request` - A struct containing the various desired metadata of the cNFT to be minted, such as its name, symbol, URI, and attributes
    ///
    /// # Returns
    /// A `Result` containing a `MintResponse` detailing the transaction signature, asset ID, and whether the cNFT was minted successfully
    pub async fn mint_compressed_nft(&self, request: MintCompressedNftRequest) -> Result<MintResponse> {
        self.rpc_client.post_rpc_request("mintCompressedNft", request).await
    }
}
