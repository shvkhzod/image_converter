use clap::Parser;
use webp::Encoder;
use anyhow::Result;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use tracing::info;
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(author, version, about = "Convert images to WebP format", long_about = None)]
struct Args {
    /// Input image files
    #[arg(required = true)]
    inputs: Vec<PathBuf>,

    /// Output directory
    #[arg(short, long, default_value = "output")]
    output_dir: PathBuf,

    /// Quality (0-100)
    #[arg(short, long, default_value_t = 80)]
    quality: u8,
}

fn convert_to_webp(input_path: &PathBuf, output_path: &PathBuf, quality: u8) -> Result<()> {
    // Read the input image
    let img = image::io::Reader::open(input_path)?
        .with_guessed_format()?
        .decode()?
        .to_rgba8();

    // Create WebP encoder
    let encoder = Encoder::from_rgba(img.as_raw(), img.width(), img.height());
    let encoded = encoder.encode(quality as f32 / 100.0);

    // Write to output file
    let mut output_file = File::create(output_path)?;
    output_file.write_all(&encoded)?;

    info!("Converted {} to {}", input_path.display(), output_path.display());
    Ok(())
}

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse arguments
    let args = Args::parse();

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output_dir)?;

    // Process each input file
    for input_path in &args.inputs {
        let input_filename = input_path.file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid input path: {}", input_path.display()))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid filename: {}", input_path.display()))?;

        // Create output path with .webp extension
        let output_filename = format!("{}.webp", input_filename.trim_end_matches(|c: char| c == '.'));
        let output_path = args.output_dir.join(output_filename);

        // Convert the image
        convert_to_webp(&input_path, &output_path, args.quality)?;
    }

    info!("Conversion completed successfully!");
    Ok(())
}
