mod color;
mod config_parser;
mod config_util;
mod highlight_processor;
mod highlight_utils;
mod highlighters;
mod less;
mod line_info;
mod tail;

use clap::Parser;
use rand::random;
use std::fs::File;
use std::io::{BufWriter, IsTerminal};
use std::path::PathBuf;
use std::process::exit;
use tokio::sync::oneshot;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[command(name = "spin")]
#[command(about = "A log file highlighter")]
struct Args {
    /// Filepath
    #[clap(name = "FILE")]
    file_path: Option<String>,

    /// Follow (tail) the contents of the file
    #[clap(short = 'f', long = "follow")]
    follow: bool,

    /// Path to a custom configuration file
    #[clap(short = 'c', long = "config-path")]
    config_path: Option<String>,

    /// Tails the output of the provided command
    #[clap(short = 't', long = "tail-command")]
    tail_command: Option<String>,

    /// Generate a new configuration file
    #[clap(long = "generate-config")]
    generate_config: bool,

    /// Print the default configuration
    #[clap(long = "print-default-config", conflicts_with = "generate_config")]
    print_default_config: bool,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    let follow = should_follow(args.follow, args.tail_command.is_some());
    let is_stdin = !std::io::stdin().is_terminal();

    if args.generate_config {
        config_parser::generate_default_config();
        exit(0);
    }

    if args.print_default_config {
        // call the function to print default config
        println!("print default config");
        exit(0);
    }

    let file_path = match args.file_path {
        Some(path) => path,
        None => {
            if !is_stdin && args.tail_command.is_none() {
                println!("Missing filename (`spin --help` for help) ");

                exit(0);
            }

            "".to_string()
        }
    };

    let config_path = args.config_path.clone();
    let config = config_parser::load_config(config_path);

    let highlighter = highlighters::Highlighters::new(config);
    let highlight_processor = highlight_processor::HighlightProcessor::new(highlighter);

    let (_temp_dir, output_path, output_writer) = create_temp_file();
    let (reached_eof_tx, reached_eof_rx) = oneshot::channel::<()>();

    if is_stdin {
        tokio::spawn(async move {
            tail::tail_stdin(
                output_writer,
                highlight_processor,
                follow,
                Some(reached_eof_tx),
            )
            .await
            .expect("Failed to tail file");
        });
    } else if args.tail_command.is_some() {
        tokio::spawn(async move {
            tail::tail_command_output(
                output_writer,
                highlight_processor,
                Some(reached_eof_tx),
                args.tail_command.unwrap().as_str(),
            )
            .await
            .expect("Failed to tail file");
        });
    } else {
        tokio::spawn(async move {
            tail::tail_file(
                &file_path,
                follow,
                output_writer,
                highlight_processor,
                Some(reached_eof_tx),
            )
            .await
            .expect("Failed to tail file");
        });
    }

    reached_eof_rx
        .await
        .expect("Could not receive EOF signal from oneshot channel");

    less::open_file_with_less(output_path.to_str().unwrap(), follow);

    cleanup(output_path);
}

fn should_follow(follow: bool, has_follow_command: bool) -> bool {
    if has_follow_command {
        return true;
    }

    follow
}

fn create_temp_file() -> (tempfile::TempDir, PathBuf, BufWriter<File>) {
    let unique_id: u32 = random();
    let filename = format!("tailspin.temp.{}", unique_id);

    let temp_dir = tempfile::tempdir().unwrap();

    let output_path = temp_dir.path().join(filename);
    let output_file = File::create(&output_path).unwrap();
    let output_writer = BufWriter::new(output_file);

    (temp_dir, output_path, output_writer)
}

fn cleanup(output_path: PathBuf) {
    if let Err(err) = std::fs::remove_file(output_path) {
        eprintln!("Failed to remove the temporary file: {}", err);
    }
}
