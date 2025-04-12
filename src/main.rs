use miette::{IntoDiagnostic, Result, miette};
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
    let (io, presenter, initial_read_complete_sender) = initialize_io(config.source, config.output_target).await;

    let read_write_apply_task = tokio::spawn(async move { read_write_and_apply(io, config.highlighter).await });

    initial_read_complete_sender.wait().await?;

    let presenter_task = tokio::spawn(async move { presenter.present() });

    tokio::select! {
        res = presenter_task => res.into_diagnostic()??,
        res = read_write_apply_task => res.into_diagnostic()??,
    }

    Ok(())
}

async fn read_write_and_apply(mut io: Io, highlighter: Highlighter) -> Result<()> {
    while let Some(line_batch) = io.reader.next_line_batch().await? {
        let highlighted_lines = line_batch
            .into_par_iter()
            .map(|line| highlighter.apply(line.as_str()).to_string())
            .collect::<Vec<_>>()
            .join("\n");

        io.writer.write_line(&highlighted_lines).await?;
    }

    Ok(())
}
