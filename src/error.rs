#[derive(Debug)]
pub enum Error {
    ValueOutOfRange(f32, f32),
    TableParsingError(String),
}
