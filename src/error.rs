#[derive(Debug)]
pub enum Error {
  ReadFile,
  WriteFile,
  ParseJson,
  ReadDir,

  InvalidPassword
}

pub type Result<T> = core::result::Result<T, Error>;