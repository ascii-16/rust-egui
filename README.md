# rust-iced-converter

A simple and modern unit converter desktop app built with Rust and the [iced](https://github.com/iced-rs/iced) GUI framework.

> **Note:** This project is part of my learning journey with Rust and GUI development.

## Screenshots

![App Screenshot]([https://example.com/screenshot.png](https://private-user-images.githubusercontent.com/42437801/479846911-87daad98-fd2a-488b-85f5-22728273cc0d.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NTU2NzEzMTIsIm5iZiI6MTc1NTY3MTAxMiwicGF0aCI6Ii80MjQzNzgwMS80Nzk4NDY5MTEtODdkYWFkOTgtZmQyYS00ODhiLTg1ZjUtMjI3MjgyNzNjYzBkLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNTA4MjAlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjUwODIwVDA2MjMzMlomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTI2ZmEwYjgwNWUwZmE5YWM1YTU1OThjNmEyMzljNTdiNzRmOTY2ZTQ0ZTk5M2RlM2JkZTFmZGE1NmU3Y2RmZDMmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.m9q4u_R3SaEfJQdLAMRc1kna_wv2ma45b3NKSE6vv40))

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
