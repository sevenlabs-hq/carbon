use crate::error::{HeliusError, Result};
use reqwest::{Client, Method, RequestBuilder, Response, StatusCode, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use std::sync::Arc;

/// Manages HTTP requests for the `Helius` client
///
/// This struct is responsible for sending HTTP requests and handling responses. It encapsulates details
/// of the `reqwest::Client` to provide a simplified interface for making requests and processing responses
#[derive(Clone)]
pub struct RequestHandler {
    pub http_client: Arc<Client>,
}

impl RequestHandler {
    /// Creates a new instance of `RequestHandler`
    ///
    /// # Arguments
    /// * `client` - A shared instance of a `reqwest::Client`
    pub fn new(client: Arc<Client>) -> Result<Self> {
        Ok(Self { http_client: client })
    }

    /// Asynchronously sends a specified HTTP request using the `RequestBuilder`
    ///
    /// # Arguments
    /// * `request_builder` - Configured `RequestBuilder` for a specific HTTP request
    ///
    /// # Returns
    /// A `Result` wrapping a `reqwest::Response` if the request is send and received successfully
    ///
    /// # Errors
    /// Returns `HeliusError::Network` if there is an issue sending the request
    async fn send_request(&self, request_builder: RequestBuilder) -> Result<Response> {
        request_builder.send().await.map_err(HeliusError::Network)
    }

    /// Sends an HTTP request and processes the response to deserialize into a specified generic type
    ///
    /// # Type Parameters
    /// * `R` - The type of the request body, which must implement `Serialize`
    /// * `T` - The expected type of the response, which must implement `Deserialize`
    ///
    /// # Arguments
    /// * `method` - The HTTP method to be used for the request
    /// * `url` - The URL to which the request is sent
    /// * `body` - An optional request body, serialized as JSON if provided
    ///
    /// # Returns
    /// A `Result` wrapping the deserialized response data if the response is successful
    ///
    /// # Errors
    /// Returns an error if the request fails at any stage, including network errors, serialization errors
    /// or if the response status is not successful
    pub async fn send<R, T>(&self, method: Method, url: Url, body: Option<&R>) -> Result<T>
    where
        R: Serialize + ?Sized + Send + Sync + Debug,
        T: for<'de> Deserialize<'de> + Default,
    {
        let mut request_builder: RequestBuilder = self.http_client.request(method, url);

        if let Some(body) = body {
            request_builder = request_builder.json(body);
        }

        let response: Response = self.send_request(request_builder).await?;
        self.handle_response(response).await
    }

    /// Handles the Response for a given HTTP request, attempting to deserialize the response body into the requested type
    ///
    /// # Type Parameters
    /// * `T` - The type to which the response body should be deserialized
    ///
    /// # Arguments
    /// * `response` - The `reqwest::Response` received from the HTTP request
    ///
    /// # Returns
    /// A `Result` wrapping the deserialized data if the response is parsed successfully
    ///
    /// # Errors
    /// Returns an error if deserialization fails or if the response status indicates a failure (e.g., 404 Not Found)
    async fn handle_response<T: for<'de> Deserialize<'de> + Default>(&self, response: Response) -> Result<T> {
        let status: StatusCode = response.status();
        let path: String = response.url().path().to_string();
        let body_text: String = response.text().await.unwrap_or_default();

        if status.is_success() {
            if body_text.is_empty() {
                return Ok(T::default());
            }

            match serde_json::from_str::<T>(&body_text) {
                Ok(data) => Ok(data),
                Err(e) => {
                    println!("Deserialization error: {}", e);
                    println!("Raw JSON: {}", body_text);
                    Err(HeliusError::from(e))
                }
            }
        } else {
            let body_json: serde_json::Result<Value> = serde_json::from_str(&body_text);
            match body_json {
                Ok(body) => {
                    let error_message = match body["error"].clone() {
                        Value::Object(error_value) => error_value
                            .into_iter()
                            .map(|(k, v)| format!("{}: {}", k, v))
                            .collect::<Vec<String>>()
                            .join(", ")
                            .to_string(),
                        Value::String(error_value) => error_value,
                        _ => "Unknown error".to_string(),
                    };
                    Err(HeliusError::from_response_status(status, path, error_message))
                }
                Err(_) => Err(HeliusError::from_response_status(status, path, body_text)),
            }
        }
    }
}
