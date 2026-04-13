# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2025-01-31

### Changed
- Renamed crate from `tform` to `ratatui-form` to avoid confusion with the
  unrelated `tform` crate on crates.io. Users of the previous name should
  update their `Cargo.toml` dependency to `ratatui-form`.

## [0.1.0] - 2025-01-31

### Added
- Initial release.
- Fluent `Form::builder()` API with `.text()`, `.select()`, `.checkbox()`,
  `.block()`, `.title()`, and `.style()` methods.
- Field types: `TextInput`, `Select` (dropdown), `Checkbox`.
- Composite blocks: `AddressBlock`, `ContactBlock`, `DateRangeBlock`.
- Built-in validators: `Required`, `Email`, `MinLength`, `MaxLength`,
  `Pattern` (regex), plus pre-built `Pattern::zip_code()`, `Pattern::phone()`,
  `Pattern::date()`.
- Custom validators via the `Validator` trait.
- Keyboard navigation: Tab / Shift+Tab / Arrow keys / Enter / Space / Esc,
  plus Ctrl+A, Ctrl+E, Ctrl+U in text inputs.
- Theming via `FormStyle` with `dark()` and `light()` presets and fluent
  per-component overrides.
- JSON export via `Form::to_json()` and `Form::write_json()`.

[Unreleased]: https://github.com/DavidLiedle/ratatui-form/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/DavidLiedle/ratatui-form/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/DavidLiedle/ratatui-form/releases/tag/v0.1.0
