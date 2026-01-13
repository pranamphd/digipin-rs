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
use crate::validation::{validate_digipin_territory, validate_geodetic_coordinates};

/// Encodes coordinates into a canonical DIGIPIN.
/// # Arguments
/// - `location`: A struct containing the latitude and longitude to encode.
/// # Returns
/// - `Ok(String)`: A string representing the encoded DIGIPIN.
/// - `Err(DigipinError)`: An error if the coordinates are invalid or outside the supported territory.
pub fn encode(location: Location) -> Result<String, DigipinError> {
    validate_geodetic_coordinates(location)?;
    validate_digipin_territory(location)?;

    let mut latitude_min = DIGIPIN_LATITUDE_MIN;
    let mut latitude_max = DIGIPIN_LATITUDE_MAX;
    let mut longitude_min = DIGIPIN_LONGITUDE_MIN;
    let mut longitude_max = DIGIPIN_LONGITUDE_MAX;

    let mut digipin = String::with_capacity(DIGIPIN_LENGTH);

    for _ in 0..DIGIPIN_LENGTH {
        let latitude_step = (latitude_max - latitude_min) / DIGIPIN_GRID_SIZE as f64;
        let longitude_step = (longitude_max - longitude_min) / DIGIPIN_GRID_SIZE as f64;

        let mut row = ((latitude_max - location.latitude) / latitude_step).floor() as usize;
        let mut column = ((location.longitude - longitude_min) / longitude_step).floor() as usize;

        // Clamp row and column to valid grid bounds
        row = row.min(DIGIPIN_GRID_SIZE - 1);
        column = column.min(DIGIPIN_GRID_SIZE - 1);

        let symbol = DIGIPIN_LABEL_GRID[row][column];
        digipin.push(symbol);

        latitude_max -= row as f64 * latitude_step;
        latitude_min = latitude_max - latitude_step;

        longitude_min += column as f64 * longitude_step;
        longitude_max = longitude_min + longitude_step;
    }

    Ok(digipin)
}
