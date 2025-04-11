use miette::{IntoDiagnostic, Report, Result};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tailspin::Highlighter;

mod cli;
mod config;
mod eof_signal;
mod highlighter_builder;
mod io;
mod theme;

use crate::cli::get_config;
use crate::io::controller::{Io, initialize_io};
use crate::io::presenter::Present;
use crate::io::reader::AsyncLineReader;
use crate::io::writer::AsyncLineWriter;

#[tokio::main]
async fn main() -> Result<()> {
    let config = get_config()?;
    let (io, presenter, eof_signal_receiver) = initialize_io(config.source, config.output_target).await;

    let line_processing = tokio::spawn(async move {
        process_lines(io, config.highlighter).await?;
        Ok::<(), Report>(())
    });

    eof_signal_receiver.wait().await?;

    let presenter_task = tokio::spawn(async move { presenter.present() });

    tokio::select! {
        res = presenter_task => {
            res.into_diagnostic()??;
        },
        res = line_processing => {
            res.into_diagnostic()??;
        }
    }

    Ok(())
}

async fn process_lines(mut io: Io, highlighter: Highlighter) -> Result<()> {
    while let Ok(Some(line)) = io.reader.next_line_batch().await {
        let highlighted_lines = line
            .into_par_iter()
            .map(|line| highlighter.apply(line.as_str()).to_string())
            .collect::<Vec<_>>()
            .join("\n");

        io.writer.write_line(&highlighted_lines).await.into_diagnostic()?;
    }

    Ok(())
}
