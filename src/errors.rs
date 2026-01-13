// Copyright (c) 2026 Pranam
// ORCID: https://orcid.org/0009-0007-9316-3616
//
// This code is licensed under the Apache License, Version 2.0.
//
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// QR Code:
// ![QR Code](/ORCID.png)
// Scan the QR code to access my ORCID profile.

use std::fmt::{self};

/// Errors returned by DIGIPIN operations.
/// # Variants
/// - `InvalidLatitude`: Latitude is outside geodetic bounds.
/// - `InvalidLongitude`: Longitude is outside geodetic bounds.
/// - `OutsideSupportedTerritory`: Location is outside the DIGIPIN supported territory.
/// - `InvalidDigipinLength`: DIGIPIN does not conform to required length.
/// - `InvalidDigipinFormat`: DIGIPIN contains invalid symbols or format.
#[derive(Debug, Clone, PartialEq)]
pub enum DigipinError {
    InvalidLatitude,
    InvalidLongitude,
    OutsideSupportedTerritory,
    InvalidDigipinLength,
    InvalidDigipinFormat,
}

/// Implements Display trait for DigipinError to provide human-readable error messages.
/// # Arguments
/// - `f`: A mutable reference to a Formatter.
/// # Returns
/// - `fmt::Result`: The result of the formatting operation.
impl fmt::Display for DigipinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLatitude => {
                write!(f, "Latitude is outside geodetic bounds.")
            }
            Self::InvalidLongitude => {
                write!(f, "Longitude is outside geodetic bounds.")
            }
            Self::OutsideSupportedTerritory => {
                write!(f, "Location is outside the DIGIPIN supported territory.")
            }
            Self::InvalidDigipinLength => {
                write!(f, "DIGIPIN does not conform to required length.")
            }
            Self::InvalidDigipinFormat => {
                write!(f, "DIGIPIN contains invalid symbols or format.")
            }
        }
    }
}
/// Implements the standard Error trait for DigipinError.
/// # Note
/// This allows DigipinError to be used as a standard error type in Rust.
impl std::error::Error for DigipinError {}
