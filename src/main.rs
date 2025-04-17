use crate::io::controller::{Reader, Writer};
use io::controller::initialize_io;
use io::presenter::Present;
use io::reader::{AsyncLineReader, ReadType};
use io::writer::AsyncLineWriter;
use miette::{IntoDiagnostic, Result};
use rayon::prelude::*;
use tailspin::Highlighter;
use tokio::{main, select, spawn};

mod cli;
mod config;
mod highlighter_builder;
mod initial_read;
mod io;
mod theme;

#[main]
async fn main() -> Result<()> {
    let (reader, writer, presenter, highlighter, initial_read_complete_receiver) = initialize_io().await?;

    let read_write_highlight_task = spawn(async move { read_write_and_highlight(reader, writer, highlighter).await });

    initial_read_complete_receiver.receive().await?;

    let presenter_task = spawn(async move { presenter.present() });

    select! {
        result = presenter_task => result.into_diagnostic()??,
        result = read_write_highlight_task => result.into_diagnostic()??,
    }

    Ok(())
}

async fn read_write_and_highlight(mut reader: Reader, mut writer: Writer, highlighter: Highlighter) -> Result<()> {
    loop {
        match reader.next().await? {
            ReadType::StreamEnded => return Ok(()),
            ReadType::SingleLine(line) => write_line(&mut writer, &highlighter, line.as_str()).await?,
            ReadType::MultipleLines(lines) => write_batch(&mut writer, &highlighter, lines).await?,
        }
    }
}

async fn write_line(writer: &mut Writer, highlighter: &Highlighter, line: &str) -> Result<()> {
    let highlighted = &highlighter.apply(line);

    writer.write_line(highlighted).await?;

    Ok(())
}

async fn write_batch(writer: &mut Writer, highlighter: &Highlighter, lines: Vec<String>) -> Result<()> {
    let highlighted = lines
        .par_iter()
        .map(|line| highlighter.apply(line.as_str()))
        .collect::<Vec<_>>()
        .join("\n");

    writer.write_line(&highlighted).await?;

    Ok(())
}
