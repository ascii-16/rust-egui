# rust-iced-unit-converter

A simple and modern unit converter desktop app built with Rust and the [iced](https://github.com/iced-rs/iced) GUI framework.

> **Note:** This project is part of my learning journey with Rust and GUI development.

## Screenshots

![App Screenshot](https://private-user-images.githubusercontent.com/42437801/480401591-4b5a0b6f-cc77-4ec6-aa67-af246853385f.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NTU3NjUwODAsIm5iZiI6MTc1NTc2NDc4MCwicGF0aCI6Ii80MjQzNzgwMS80ODA0MDE1OTEtNGI1YTBiNmYtY2M3Ny00ZWM2LWFhNjctYWYyNDY4NTMzODVmLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNTA4MjElMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjUwODIxVDA4MjYyMFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTI4Y2I3MjNlNzkwYmQwOWY2ZGY3YWU4NTUzNWQ2MTdhMDE5YjM3NjZkYWVmZjQ3OGVhOGQwZWI4MzI2MjNkMDEmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.S3gAa1M93jXMrcto9Np3j1pTojaGQNl6GvNq4HhaMdQ)

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (edition 2024 or later)

### Running the App

1. Clone the repository:
   ```sh
   git clone https://github.com/ascii-16/rust-egui.git](https://github.com/ascii-16/rust-iced-unit-converter.git
   cd rust-iced-unit-converter
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
