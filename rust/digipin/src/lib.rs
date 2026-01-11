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

/// DIGIPIN – Digital Postal Index Number
///
/// This library provides functionality to encode geographic locations
/// (latitude and longitude) into a canonical 10-character DIGIPIN string
/// and decode DIGIPIN strings back into geographic locations.
///
/// Public API:
/// - `encode`: Encode geographic coordinates into DIGIPIN strings.
/// - `decode`: Decode DIGIPIN strings into geographic coordinates.
/// - `errors`: Error types for DIGIPIN operations.
/// # Usage
/// To use this library, include it as a dependency in your Rust project and call the
/// `encode` and `decode` functions as needed.
/// ```rust
/// use digipin::{encode, decode, Location};
/// let location = Location { latitude: 12.3456, longitude: 78.9012 };
/// let digipin = encode(location).unwrap();
/// let decoded_location = decode(&digipin).unwrap();
/// ```
mod constants;
mod decode;
mod encode;
mod errors;
mod location;
mod normalize;
mod validation;

pub use decode::decode;
pub use encode::encode;
pub use errors::DigipinError;
pub use location::Location;

#[cfg(test)]
mod tests;
