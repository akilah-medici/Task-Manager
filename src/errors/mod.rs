#[derive(Debug)]
#[allow(dead_code)]
pub enum Errors{
    FileNotFound(String),
    FileError(String),
}