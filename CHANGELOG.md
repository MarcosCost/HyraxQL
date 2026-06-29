# Changelog

All notable changes to this project will be documented in this file.

## [0.1.1] - 2026-06-29
### Added
- Full codebase refactor to a trait-based layered architecture for extensibility
- Connection system now supports URL strings directly — no need to break into config objects
- URL validation for SQL databases with 25+ test cases
- Unified error type across all layers

### Changed
- Database logic split into connection / commands / engine layers
- Connection factory handles both raw URLs and structured configs
- Commands are now backend-agnostic via the Connection trait
- Engine acts as single entry point for UI consumers

### Fixed
- URL-based connections now actually work (were broken before)
- SQL dialect logic moved into the connection layer instead of commands

## [0.1.0] - 2026-05-21
### Added
- Basic TUI
- Connection to suported relational databases
- Explore Database Feature (in development)

### Security
- Added MIT License
