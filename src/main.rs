use clap::Parser;
use miette::{IntoDiagnostic, Report, Result, WrapErr};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokio::sync::oneshot;

mod cli;
mod config;
mod highlighter;
mod io;
mod theme;

use crate::cli::{completions, keywords::get_keywords_from_cli, Cli};
use crate::io::controller::get_io_and_presenter;
use crate::io::presenter::Present;
use crate::io::reader::AsyncLineReader;
use crate::io::writer::AsyncLineWriter;
use highlighter::groups;
use inlet_manifold::Highlighter;
use theme::reader;

#[tokio::main]
async fn main() -> Result<()> {
    completions::generate_shell_completions_or_continue();

    let cli = Cli::parse();

    let config = config::create_config(&cli)?;
    let theme = reader::parse_theme(cli.config_path.clone())?;
    let keywords_from_cli = get_keywords_from_cli(&cli);
    let highlighter_groups = groups::get_highlighter_groups(&cli.enable, &cli.disable)?;

    let highlighter =
        highlighter::get_highlighter(highlighter_groups, theme, keywords_from_cli, cli.no_builtin_keywords)?;

    let (reached_eof_tx, reached_eof_rx) = oneshot::channel::<()>();
    let (io, presenter) = get_io_and_presenter(config, Some(reached_eof_tx)).await;

    tokio::spawn(async move {
        process_lines(io, highlighter).await?;

        Ok::<(), Report>(())
    });

    reached_eof_rx
        .await
        .into_diagnostic()
        .wrap_err("Failed to receive EOF signal from oneshot channel")?;

    presenter.present();

    Ok(())
}

async fn process_lines<T: AsyncLineReader + AsyncLineWriter + Unpin + Send>(
    mut io: T,
    highlighter: Highlighter,
) -> Result<()> {
    while let Ok(Some(line)) = io.next_line_batch().await {
        let highlighted_lines = line
            .into_par_iter()
            .map(|line| highlighter.apply(line.as_str()))
            .collect::<Vec<_>>()
            .join("\n");

        io.write_line(&highlighted_lines).await.into_diagnostic()?;
    }

    Ok(())
}
