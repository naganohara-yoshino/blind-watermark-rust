use anyhow::Result;
use blind_watermark::prelude::*;
use clap::{ArgGroup, Args, Parser, Subcommand};
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Embed(args) => run_embed(args),
        Commands::Extract(args) => run_extract(args),
    }
}

/// Watermark CLI tool
#[derive(Parser, Debug)]
#[command(name = "watermark", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Embed a watermark
    Embed(EmbedArgs),

    /// Extract a watermark
    Extract(ExtractArgs),
}

#[derive(Args, Debug)]
#[command(group(
    ArgGroup::new("output_mode")
        .required(true)
        .args(["output", "prefix"])
))]
pub struct EmbedArgs {
    /// Input file or directory
    #[arg(short, long)]
    pub input: PathBuf,

    /// Watermark string
    #[arg(short, long)]
    pub string: String,

    /// Output file (conflicts with --prefix)
    #[arg(short, long, conflicts_with = "prefix")]
    pub output: Option<PathBuf>,

    /// Prefix for batch mode (conflicts with --output)
    #[arg(short, long, conflicts_with = "output")]
    pub prefix: Option<String>,

    /// Optional seed
    #[arg(short, long)]
    pub seed: Option<u64>,

    /// Recursively scan directory
    #[arg(short, long)]
    pub recursive: bool,
}

#[derive(Args, Debug)]
pub struct ExtractArgs {
    /// Input file
    #[arg(short, long, value_parser = is_file)]
    pub input: PathBuf,

    /// Expected watermark length
    #[arg(short, long)]
    pub length: usize,

    /// Optional seed
    #[arg(short, long)]
    pub seed: Option<u64>,
}

fn is_file(s: &str) -> Result<PathBuf, String> {
    let p = PathBuf::from(s);
    if p.is_file() {
        Ok(p)
    } else {
        Err("extract --input only accepts a single file".into())
    }
}

fn run_embed(args: EmbedArgs) {
    if let Some(out) = args.output {
        embed_watermark_string(args.input, out, &args.string, args.seed).unwrap();
    } else {
        let walk = match args.recursive {
            true => WalkBuilder::new(&args.input)
                .standard_filters(true)
                .hidden(false)
                .build(),
            false => WalkBuilder::new(&args.input)
                .standard_filters(true)
                .hidden(false)
                .max_depth(Some(1))
                .build(),
        };

        // Collect image files first
        let files: Vec<PathBuf> = walk
            .filter_map(|e| {
                if let Ok(e) = e
                    && let Some(ext) = e.path().extension()
                    && let Some(ext) = ext.to_str()
                    && matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "webp")
                {
                    Some(e.path().to_path_buf())
                } else {
                    None
                }
            })
            .collect();

        let pb = ProgressBar::new(files.len() as u64);
        let style =
            ProgressStyle::with_template("Embedding [{bar:60.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#> ");
        pb.set_style(style);

        let prefix = args
            .prefix
            .expect("--prefix is required for directory input");

        // Multi-threaded processing
        files.par_iter().for_each(|input| {
            let stem = input
                .file_stem()
                .expect("illformed input")
                .to_str()
                .expect("illformed input");
            let ext = input
                .extension()
                .expect("illformed input")
                .to_str()
                .expect("illformed input");
            let output = input.with_file_name(format!("{}{}.{}", prefix, stem, ext));
            pb.println(format!("Embedding {}", input.display()));
            // Run embed
            embed_watermark_string(input, &output, &args.string, args.seed).unwrap();

            pb.inc(1);
        });

        pb.finish_with_message("Done");
    }
}

fn run_extract(args: ExtractArgs) {
    let extracted = extract_watermark_string(args.input, args.length, args.seed)
        .expect("Failed to extract watermark");

    println!("Extracted watermark: {}", extracted);
}
