use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Client error: status code: {status_code}, error code: {error_code:?}, error message: {error_message}, error data: {error_data:?}")]
    ClientRequest {
        status_code: u16,
        error_code: Option<u16>,
        error_message: String,
        error_data: Option<String>,
    },
    #[error("Server error: status code: {status_code}, error message: {error_message}")]
    ServerRequest {
        status_code: u16,
        error_message: String,
    },
    #[error("Generic request error: {0:?}")]
    GenericRequest(String),
    #[error("Chain type not allowed for this function")]
    ChainNotAllowed,
    #[error("Error from Eip712 struct: {0:?}")]
    Eip712(String),
    #[error("Json parse error: {0:?}")]
    JsonParse(String),
    #[error("Generic parse error: {0:?}")]
    GenericParse(String),
    #[error("Wallet error: {0:?}")]
    Wallet(String),
    #[error("Private key parse error: {0:?}")]
    PrivateKeyParse(String),
    #[error("Rmp parse error: {0:?}")]
    RmpParse(String),
    #[error("ECDSA signature failed: {0:?}")]
    SignatureFailure(String),
    #[error("Vault address not found")]
    VaultAddressNotFound,
}
