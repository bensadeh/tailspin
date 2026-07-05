#![forbid(unsafe_code)]

mod cli;
mod io;
mod theme;

use cli::{FullConfig, get_config};
use io::presenter::Presenter;
use io::presenter::pager::Pager;
use io::reader::{LineBatch, Reader, StreamEvent};
use io::setup::{IoSetup, initialize_io};
use io::writer::Writer;
use io::writer::stdout::BrokenPipe;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use shared_child::SharedChild;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::mpsc;
use std::thread;
use tailspin::Highlighter;

enum Event {
    Stream(anyhow::Result<()>),
    Pager(anyhow::Result<()>),
}

fn main() -> anyhow::Result<()> {
    let FullConfig {
        source,
        target,
        highlighter,
    } = get_config()?;
    let IoSetup {
        reader,
        writer,
        presenter,
    } = initialize_io(source, target)?;

    match presenter {
        Presenter::StdOut => run_to_stdout(reader, writer, &highlighter),
        Presenter::Pager(pager) => run_with_pager(reader, writer, highlighter, pager),
    }
}

fn run_to_stdout(reader: Reader, writer: Writer, highlighter: &Highlighter) -> anyhow::Result<()> {
    let (initial_read_tx, _) = mpsc::channel();

    BrokenPipe::suppress(process_stream(reader, writer, highlighter, initial_read_tx))
}

/// Runs the stream on its own thread while the pager runs as a child process;
/// whichever finishes first decides what happens to the other.
fn run_with_pager(reader: Reader, writer: Writer, highlighter: Highlighter, pager: Pager) -> anyhow::Result<()> {
    let exec_child = reader.exec_child();
    let (initial_read_tx, initial_read_rx) = mpsc::channel();
    let (events_tx, events) = mpsc::channel();

    let stream_tx = events_tx.clone();
    thread::spawn(move || {
        // A panic must still produce an event, or the recv loop blocks forever
        let result = catch_unwind(AssertUnwindSafe(|| {
            process_stream(reader, writer, &highlighter, initial_read_tx)
        }))
        .unwrap_or_else(|_| Err(anyhow::anyhow!("stream thread panicked")));
        let _ = stream_tx.send(Event::Stream(result));
    });

    if initial_read_rx.recv().is_err() {
        let Event::Stream(result) = events.recv()? else {
            unreachable!("the pager is not spawned yet")
        };
        return result;
    }

    let pager_child = match pager.spawn() {
        Ok(pager_child) => pager_child,
        Err(e) => {
            kill_exec_child(exec_child.as_deref());
            return Err(e);
        }
    };
    let waiter = pager_child.waiter();
    thread::spawn(move || {
        let _ = events_tx.send(Event::Pager(waiter.wait()));
    });

    loop {
        match events.recv()? {
            Event::Pager(result) => {
                // The stream thread dies with the process; an --exec child does not
                kill_exec_child(exec_child.as_deref());
                return result;
            }
            Event::Stream(Err(e)) => {
                pager_child.kill();
                return Err(e);
            }
            Event::Stream(Ok(())) => {}
        }
    }
}

fn kill_exec_child(exec_child: Option<&SharedChild>) {
    if let Some(child) = exec_child {
        let _ = child.kill();
    }
}

fn process_stream(
    mut reader: Reader,
    mut writer: Writer,
    highlighter: &Highlighter,
    initial_read_tx: mpsc::Sender<()>,
) -> anyhow::Result<()> {
    loop {
        match reader.next()? {
            // A dropped receiver is fine: the stdout path has no pager to gate
            StreamEvent::InitialReadComplete => {
                let _ = initial_read_tx.send(());
            }
            StreamEvent::Ended => return Ok(()),
            StreamEvent::Lines(batch) => write_lines(&mut writer, highlighter, &batch)?,
        }
    }
}

fn write_lines(writer: &mut Writer, highlighter: &Highlighter, batch: &LineBatch) -> anyhow::Result<()> {
    let highlighted: Vec<String> = batch
        .lines
        .par_iter()
        .map(|range| {
            let line = String::from_utf8_lossy(&batch.buf[range.clone()]);
            highlighter.apply(&line).into_owned()
        })
        .collect();

    writer.write_batch(highlighted.iter().map(AsRef::as_ref))
}
