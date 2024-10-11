use crate::error::Result;
use crate::types::{CreateWebhookRequest, EditWebhookRequest, Webhook};
use crate::Helius;

use reqwest::{Method, Url};

impl Helius {
    /// Creates a webhook given account addresses
    ///
    /// # Arguments
    /// * `request` - A `CreateWebhookRequest` containing the relevant webhook parameters for creation such as the webhook url and type
    ///
    /// # Returns
    /// A `Result` wrapping a `Webhook` if the webhook is successfully created, or a `HeliusError` if creation fails
    pub async fn create_webhook(&self, request: CreateWebhookRequest) -> Result<Webhook> {
        let url: String = format!(
            "{}v0/webhooks?api-key={}",
            self.config.endpoints.api, self.config.api_key
        );
        let parsed_url: Url = Url::parse(&url).expect("Failed to parse URL");

        self.rpc_client
            .handler
            .send(Method::POST, parsed_url, Some(&request))
            .await
    }

    /// Edits a Helius webhook programmatically
    ///
    /// # Arguments
    /// * `request` - An `EditWebhookRequest` containing the parameters to be updated for a given webhook
    ///
    /// # Returns
    /// A `Result` wrapping the updated `Webhook`, or a `HeliusError` if the edit request fails
    pub async fn edit_webhook(&self, request: EditWebhookRequest) -> Result<Webhook> {
        let url: String = format!(
            "{}v0/webhooks/{}?api-key={}",
            self.config.endpoints.api, request.webhook_id, self.config.api_key
        );
        let parsed_url: Url = Url::parse(&url).expect("Failed to parse URL");

        self.rpc_client
            .handler
            .send(Method::PUT, parsed_url, Some(&request))
            .await
    }

    /// Appends a set of addresses to a given webhook
    ///
    /// # Arguments
    /// * `webhook_id` - The ID of the webhook to be updated
    /// * `new_addresses` - A slice of strings representing the new account addresses to be added to the given webhook
    ///
    /// # Returns
    /// A `Result` containing the updated `Webhook` object
    pub async fn append_addresses_to_webhook(&self, webhook_id: &str, new_addresses: &[String]) -> Result<Webhook> {
        let mut webhook: Webhook = self.get_webhook_by_id(webhook_id).await?;
        webhook.account_addresses.extend(new_addresses.to_vec());

        let edit_request: EditWebhookRequest = EditWebhookRequest {
            webhook_id: webhook_id.to_string(),
            webhook_url: webhook.webhook_url,
            transaction_types: webhook.transaction_types,
            auth_header: webhook.auth_header,
            txn_status: webhook.txn_status,
            encoding: webhook.encoding,
            account_addresses: webhook.account_addresses,
            webhook_type: webhook.webhook_type,
        };
        self.edit_webhook(edit_request).await
    }

    /// Removes a list of addresses from an existing webhook by its ID
    ///
    /// # Arguments
    /// * `webhook_id` - The ID of the webhook to edit
    /// * `addresses_to_remove` - The vector of addresses to be removed from the webhook provided
    ///
    /// # Returns
    /// A `Result` wrapping the edited `Webhook`, if successful
    pub async fn remove_addresses_from_webhook(
        &self,
        webhook_id: &str,
        addresses_to_remove: &[String],
    ) -> Result<Webhook> {
        let mut webhook: Webhook = self.get_webhook_by_id(webhook_id).await?;
        webhook
            .account_addresses
            .retain(|address| !addresses_to_remove.contains(address));

        let edit_request: EditWebhookRequest = EditWebhookRequest {
            webhook_id: webhook_id.to_string(),
            webhook_url: webhook.webhook_url,
            transaction_types: webhook.transaction_types,
            auth_header: webhook.auth_header,
            txn_status: webhook.txn_status,
            encoding: webhook.encoding,
            account_addresses: webhook.account_addresses,
            webhook_type: webhook.webhook_type,
        };

        self.edit_webhook(edit_request).await
    }

    /// Gets a webhook config given a webhook ID
    ///
    /// # Arguments
    /// * `webhook_id` - The ID of the webhook to be updated
    ///
    /// # Returns
    /// A `Result` wrapping the `Webhook` queried, if it exists
    pub async fn get_webhook_by_id(&self, webhook_id: &str) -> Result<Webhook> {
        let url: String = format!(
            "{}v0/webhooks/{}?api-key={}",
            self.config.endpoints.api, webhook_id, self.config.api_key
        );
        let parsed_url: Url = Url::parse(&url).expect("Failed to parse URL");

        self.rpc_client.handler.send(Method::GET, parsed_url, None::<&()>).await
    }

    /// Retrieves all Helius webhooks programmatically
    ///
    /// Due to response size limitations, we will truncate addresses returned to 100 per config
    ///
    /// # Returns
    /// A `Result` containing a vector of `Webhook` representing all configured webhooks for a given account
    pub async fn get_all_webhooks(&self) -> Result<Vec<Webhook>> {
        let url: String = format!(
            "{}v0/webhooks?api-key={}",
            self.config.endpoints.api, self.config.api_key
        );
        let parsed_url: Url = Url::parse(&url).expect("Failed to parse URL");

        self.rpc_client.handler.send(Method::GET, parsed_url, None::<&()>).await
    }

    /// Deletes a given Helius webhook programmatically
    ///
    /// # Arguments
    /// * `webhook_id` - The ID of the webhook to be deleted
    ///
    /// # Returns
    /// A unit since there isn't any response
    pub async fn delete_webhook(&self, webhook_id: &str) -> Result<()> {
        let url: String = format!(
            "{}v0/webhooks/{}?api-key={}",
            self.config.endpoints.api, webhook_id, self.config.api_key
        );
        let parsed_url: Url = Url::parse(&url).expect("Failed to parse URL");

        self.rpc_client
            .handler
            .send(Method::DELETE, parsed_url, None::<&()>)
            .await
    }
}
