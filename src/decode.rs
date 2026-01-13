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

use crate::constants::*;
use crate::errors::DigipinError;
use crate::location::Location;
use crate::normalize::normalize;
use crate::validation::validate_digipin_format;

/// Decodes a DIGIPIN into geodetic coordinates.
///
/// # Arguments
/// - `digipin`: A string slice representing the DIGIPIN to decode.
///
/// # Returns
/// - `Ok(Location)`: A struct containing the decoded latitude and longitude.
/// - `Err(DigipinError)`: An error if the DIGIPIN format is invalid.
pub fn decode(digipin: &str) -> Result<Location, DigipinError> {
    let normalized = normalize(digipin);
    validate_digipin_format(&normalized)?;

    let mut latitude_min = DIGIPIN_LATITUDE_MIN;
    let mut latitude_max = DIGIPIN_LATITUDE_MAX;
    let mut longitude_min = DIGIPIN_LONGITUDE_MIN;
    let mut longitude_max = DIGIPIN_LONGITUDE_MAX;
    for symbol in normalized.chars() {
        let latitude_step = (latitude_max - latitude_min) / DIGIPIN_GRID_SIZE as f64;
        let longitude_step = (longitude_max - longitude_min) / DIGIPIN_GRID_SIZE as f64;

        let mut found = false;

        for (row, row_cells) in DIGIPIN_LABEL_GRID.iter().enumerate() {
            for (column, cell) in row_cells.iter().enumerate() {
                if *cell == symbol {
                    latitude_max -= row as f64 * latitude_step;
                    latitude_min = latitude_max - latitude_step;

                    longitude_min += column as f64 * longitude_step;
                    longitude_max = longitude_min + longitude_step;

                    found = true;
                    break;
                }
            }

            if found {
                break;
            }
        }

        if !found {
            return Err(DigipinError::InvalidDigipinFormat);
        }
    }

    Ok(Location {
        latitude: (latitude_min + latitude_max) / 2.0,
        longitude: (longitude_min + longitude_max) / 2.0,
    })
}
