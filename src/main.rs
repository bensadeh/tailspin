mod cli;
mod color;
mod config;
mod highlight_processor;
mod highlight_utils;
mod highlighters;
mod io;
mod keyword;
mod line_info;
mod regex;
mod theme;
mod theme_io;
mod types;

use crate::cli::Cli;
use crate::highlight_processor::HighlightProcessor;
use crate::theme::Theme;
use crate::types::Config;
use color_eyre::eyre::Result;
use io::controller::{get_reader_and_writer, Reader, Writer};
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = cli::get_args_or_exit_early();
    let theme = theme_io::load_theme(cli.config_path.clone());
    let config = config::create_config_or_exit_early(&cli);

    run(theme, config, cli).await;

    Ok(())
}

pub async fn run(theme: Theme, config: Config, cli: Cli) {
    let (eof_signaler, eof_receiver) = oneshot::channel::<()>();
    let (reader, writer) = get_reader_and_writer(config, Some(eof_signaler)).await;
    let highlight_processor = {
        let highlighter = highlighters::Highlighters::new(&theme, &cli);
        HighlightProcessor::new(highlighter)
    };

    let page_process = tokio::spawn(start(reader, writer, highlight_processor));

    let (res1, res2) = tokio::join!(page_process, eof_receiver);
    res1.unwrap();
    res2.expect("Could not receive EOF signal from oneshot channel");
}

async fn start(mut reader: Reader, mut writer: Writer, highlight_processor: HighlightProcessor) {
    while let Ok(Some(line)) = reader.next_line().await {
        let highlighted_line = highlight_processor.apply(&line);
        writer.write_line(&highlighted_line).await.unwrap();
    }
}
