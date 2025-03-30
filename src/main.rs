use miette::{IntoDiagnostic, Report, Result, WrapErr};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokio::sync::oneshot;

mod cli;
mod config;
mod highlighter_builder;
mod io;
mod theme;

use crate::cli::get_config;
use crate::io::controller::get_io_and_presenter;
use crate::io::presenter::Present;
use crate::io::reader::AsyncLineReader;
use crate::io::writer::AsyncLineWriter;
use inlet_manifold::Highlighter;

#[tokio::main]
async fn main() -> Result<()> {
    let config = get_config()?;

    let (reached_eof_tx, reached_eof_rx) = oneshot::channel::<()>();
    let (io, presenter) = get_io_and_presenter(config.input, config.output, Some(reached_eof_tx)).await;

    tokio::spawn(async move {
        process_lines(io, config.highlighter).await?;

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
