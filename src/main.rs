mod color;
mod config_parser;
mod config_util;
mod highlight_processor;
mod highlight_utils;
mod highlighters;
mod less;
mod line_info;
mod tail;

use rand::random;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufWriter};
use std::path::{Path, PathBuf};
use std::process::exit;
use tokio::sync::oneshot;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(name = "FILE")]
    input: String,

    /// Follow (tail) the contents of the file
    #[clap(short = 'f', long = "follow")]
    follow: bool,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    let input = args.input.clone();
    let input_path = Path::new(&input);

    if !input_path.exists() {
        eprintln!(
            "Error: File '{}' does not exist",
            input_path.to_str().unwrap()
        );
        exit(1);
    }

    let config = config_parser::load_config(None);

    let line_count = if args.follow {
        1
    } else {
        count_lines(input_path).expect("Failed to count lines")
    };

    let highlighter = highlighters::Highlighters::new(config);
    let highlight_processor = highlight_processor::HighlightProcessor::new(highlighter);

    let unique_id: u32 = random();
    let filename = format!("tailspin.temp.{}", unique_id);
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join(filename);
    let output_file = File::create(&output_path).unwrap();
    let output_writer = BufWriter::new(output_file);

    let (reached_eof_tx, reached_eof_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        tail::tail_file(
            &input,
            output_writer,
            highlight_processor,
            line_count,
            Some(reached_eof_tx),
        )
        .await
        .expect("Failed to tail file");
    });

    // Wait for the signal from the other task before continuing
    reached_eof_rx
        .await
        .expect("Failed receiving from oneshot channel");

    less::open_file_with_less(output_path.to_str().unwrap(), args.follow);

    cleanup(output_path);
}

fn cleanup(output_path: PathBuf) {
    if let Err(err) = std::fs::remove_file(output_path) {
        eprintln!("Failed to remove the temporary file: {}", err);
    }
}

fn count_lines<P: AsRef<Path>>(file_path: P) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines().count())
}
