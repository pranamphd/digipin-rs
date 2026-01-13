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

use crate::constants::{DIGIPIN_LABEL_GRID, DIGIPIN_MAX_DECODE_ERROR_DEGREES, DIGIPIN_SYMBOLS};
use crate::decode::decode;
use crate::encode::encode;
use crate::location::Location;

/// Tests the roundtrip encoding and decoding of a DIGIPIN.
#[test]
fn encode_decode_roundtrip() {
    let location = Location {
        latitude: 28.622788,
        longitude: 77.213033,
    };

    let digipin = encode(location).unwrap();
    let decoded_location = decode(&digipin).unwrap();

    assert!(
        (decoded_location.latitude - location.latitude).abs() <= DIGIPIN_MAX_DECODE_ERROR_DEGREES
    );
    assert!(
        (decoded_location.longitude - location.longitude).abs() <= DIGIPIN_MAX_DECODE_ERROR_DEGREES
    );
}

/// Tests that decoding accepts lowercase letters and separators.
#[test]
fn decode_accpets_lowercase_and_seperators() {
    let formatted = "2c3-4k5-pft9";
    assert!(decode(formatted).is_ok());
}

/// Tests that encoding rejects locations outside the defined territory.
#[test]
fn encode_rejects_outside_territory() {
    let location = Location {
        latitude: 0.0,
        longitude: 0.0,
    };
    assert!(encode(location).is_err());
}

/// Tests encoding at the boundary of the supported territory (south-west corner).
#[test]
fn boundary_south_west() {
    let location = Location {
        latitude: 2.5,
        longitude: 63.5,
    };
    assert!(encode(location).is_ok());
}

/// Tests encoding at the boundary of the supported territory (north-east corner).
#[test]
fn boundary_north_east() {
    let location = Location {
        latitude: 38.5,
        longitude: 99.5,
    };
    assert!(encode(location).is_ok())
}

/// Tests that encoding rejects latitude below minimum supported value.
#[test]
fn encode_rejects_latitude_below_min() {
    let location = Location {
        latitude: 2.48,
        longitude: 70.0,
    };
    assert!(encode(location).is_err());
}

/// Tests that encoding rejects latitude above maximum supported value.
#[test]
fn encode_rejects_longitude_above_max() {
    let location = Location {
        latitude: 20.0,
        longitude: 99.51,
    };
    assert!(encode(location).is_err());
}

/// Tests that decoding accepts formatted DIGIPIN strings with separators.
#[test]
fn decode_accepts_formatted_digipin() {
    assert!(decode("2c3-4km-pft9").is_ok());
}

/// Tests that decoding rejects DIGIPIN strings of invalid length or invalid symbols.
#[test]
fn decode_rejects_invalid_symbol() {
    assert!(decode("2C3J4K5MZFT9").is_err());
    assert!(decode("A4D-SK2-S7Z5").is_err());
}

/// Tests that the DIGIPIN symbols match the label grid.
#[test]
fn symbols_match_label_grid() {
    let flattened: Vec<char> = DIGIPIN_LABEL_GRID.iter().flatten().copied().collect();
    assert_eq!(flattened, DIGIPIN_SYMBOLS)
}
