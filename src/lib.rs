use thiserror::Error;
pub mod cin;
mod gstin;
mod pan;
mod udyam;
mod utils;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Error parsing CIN: {0}")]
    Cin(String),
    #[error("Error parsing PAN: {0}")]
    Pan(String),
    #[error("Error parsing GSTIN: {0}")]
    Gstin(String),
    #[error("Error parsing Udyam: {0}")]
    Udyam(String),
}