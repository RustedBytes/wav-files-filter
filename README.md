# wav-files-filter

A simple, efficient command-line tool for recursively filtering WAV audio files from a directory based on their duration (in milliseconds). Files within the specified min/max length range are copied to an output directory, preserving the original folder structure.

## Features

- **Recursive Processing**: Scans input directory and all subfolders for `.wav` files.
- **Duration Filtering**: Computes audio duration using sample rate and length; filters based on user-defined bounds.
- **Structure Preservation**: Maintains relative paths in the output directory.
- **Error Handling**: Robust with detailed error messages for invalid files or paths.
- **Performance**: Lightweight, using only essential crates; no unnecessary dependencies.

## Installation

### From Crates.io (Recommended)

```bash
cargo install --git https://github.com/RustedBytes/wav-files-filter
```

### From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/RustedBytes/wav-files-filter
   cd wav-files-filter
   ```

2. Build and install:
   ```bash
   cargo install --path .
   ```

Requires Rust 1.70+ (stable channel).

## Usage

```bash
wav-files-filter [OPTIONS]
```

### Options

- `-i, --input <INPUT>`: Input directory path (required; processed recursively).
- `-o, --output <OUTPUT>`: Output directory path (required; created if it doesn't exist).
- `-m, --min-length <MIN_LENGTH>`: Minimum duration in ms (default: `0`).
- `-M, --max-length <MAX_LENGTH>`: Maximum duration in ms (default: unlimited).

Run `wav-files-filter --help` for full details.

## Examples

### Basic Filtering

Filter WAV files longer than 500ms and shorter than 5000ms from `input_dir` to `output_dir`:

```bash
wav-files-filter -i input_dir -o output_dir -m 500 -M 5000
```

### No Minimum Length (All Files Up to 10 Seconds)

```bash
wav-files-filter -i ./sounds -o ./filtered_sounds -M 10000
```

### Dry Run (Preview Without Copying)

The tool doesn't have a built-in dry-run flag yet; for now, you can inspect durations manually or extend the source if needed.

## Output

- Filtered files are copied to the output directory with the same relative structure (e.g., `input_dir/subfolder/file.wav` → `output_dir/subfolder/file.wav`).
- Console output: Reports the number of files copied (e.g., "Filtered and copied 42 WAV files to ./output_dir").

## Testing

Run the test suite:

```bash
cargo test
```

Includes unit tests for duration calculation (happy paths, edges like 0ms, invalid files) using temporary files.

## Dependencies

- `clap`: Argument parsing.
- `hound`: WAV file reading.
- `walkdir`: Recursive directory traversal.
- `anyhow`: Error handling.

See `Cargo.toml` for versions.

## Contributing

1. Fork the repo.
2. Create a feature branch (`git checkout -b feature/AmazingFeature`).
3. Commit changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a Pull Request.
