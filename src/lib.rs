use thiserror::Error;
pub mod cin;
pub mod gstin;
pub mod pan;
pub mod udyam;
pub mod utils;

pub use cin::CinMeta;
pub use gstin::GstinMeta;
pub use pan::PanMeta;
pub use udyam::UdyamMeta;

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