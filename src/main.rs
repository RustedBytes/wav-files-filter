// src/main.rs

use anyhow::{Context, Result};
use clap::Parser;
use hound::WavReader;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// CLI application to filter WAV audio files by duration.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input directory containing WAV files (processed recursively)
    #[arg(short = 'i', long)]
    input: PathBuf,

    /// Output directory to copy filtered WAV files (preserves relative path structure)
    #[arg(short = 'o', long)]
    output: PathBuf,

    /// Minimum length in milliseconds (default: 0)
    #[arg(short = 'm', long, default_value_t = 0u64)]
    min_length: u64,

    /// Maximum length in milliseconds (default: no limit)
    #[arg(short = 'M', long, default_value_t = u64::MAX)]
    max_length: u64,
}

/// Calculates the duration of a WAV file in milliseconds.
fn get_duration_ms(path: &Path) -> Result<u64> {
    let reader = WavReader::open(path)
        .with_context(|| format!("Failed to open WAV file: {}", path.display()))?;
    let spec = reader.spec();
    let num_samples = reader.len() as f64;
    let duration_sec = num_samples / spec.sample_rate as f64;
    let duration_ms = (duration_sec * 1000.0) as u64;
    Ok(duration_ms)
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate input directory
    if !args.input.exists() {
        anyhow::bail!("Input directory does not exist: {}", args.input.display());
    }
    if !args.input.is_dir() {
        anyhow::bail!("Input path is not a directory: {}", args.input.display());
    }

    // Ensure output directory exists
    fs::create_dir_all(&args.output).with_context(|| {
        format!(
            "Failed to create output directory: {}",
            args.output.display()
        )
    })?;

    let mut copied_count = 0u64;

    // Walk the input directory recursively
    for entry in WalkDir::new(&args.input).follow_links(false).into_iter() {
        let entry = entry.with_context(|| "Failed to read directory entry")?;
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("wav") {
            continue;
        }

        let duration = get_duration_ms(path)?;
        if duration >= args.min_length && duration <= args.max_length {
            // Compute relative path and target output path
            let rel_path = path.strip_prefix(&args.input).with_context(|| {
                format!("Failed to compute relative path for: {}", path.display())
            })?;
            let out_path = args.output.join(rel_path);

            // Ensure parent directories exist
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!(
                        "Failed to create parent directory for: {}",
                        out_path.display()
                    )
                })?;
            }

            // Copy the file
            fs::copy(path, &out_path).with_context(|| {
                format!(
                    "Failed to copy {} to {}",
                    path.display(),
                    out_path.display()
                )
            })?;
            copied_count += 1;
        }
    }

    println!(
        "Filtered and copied {} WAV files to {}",
        copied_count,
        args.output.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    /// Helper to create a temporary WAV file with given sample rate and length in samples.
    fn create_temp_wav(sample_rate: u32, num_samples: u32) -> Result<NamedTempFile> {
        let mut file = NamedTempFile::new()?;
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::new(&mut file, spec)?;
        for _ in 0..num_samples {
            writer.write_sample(i16::default())?;
        }
        writer.finalize()?;
        file.as_file().sync_all()?;
        Ok(file)
    }

    #[test]
    fn test_get_duration_ms() -> Result<()> {
        // Test with 1 second duration at 44100 Hz (44100 samples)
        let temp_file = create_temp_wav(44100, 44100)?;
        let duration = get_duration_ms(temp_file.path())?;
        assert_eq!(duration, 1000);

        // Test with 500 ms duration (22050 samples)
        let temp_file = create_temp_wav(44100, 22050)?;
        let duration = get_duration_ms(temp_file.path())?;
        assert_eq!(duration, 500);

        // Test with 0 samples (0 ms)
        let temp_file = create_temp_wav(44100, 0)?;
        let duration = get_duration_ms(temp_file.path())?;
        assert_eq!(duration, 0);

        Ok(())
    }

    #[test]
    fn test_get_duration_ms_invalid_file() {
        let result = get_duration_ms(Path::new("nonexistent.wav"));
        assert!(result.is_err());
    }
}
