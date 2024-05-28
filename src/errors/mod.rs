#[derive(Debug)]
pub struct ConversionError;

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conversion failed")
    }
}
impl std::error::Error for ConversionError {}
