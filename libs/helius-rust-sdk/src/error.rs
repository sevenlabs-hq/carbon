use reqwest::{Error as ReqwestError, StatusCode};
use serde_json::Error as SerdeJsonError;
use solana_client::client_error::ClientError;
use solana_sdk::{
    message::CompileError, sanitize::SanitizeError, signature::SignerError, transaction::TransactionError,
};
use thiserror::Error;

/// Represents all possible errors returned by the `Helius` client
///
/// This enum provides a detailed categorization of various error scenarios, ranging from network failures to data serialization issues
#[derive(Debug, Error)]
pub enum HeliusError {
    /// Indicates an improperly formatted request
    ///
    /// This error occurs when the request parameters do not meet the expected format, are missing required fields,
    /// or contains invalid data
    #[error("Bad request to {path}: {text}")]
    BadRequest { path: String, text: String },

    /// Represents errors from the Solana client
    ///
    /// This captures errors from the Solana client library
    #[error("Solana client error: {0}")]
    ClientError(#[from] ClientError),

    /// Indicates if a client is not already initialized
    ///
    /// Useful for the new_with_async_solana method on the `Helius` client
    #[error("Client not initialized: {text}")]
    ClientNotInitialized { text: String },

    /// Represents compile errors from the Solana SDK
    ///
    /// This captures all compile errors thrown by the Solana SDK
    #[error("Compile error: {0}")]
    CompileError(#[from] CompileError),

    /// Represents errors that occur internally with Helius and our servers
    ///
    /// If the server encounters an unexpected condition that prevents it from fulfilling the request, this error is returned.
    /// It includes the HTTP status code and a more detailed message about what went wrong.
    /// If you are seeing this error continually, please reach out to Helius support.
    #[error("Internal server error: {code} - {text}")]
    InternalError { code: StatusCode, text: String },

    /// Indicates that a required input was either missing or invalid.
    ///
    /// This error is used for scenarios where user input does not conform to expected values, such as an empty string for an API key
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Covers general network failures
    ///
    /// This could range from DNS resolution failures, lost connections, issues with Solana, or any issue that prevents the client from reaching the server
    #[error("Network error: {0}")]
    Network(ReqwestError),

    /// Indicates the requested resource was not found
    ///
    /// This error can occur if a specified identifier does not match any existing entities known to the server
    #[error("Not found: {text}")]
    NotFound { text: String },

    /// Indicates too many requests are sent in a given amount of time
    ///
    /// This error includes the path to help identify a throttled request. Please visit https://docs.helius.dev/welcome/pricing-and-rate-limits to see all the
    /// current rate limits for each standard plan
    #[error("Too many requests made to {path}")]
    RateLimitExceeded { path: String },

    /// Indicates an error from the underlying HTTP client (i.e., reqwest)
    ///
    /// This captures errors from the `reqwest` library specifically
    #[error("Request error: {0}")]
    ReqwestError(ReqwestError),

    /// Occurs during the serialization or deserialization of JSON data
    ///
    /// If the JSON data cannot be encoded or decoded, this error will be thrown, typically indicating an issue with the data structure
    ///
    #[error("Serialization / Deserialization error: {0}")]
    SerdeJson(SerdeJsonError),

    /// Represents errors from the Solana SDK for signing operations
    ///
    /// This captures errors from the signing operations in the Solana SDK
    #[error("Signer error: {0}")]
    SignerError(#[from] SignerError),

    /// Indicates that the transaction confirmation timed out
    ///
    /// For polling a transaction's confirmation status
    #[error("Transaction confirmation timed out with error code {code}: {text}")]
    Timeout { code: StatusCode, text: String },

    /// Represents transaction errors from the Solana SDK
    ///
    /// This captures errors that occur when processing transactions
    #[error("Transaction error: {0}")]
    TransactionError(#[from] TransactionError),

    /// Indicates the request lacked valid authentication credentials
    ///
    /// This error is returned in response to a missing, invalid, or expired API key
    #[error("Unauthorized access to {path}: {text}")]
    Unauthorized { path: String, text: String },

    /// A fallback error used when an unexpected or uncategorized issue occurs
    ///
    /// This error includes the HTTP status code and message to help with debugging, acting as a generic catch-all for all other errors
    #[error("Unknown error has occurred: HTTP {code} - {text}")]
    Unknown { code: StatusCode, text: String },

    #[error("Unable to connect to server: {0}")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Websocket connection closed (({0})")]
    WebsocketClosed(String),

    #[error("Enhanced websocket: {message}: {reason}")]
    EnhancedWebsocket { reason: String, message: String },

    #[error("Url parse error")]
    UrlParseError(#[from] url::ParseError),

    #[error("TLS error: {0}")]
    TlsError(String),
}

impl HeliusError {
    /// Converts a `StatusCode` and message into the appropriate `HeliusError`
    ///
    /// This utility function helps map HTTP status codes to the more specific errors detailed in this enum
    pub fn from_response_status(status: StatusCode, path: String, text: String) -> Self {
        match status {
            StatusCode::BAD_REQUEST => HeliusError::BadRequest { path, text },
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => HeliusError::Unauthorized { path, text },
            StatusCode::NOT_FOUND => HeliusError::NotFound { text },
            StatusCode::INTERNAL_SERVER_ERROR => HeliusError::InternalError { code: status, text },
            StatusCode::TOO_MANY_REQUESTS => HeliusError::RateLimitExceeded { path },
            _ => HeliusError::Unknown { code: status, text },
        }
    }
}

impl From<SerdeJsonError> for HeliusError {
    /// Converts a `SerdeJsonError` into a `HeliusError`
    ///
    /// This allows for the seamless integration of JSON parsing errors into the broader error handling system
    fn from(err: SerdeJsonError) -> HeliusError {
        HeliusError::SerdeJson(err)
    }
}

impl From<SanitizeError> for HeliusError {
    fn from(err: SanitizeError) -> Self {
        HeliusError::InvalidInput(err.to_string())
    }
}

impl From<ReqwestError> for HeliusError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_builder() {
            HeliusError::TlsError(err.to_string())
        } else {
            HeliusError::ReqwestError(err)
        }
    }
}

/// A handy type alias for handling results across the Helius SDK
pub type Result<T> = std::result::Result<T, HeliusError>;
