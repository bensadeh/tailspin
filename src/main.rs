#![forbid(unsafe_code)]

use crate::initial_read::InitialReadCompleteSender;
use crate::io::controller::{Reader, Writer};
use io::controller::initialize_io;
use io::presenter::Present;
use io::reader::{AsyncLineReader, StreamEvent};
use io::writer::AsyncLineWriter;
use miette::{IntoDiagnostic, Result};
use rayon::prelude::*;
use tailspin::Highlighter;
use tokio::task::JoinHandle;

mod cli;
mod config;
mod highlighter_builder;
mod initial_read;
mod io;
mod theme;

#[tokio::main]
async fn main() -> Result<()> {
    let (reader, writer, presenter, highlighter, initial_read_complete_tx, initial_read_complete_rx, _temp_dir) =
        initialize_io().await?;

    let mut process_stream_task = tokio::spawn(process_stream(reader, writer, highlighter, initial_read_complete_tx));

    initial_read_complete_rx.receive().await?;

    let mut presenter_task = tokio::spawn(async move { presenter.present().await });

    tokio::select! {
        presenter_result = &mut presenter_task => {
            abort_and_drain(&mut process_stream_task).await;
            presenter_result.into_diagnostic()??;
        },
        process_stream_result = &mut process_stream_task => {
            abort_and_drain(&mut presenter_task).await;
            process_stream_result.into_diagnostic()??;
        },
    }

    Ok(())
}

async fn abort_and_drain<T>(handle: &mut JoinHandle<T>) {
    handle.abort();
    let _drain = handle.await;
}

async fn process_stream(
    mut reader: Reader,
    mut writer: Writer,
    highlighter: Highlighter,
    mut initial_read_complete: InitialReadCompleteSender,
) -> Result<()> {
    loop {
        match reader.next().await? {
            StreamEvent::Started => initial_read_complete.send()?,
            StreamEvent::Ended => return Ok(()),
            StreamEvent::Line(line) => write_line(&mut writer, &highlighter, line.as_str()).await?,
            StreamEvent::Lines(lines) => write_lines(&mut writer, &highlighter, lines).await?,
        }
    }
}

async fn write_line(writer: &mut Writer, highlighter: &Highlighter, line: &str) -> Result<()> {
    let highlighted = highlighter.apply(line);

    writer.write(&highlighted).await?;

    Ok(())
}

async fn write_lines(writer: &mut Writer, highlighter: &Highlighter, lines: Vec<String>) -> Result<()> {
    let highlighted = lines
        .par_iter()
        .map(|line| highlighter.apply(line.as_str()))
        .collect::<Vec<_>>()
        .join("\n");

    writer.write(&highlighted).await?;

    Ok(())
}
