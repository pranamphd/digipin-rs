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

use crate::errors::DigipinError;
use crate::{constants::*, location::Location};

/// Validates geodetic latitude and longitude.
/// # Arguments
/// - `location`: A Location struct containing latitude and longitude.
/// # Returns
/// - `Ok(())`: If the coordinates are valid.
/// - `Err(DigipinError)`: If the latitude or longitude is out of bounds.
pub fn validate_geodetic_coordinates(location: Location) -> Result<(), DigipinError> {
    if !(GEODETIC_LATITUDE_MIN..=GEODETIC_LATITUDE_MAX).contains(&location.latitude) {
        return Err(DigipinError::InvalidLatitude);
    }

    if !(GEODETIC_LONGITUDE_MIN..=GEODETIC_LONGITUDE_MAX).contains(&location.longitude) {
        return Err(DigipinError::InvalidLongitude);
    }

    Ok(())
}

/// Validates coordinates against the DIGIPIN supported territory.
/// # Arguments
/// - `location`: A Location struct containing latitude and longitude.
/// # Returns
/// - `Ok(())`: If the coordinates are within the DIGIPIN supported territory.
/// - `Err(DigipinError)`: If the coordinates are outside the supported territory.
pub fn validate_digipin_territory(location: Location) -> Result<(), DigipinError> {
    if !(DIGIPIN_LATITUDE_MIN..=DIGIPIN_LATITUDE_MAX).contains(&location.latitude)
        || !(DIGIPIN_LONGITUDE_MIN..=DIGIPIN_LONGITUDE_MAX).contains(&location.longitude)
    {
        return Err(DigipinError::OutsideSupportedTerritory);
    }

    Ok(())
}

/// Validates the DIGIPIN string format.
/// # Arguments
/// - `digipin`: A string slice representing the DIGIPIN to validate.
/// # Returns
/// - `Ok(())`: If the DIGIPIN format is valid.
/// - `Err(DigipinError)`: If the DIGIPIN format is invalid
pub fn validate_digipin_format(digipin: &str) -> Result<(), DigipinError> {
    if digipin.len() != DIGIPIN_LENGTH {
        return Err(DigipinError::InvalidDigipinLength);
    }

    if !digipin.chars().all(|c| DIGIPIN_SYMBOLS.contains(&c)) {
        return Err(DigipinError::InvalidDigipinFormat);
    }

    Ok(())
}
