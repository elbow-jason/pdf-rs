use std::io;

use crate::objects::{Object, ObjectType};

#[derive(Debug)]
pub enum ParseError {
    MismatchedByte {
        expected: u8,
        found: Option<u8>,
    },
    MismatchedByteMany {
        expected: &'static [u8],
        found: Option<u8>,
    },
    UnexpectedEof,
    IoError(io::Error),
    MismatchedObjectType {
        expected: ObjectType,
        found: Object,
    },
    MissingRequiredKey {
        key: &'static str,
    },
    ArrayOfInvalidLength {
        expected: usize,
        found: Vec<Object>,
    },
    UnrecognizedVariant {
        found: String,
        ty: &'static str,
    },
    Todo,
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

pub type PdfResult<T> = Result<T, ParseError>;