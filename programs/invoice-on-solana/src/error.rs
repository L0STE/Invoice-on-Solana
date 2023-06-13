use anchor_lang::error_code;

#[error_code]
pub enum InvoiceError {
    #[msg("This Invoice didn't expire, please Wait")]
    ExpirationError,
}