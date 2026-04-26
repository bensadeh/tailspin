#![forbid(unsafe_code)]

mod cli;
mod config;
mod highlighter_builder;
mod io;
mod theme;

use cli::{FullConfig, get_config};
use io::initial_read::{InitialReadCompleteSender, initial_read_complete_channel};
use io::presenter::pager::Pager;
use io::presenter::{Present, Presenter};
use io::reader::{AsyncLineReader, Reader, StreamEvent};
use io::setup::initialize_io;
use io::writer::stdout::BrokenPipe;
use io::writer::{AsyncLineWriter, Writer};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use tailspin::Highlighter;
use tokio::task::JoinHandle;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let FullConfig {
        source,
        target,
        highlighter,
    } = get_config()?;
    let (io, _temp_dir) = initialize_io(source, target).await?;

    let (initial_read_tx, initial_read_rx) = initial_read_complete_channel();

    let process_stream_task = tokio::spawn(process_stream(io.reader, io.writer, highlighter, initial_read_tx));

    if initial_read_rx.receive().await.is_err() {
        return process_stream_task.await?;
    }

    match io.presenter {
        Presenter::StdOut(_) => BrokenPipe::suppress(process_stream_task.await?),
        Presenter::Pager(pager) => race_pager_against_stream(pager, process_stream_task).await,
    }
}

async fn race_pager_against_stream(
    pager: Pager,
    mut process_stream_task: JoinHandle<anyhow::Result<()>>,
) -> anyhow::Result<()> {
    let mut pager_task = tokio::spawn(async move { pager.present().await });

    tokio::select! {
        pager_result = &mut pager_task => {
            abort_and_drain(&mut process_stream_task).await;
            pager_result?
        }
        stream_result = &mut process_stream_task => match stream_result? {
            Ok(()) => pager_task.await?,
            Err(e) => {
                abort_and_drain(&mut pager_task).await;
                Err(e)
            }
        }
    }
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
) -> anyhow::Result<()> {
    loop {
        match reader.next().await? {
            StreamEvent::Started => initial_read_complete.send()?,
            StreamEvent::Ended => return Ok(()),
            StreamEvent::Line(line) => write_line(&mut writer, &highlighter, line.as_str()).await?,
            StreamEvent::Lines(lines) => write_lines(&mut writer, &highlighter, lines).await?,
        }
    }
}

async fn write_line(writer: &mut Writer, highlighter: &Highlighter, line: &str) -> anyhow::Result<()> {
    let highlighted = highlighter.apply(line);
    writer.write(&highlighted).await
}

async fn write_lines(writer: &mut Writer, highlighter: &Highlighter, lines: Vec<String>) -> anyhow::Result<()> {
    let highlighted = lines
        .par_iter()
        .map(|line| highlighter.apply(line.as_str()))
        .collect::<Vec<_>>()
        .join("\n");

    writer.write(&highlighted).await
}
