use std::fmt::{Display, Formatter, Result as FmtResult};

use log::SetLoggerError;

pub type RagResult<T> = Result<T, RagError>;

#[derive(Debug)]
pub enum RagError {
	SetLogger(SetLoggerError),
	Empty,
}

impl From<()> for RagError {
	fn from(_: ()) -> RagError {
		RagError::Empty
	}
}

impl From<SetLoggerError> for RagError {
	fn from(e: SetLoggerError) -> RagError {
		RagError::SetLogger(e)
	}
}

impl Display for RagError {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match *self {
			RagError::SetLogger(ref e) => Display::fmt(e, f),
			RagError::Empty => Display::fmt("Empty", f),
		}
	}
}
