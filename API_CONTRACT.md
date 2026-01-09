# DIGIPIN Library API Contract

This document defines the **canonical public interface** that all DIGIPIN language libraries must implement.
The goal is **consistency, predictability, and cross-language parity**.

---

## 1. Core concepts

* DIGIPIN is a **pure, deterministic encoding**
* No I/O, no networking, no external dependencies
* Same input → same output in every language
* No hidden state

---

## 2. Public functions (mandatory)

### 2.1 Encode: latitude/longitude → DIGIPIN

**Purpose**
Convert geographic coordinates to a 10-character DIGIPIN code.

**Signature (conceptual)**

```
encode(latitude, longitude) → digipin
```

**Parameters**

* `latitude`

  * Type: floating point number
  * Unit: decimal degrees
  * Range: 2.5 ≤ latitude ≤ 38.5
* `longitude`

  * Type: floating point number
  * Unit: decimal degrees
  * Range: 63.5 ≤ longitude ≤ 99.5

**Return value**

* `digipin`

  * Type: string
  * Format: 10 alphanumeric characters (optionally formatted with separators)
  * Example: `39J-49L-L8T4` or `39J49LL8T4`

**Behavior**

* Uses EPSG:4326 (WGS84)
* Implements DIGIPIN hierarchical 4×4 grid exactly as per spec
* Handles boundary conditions exactly as defined in the technical document

**Errors**

* Invalid latitude or longitude → error/exception
* Inputs outside bounding box → error/exception

---

### 2.2 Decode: DIGIPIN → geographic coordinates

**Purpose**
Convert a DIGIPIN code to the geographic location it represents.

**Signature (conceptual)**

```
decode(digipin) → location
```

**Parameters**

* `digipin`

  * Type: string
  * Length: 10 characters (separators may be ignored)
  * Case-insensitive (must be normalized internally)

**Return value**

* `location`

  * `latitude` (float, decimal degrees)
  * `longitude` (float, decimal degrees)

The returned coordinates represent the **center (centroid)** of the DIGIPIN grid cell.

**Errors**

* Invalid length
* Invalid characters
* Invalid grid sequence

---

## 3. Optional (but recommended) public helpers

These improve usability but must not change core behavior.

### 3.1 Normalize DIGIPIN

```
normalize(digipin) → digipin
```

* Removes separators
* Converts to uppercase
* Validates character set

---

### 3.2 Validate DIGIPIN

```
is_valid(digipin) → boolean
```

* Returns `true` if syntactically and semantically valid
* Must not throw (pure validation)

---

## 4. Formatting rules

* Internal representation: **no separators**
* Optional formatting for display:

  * After 3rd and 6th character (e.g. `XXX-XXX-XXXX`)
* Formatting must be **cosmetic only**

---

## 5. Error handling rules (important)

* Libraries must use **idiomatic error handling**:

  * Exceptions (Python, Swift, C#)
  * `Result` / `Option` (Rust)
  * `(value, error)` (Go)
* Error messages should be:

  * Deterministic
  * Human-readable
  * Not localized (English only)

---

## 6. What is explicitly out of scope

* Address resolution
* Reverse geocoding
* Maps, tiles, or visualization
* Persistence or databases
* Network services or APIs

---

## 7. Versioning guarantee

* This API contract is **stable starting v1.0.0**
* Any breaking change requires a major version bump
* All language SDKs must adhere to this contract

---

## 8. Compliance requirement

A DIGIPIN library is considered **compliant** if and only if:

* It implements `encode` and `decode`
* It passes all shared test vectors
* It follows the boundary and labeling rules of the official specification
