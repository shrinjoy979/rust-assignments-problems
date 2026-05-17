/*
  Problem 55: Custom Error Enum (thiserror-style)

  Define an error enum DataError with variants InvalidLength { expected, actual },
  ChecksumMismatch, and Utf8Error(std::string::FromUtf8Error).
  Implement std::fmt::Display and std::error::Error manually.
  Write a function validate_packet that checks a &[u8] has exactly 10 bytes
  with a valid checksum (last byte = XOR of all previous bytes).

  Run the tests for this problem with:
    cargo test --test custom_error_enum_test
*/

use std::fmt;

#[derive(Debug)]
pub enum DataError {
    InvalidLength { expected: usize, actual: usize },
    ChecksumMismatch,
    Utf8Error(std::string::FromUtf8Error),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::InvalidLength { expected, actual } => {
                write!(
                    f,
                    "Invalid length: expected {}, got {}",
                    expected, actual
                )
            }
            DataError::ChecksumMismatch => {
                write!(f, "Checksum mismatch")
            }
            DataError::Utf8Error(err) => {
                write!(f, "UTF-8 error: {}", err)
            }
        }
    }
}

impl std::error::Error for DataError {}

pub fn validate_packet(data: &[u8]) -> Result<(), DataError> {
    // Packet must contain exactly 10 bytes
    if data.len() != 10 {
        return Err(DataError::InvalidLength {
            expected: 10,
            actual: data.len(),
        });
    }

    // Compute XOR checksum of first 9 bytes
    let checksum = data[..9]
        .iter()
        .fold(0u8, |acc, &byte| acc ^ byte);

    // Last byte must match checksum
    if checksum != data[9] {
        return Err(DataError::ChecksumMismatch);
    }

    Ok(())
}