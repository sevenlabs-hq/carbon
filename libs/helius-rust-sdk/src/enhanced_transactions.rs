use crate::error::Result;
use crate::types::{EnhancedTransaction, ParseTransactionsRequest, ParsedTransactionHistoryRequest};
use crate::Helius;

use reqwest::{Method, Url};

impl Helius {
    /// Parses transactions given an array of transaction IDs
    ///
    /// # Arguments
    /// * `ParseTransactionsRequest` - A parse transaction request, which includes:
    /// - A vector of transaction IDs to be parsed
    ///
    /// # Returns
    /// A `Result` wrapping a vector of `EnhancedTransaction`s
    pub async fn parse_transactions(&self, request: ParseTransactionsRequest) -> Result<Vec<EnhancedTransaction>> {
        let url: String = format!(
            "{}v0/transactions?api-key={}",
            self.config.endpoints.api, self.config.api_key
        );
        let parsed_url: Url = Url::parse(&url).expect("Failed to parse URL");

        self.rpc_client
            .handler
            .send(Method::POST, parsed_url, Some(&request))
            .await
    }

    /// Retrieves a parsed transaction history for a specific address
    ///
    /// # Arguments
    /// * `ParsedTransactionHistoryRequest` - A parsed transaction history request, which includes:
    /// - An address for which a given parsed transaction history will be retrieved
    /// - An optional `before` parameter that, when provided, fetches the parsed transaction history before the given signature. This is useful for pagination
    ///
    /// # Returns
    /// A `Result` wrapping a vector of `EnhancedTransaction`s
    pub async fn parsed_transaction_history(
        &self,
        request: ParsedTransactionHistoryRequest,
    ) -> Result<Vec<EnhancedTransaction>> {
        let mut url: String = format!(
            "{}v0/addresses/{}/transactions?api-key={}",
            self.config.endpoints.api, request.address, self.config.api_key
        );

        if let Some(before) = request.before {
            url = format!("{}&before={}", url, before);
        }

        let parsed_url: Url = Url::parse(&url).expect("Failed to parse URL");

        self.rpc_client.handler.send(Method::GET, parsed_url, None::<&()>).await
    }
}
