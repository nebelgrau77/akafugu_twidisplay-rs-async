# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
...

## [0.2.0] - 2026-02-01

### Updated
- Uses Embedded HAL 1.0.0

### Added
- Example using ESP32C3 with Embassy


## [0.1.1] - 2021-11-14

### Added
- Display date

## [0.1.0] - 2021-11-01

### Added
- Display dots

## [0.0.9] - 2021-11-01

### Added 
- Use example in the readme.md file

### Changed
- Repository status to public

## [0.0.5] - 2021-10-31

### Added
- Setting the I2C address (requires more testing)
- Setting display mode (rotate/scroll)
- Displaying a single character at a given position
- Sending a single character to the display
- Displaying a number using all the digits (with leading zeros)
- Displaying temperature with a chosen unit, no leading zeros, minus sign, lower and upper threshold, LL/HH if below/over threshold, ---- if exceeding -99/999
- Displaying humidity between 0 and 100 with settable lower/upper threshold
- Displaying time in HH.MM format with an optional dot

[0.0.5]: https://github.com/nebelgrau77/akafugu_twidisplay-rs/releases/tag/v0.0.5

## [0.0.1] - 2021-10-17

### Added
- Clearing display
- Showing the current I2C address
- Setting brightness level
- 0.Displaying a single digit at a given position
- Sending a single digit to the display

[0.0.1]: https://github.com/nebelgrau77/akafugu_twidisplay-rs/releases/tag/v0.0.1