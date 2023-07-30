mod cli;
mod color;
mod config_io;
mod controller;
mod highlight_processor;
mod highlight_utils;
mod highlighters;
mod io_stream;
mod less;
mod line_info;
mod presenter;
mod reader;
mod tail;
mod theme;
mod types;
mod writer;

use crate::cli::Cli;
use crate::controller::create_io_and_presenter;
use crate::highlight_processor::HighlightProcessor;
use crate::presenter::Present;
use crate::reader::AsyncLineReader;
use crate::writer::AsyncLineWriter;
use rand::random;
use std::fs;
use std::fs::File as StdFile;
use std::io::{stdin, BufRead, IsTerminal};
use std::path::{Path, PathBuf};
use std::process::exit;
use tokio::fs::File as TokioFile;
use tokio::io::BufWriter;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let args = cli::get_args();

    if should_exit_early(&args) {
        exit(0);
    }

    let follow = should_follow(args.follow, args.tail_command.is_some());
    let is_stdin = !stdin().is_terminal();

    let file_path = match args.file_path.clone() {
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
    let config = config_io::load_config(config_path);

    let number_of_lines = count_lines(file_path.clone(), follow);

    let highlighter = highlighters::Highlighters::new(config);
    let highlight_processor = HighlightProcessor::new(highlighter);

    let (_temp_dir, output_path, output_writer) = create_temp_file().await;
    let (reached_eof_tx, reached_eof_rx) = oneshot::channel::<()>();

    dbg!("starting tailing with TailFileIoStream");

    // let input = Input::FilePath(file_path);
    // let output = Output::TempFile;
    let (io, presenter) =
        create_io_and_presenter(file_path, number_of_lines, Some(reached_eof_tx)).await;

    tokio::spawn(process_lines(io, highlight_processor));

    reached_eof_rx
        .await
        .expect("Could not receive EOF signal from oneshot channel");

    presenter.present();

    cleanup(output_path);
}

fn should_exit_early(args: &Cli) -> bool {
    if args.generate_shell_completions.is_some() {
        cli::print_completions_to_stdout();
        return true;
    }

    if args.create_default_config {
        config_io::create_default_config();
        return true;
    }

    if args.show_default_config {
        let default_config = config_io::default_config();
        println!("{}", default_config);
        return true;
    }

    false
}

async fn process_lines<T: AsyncLineReader + AsyncLineWriter + Unpin + Send>(
    mut io: T,
    highlight_processor: HighlightProcessor,
) {
    while let Ok(Some(line)) = io.next_line().await {
        let highlighted_line = highlight_processor.apply(&line);
        io.write_line(&highlighted_line).await.unwrap();
    }
}

fn should_follow(follow: bool, has_follow_command: bool) -> bool {
    if has_follow_command {
        return true;
    }

    follow
}

async fn create_temp_file() -> (tempfile::TempDir, PathBuf, BufWriter<TokioFile>) {
    let unique_id: u32 = random();
    let filename = format!("tailspin.temp.{}", unique_id);

    let temp_dir = tempfile::tempdir().unwrap();

    let output_path = temp_dir.path().join(filename);
    let output_file = TokioFile::create(&output_path).await.unwrap();
    let output_writer = BufWriter::new(output_file);

    (temp_dir, output_path, output_writer)
}

fn cleanup(output_path: PathBuf) {
    if let Err(err) = fs::remove_file(output_path) {
        eprintln!("Failed to remove the temporary file: {}", err);
    }
}

fn count_lines<P: AsRef<Path>>(file_path: P, follow: bool) -> usize {
    if follow {
        return 1;
    }

    let file = StdFile::open(file_path).expect("Could not open file");
    let reader = std::io::BufReader::new(file);

    reader.lines().count()
}
