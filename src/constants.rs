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

/// Geodetic coordinate limits (WGS-84 compatible)
pub const GEODETIC_LATITUDE_MIN: f64 = -90.0;
pub const GEODETIC_LATITUDE_MAX: f64 = 90.0;
pub const GEODETIC_LONGITUDE_MIN: f64 = -180.0;
pub const GEODETIC_LONGITUDE_MAX: f64 = 180.0;

/// Official DIGIPIN supported territory bounds
pub const DIGIPIN_LATITUDE_MIN: f64 = 2.5;
pub const DIGIPIN_LATITUDE_MAX: f64 = 38.5;
pub const DIGIPIN_LONGITUDE_MIN: f64 = 63.5;
pub const DIGIPIN_LONGITUDE_MAX: f64 = 99.5;

/// Official DIGIPIN symbols (base-16)
pub const DIGIPIN_SYMBOLS: [char; 16] = [
    'F', 'C', '9', '8', 'J', '3', '2', '7', 'K', '4', '5', '6', 'L', 'M', 'P', 'T',
];
pub const DIGIPIN_LENGTH: usize = 10;
pub const DIGIPIN_GRID_SIZE: usize = 4;
pub const DIGIPIN_LABEL_GRID: [[char; DIGIPIN_GRID_SIZE]; DIGIPIN_GRID_SIZE] = [
    ['F', 'C', '9', '8'],
    ['J', '3', '2', '7'],
    ['K', '4', '5', '6'],
    ['L', 'M', 'P', 'T'],
];

/// Maximum possible error after DIGIPIN decode in degrees (derived from specification:  36° / 4¹⁰ / 2)
#[cfg(test)]
pub const DIGIPIN_MAX_DECODE_ERROR_DEGREES: f64 = 0.0000171661376953125;
