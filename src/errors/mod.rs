
#[derive(Debug)]
pub enum Errors{
    FileNotFound(String),
    FileError(String),
    NotFound,
}