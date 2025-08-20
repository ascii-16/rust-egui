# rust-iced-converter

A simple and modern unit converter desktop app built with Rust and the [iced](https://github.com/iced-rs/iced) GUI framework.

> **Note:** This project is part of my learning journey with Rust and GUI development.

## Screenshots

*(Add screenshots here if available)*

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (edition 2024 or later)

### Running the App

1. Clone the repository:
   ```sh
   git clone https://github.com/ascii-16/rust-egui.git
   cd rust-egui
   ```
2. Create the data dir:
   ```sh
   mkdir data
   ```
3. Build and run the app:
   ```sh
   cargo run
   ```

The app window should appear. Select a category, enter a value, and convert between units!

## Project Structure

- `src/` - Main source code
  - `ui/` - UI components
  - `shared/` - Shared types and logic
  - `state/` - App state and messages
  - `units/` - Unit conversion logic
- `data/conversions.json` - Conversion data

## Dependencies
- [iced](https://github.com/iced-rs/iced)
- serde, serde_json

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
