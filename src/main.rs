mod cli;
mod config;
mod highlight_processor;
mod highlight_utils;
mod highlighter;
mod highlighters;
mod io;
mod keyword;
mod line_info;
mod theme;
mod theme_io;
mod theme_legacy;
mod types;

use crate::highlight_processor::HighlightProcessor;
use crate::io::controller::get_io_and_presenter;
use crate::io::presenter::Present;
use crate::io::reader::AsyncLineReader;
use crate::io::writer::AsyncLineWriter;
use crate::types::Config;
use color_eyre::eyre::Result;
use highlighter::groups;
use inlet_manifold::Highlighter;
use theme::reader;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = cli::get_args_or_exit_early();
    let config = config::create_config_or_exit_early(&cli);

    let cli_options = config::get_cli_opts_for_highlight_groups(&cli);
    let highlighter_groups = groups::get_highlighter_groups(cli_options)?;

    let new_theme = reader::parse_theme(cli.config_path.clone())?;
    let highlighter = highlighter::get_highlighter(highlighter_groups, new_theme, vec![], cli.no_builtin_keywords)?;

    run(highlighter, config).await;

    Ok(())
}

pub async fn run(highlighter: Highlighter, config: Config) {
    let (reached_eof_tx, reached_eof_rx) = oneshot::channel::<()>();
    let (io, presenter) = get_io_and_presenter(config, Some(reached_eof_tx)).await;

    let highlight_processor = HighlightProcessor::new(highlighter);

    tokio::spawn(process_lines(io, highlight_processor));

    reached_eof_rx
        .await
        .expect("Could not receive EOF signal from oneshot channel");

    presenter.present();
}

async fn process_lines<T: AsyncLineReader + AsyncLineWriter + Unpin + Send>(
    mut io: T,
    highlight_processor: HighlightProcessor,
) {
    while let Ok(Some(line)) = io.next_line().await {
        let highlighted_lines = highlight_processor.apply(line);
        io.write_line(&highlighted_lines).await.unwrap();
    }
}
