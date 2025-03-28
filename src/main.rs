use miette::{IntoDiagnostic, Report, Result, WrapErr};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokio::sync::oneshot;

mod cli;
mod config;
mod highlighter;
mod io;
mod theme;

use crate::cli::get_cli;
use crate::io::controller::get_io_and_presenter;
use crate::io::presenter::Present;
use crate::io::reader::AsyncLineReader;
use crate::io::writer::AsyncLineWriter;
use highlighter::groups;
use inlet_manifold::Highlighter;
use theme::reader;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = get_cli()?;

    let io_config = config::get_input_output_config(&cli.args)?;
    let theme = reader::parse_theme(cli.args.config_path)?;
    let highlighter_groups =
        groups::get_highlighter_groups(&cli.args.enabled_highlighters, &cli.args.disabled_highlighters)?;

    let highlighter = highlighter::get_highlighter(
        highlighter_groups,
        theme,
        cli.keyword_config,
        cli.args.no_builtin_keywords,
    )?;

    let (reached_eof_tx, reached_eof_rx) = oneshot::channel::<()>();
    let (io, presenter) = get_io_and_presenter(io_config, Some(reached_eof_tx)).await;

    tokio::spawn(async move {
        process_lines(io, highlighter).await?;

        Ok::<(), Report>(())
    });

    reached_eof_rx
        .await
        .into_diagnostic()
        .wrap_err("Failed to receive EOF signal from oneshot channel")?;

    presenter.present()?;

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
