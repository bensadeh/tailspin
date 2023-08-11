mod cli;
mod color;
mod controller;
mod file_utils;
mod highlight_processor;
mod highlight_utils;
mod highlighters;
mod io_stream;
mod line_info;
mod presenter;
mod reader;
mod regexes;
mod theme;
mod theme_io;
mod types;
mod writer;

use crate::cli::Cli;
use crate::controller::config::create_config;
use crate::controller::get_io_and_presenter;
use crate::highlight_processor::HighlightProcessor;
use crate::presenter::Present;
use crate::reader::AsyncLineReader;
use crate::writer::AsyncLineWriter;

use color_eyre::eyre::Result;
use std::process::exit;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = cli::get_args();

    if should_exit_early(&args) {
        exit(0);
    }

    let config_path = args.config_path.clone();
    let theme = theme_io::load_theme(config_path);

    let highlighter = highlighters::Highlighters::new(theme);
    let highlight_processor = HighlightProcessor::new(highlighter);

    let (reached_eof_tx, reached_eof_rx) = oneshot::channel::<()>();

    let config = match create_config(args) {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e.message);
            exit(e.exit_code);
        }
    };

    let (io, presenter) = get_io_and_presenter(config, Some(reached_eof_tx)).await;

    tokio::spawn(process_lines(io, highlight_processor));

    reached_eof_rx
        .await
        .expect("Could not receive EOF signal from oneshot channel");

    presenter.present();

    Ok(())
}

fn should_exit_early(args: &Cli) -> bool {
    if args.generate_shell_completions.is_some() {
        cli::print_completions_to_stdout();
        return true;
    }

    if args.create_default_config {
        theme_io::create_default_config();
        return true;
    }

    if args.show_default_config {
        let default_config = theme_io::default_theme();
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
