use miette::{IntoDiagnostic, Result};
use rayon::prelude::*;
use tailspin::Highlighter;

mod cli;
mod config;
mod highlighter_builder;
mod initial_read;
mod io;
mod theme;

use crate::io::controller::{Io, initialize_io};
use crate::io::presenter::Present;
use crate::io::reader::{AsyncLineReader, ReadType};
use crate::io::writer::AsyncLineWriter;

#[tokio::main]
async fn main() -> Result<()> {
    let (io, presenter, highlighter, initial_read_complete_sender) = initialize_io().await?;

    let read_write_highlight_task = tokio::spawn(async move { read_write_and_highlight(io, highlighter).await });

    initial_read_complete_sender.wait().await?;

    let presenter_task = tokio::spawn(async move { presenter.present() });

    tokio::select! {
        res = presenter_task => res.into_diagnostic()??,
        res = read_write_highlight_task => res.into_diagnostic()??,
    }

    Ok(())
}

async fn read_write_and_highlight(mut io: Io, highlighter: Highlighter) -> Result<()> {
    loop {
        match io.reader.next().await? {
            ReadType::StreamEnded => return Ok(()),
            ReadType::SingleLine(line) => write_line(&mut io, &highlighter, line.as_str()).await?,
            ReadType::MultipleLines(lines) => write_batch(&mut io, &highlighter, lines).await?,
        }
    }
}

async fn write_line(io: &mut Io, highlighter: &Highlighter, line: &str) -> Result<()> {
    let highlighted = &highlighter.apply(line);

    io.writer.write_line(highlighted).await?;

    Ok(())
}

async fn write_batch(io: &mut Io, highlighter: &Highlighter, lines: Vec<String>) -> Result<()> {
    let highlighted = lines
        .par_iter()
        .map(|line| highlighter.apply(line.as_str()))
        .collect::<Vec<_>>()
        .join("\n");

    io.writer.write_line(&highlighted).await?;

    Ok(())
}
