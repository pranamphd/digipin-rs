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

/// Represents a geographic location with latitude and longitude.
/// # Fields
/// - `latitude`: The latitude in degrees.
/// - `longitude`: The longitude in degrees.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}
