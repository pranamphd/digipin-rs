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

/// Normalizes DIGIPIN.
///
/// # Rules
/// - Removes '-' separators
/// - Converts to uppercase
/// # Arguments
/// - `digipin`: The DIGIPIN string to normalize.
/// # Returns
/// - A normalized DIGIPIN string.
pub fn normalize(digipin: &str) -> String {
    digipin
        .chars()
        .filter(|c| *c != '-')
        .map(|c| c.to_ascii_uppercase())
        .collect()
}
