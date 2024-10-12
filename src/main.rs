use rayon::iter::ParallelIterator;
mod cli;
mod config;
mod highlighter_builder;
mod io;
mod theme;
mod types;

use crate::cli::keywords::get_keywords_from_cli;
use crate::io::controller::get_io_and_presenter;
use crate::io::presenter::Present;
use crate::io::reader::AsyncLineReader;
use crate::io::writer::AsyncLineWriter;
use crate::types::Config;
use color_eyre::eyre::Result;
use highlighter_builder::groups;
use inlet_manifold::Highlighter;
use rayon::iter::IntoParallelIterator;
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
    let keywords_from_cli = get_keywords_from_cli(&cli);

    let highlighter = highlighter_builder::get_highlighter(
        highlighter_groups,
        new_theme,
        keywords_from_cli,
        cli.no_builtin_keywords,
    )?;

    run(highlighter, config).await;

    Ok(())
}

pub async fn run(highlighter: Highlighter, config: Config) {
    let (reached_eof_tx, reached_eof_rx) = oneshot::channel::<()>();
    let (io, presenter) = get_io_and_presenter(config, Some(reached_eof_tx)).await;

    tokio::spawn(process_lines(io, highlighter));

    reached_eof_rx
        .await
        .expect("Could not receive EOF signal from oneshot channel");

    presenter.present();
}

async fn process_lines<T: AsyncLineReader + AsyncLineWriter + Unpin + Send>(mut io: T, highlighter: Highlighter) {
    while let Ok(Some(line)) = io.next_line_batch().await {
        let highlighted_lines = line
            .into_par_iter()
            .map(|line| highlighter.apply(line))
            .collect::<Vec<_>>()
            .join("\n");

        io.write_line(&highlighted_lines).await.unwrap();
    }
}
