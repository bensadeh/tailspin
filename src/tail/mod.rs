use linemux::MuxedLines;
use std::io;
use std::io::{BufWriter, Write};

use crate::highlight_processor;
use tokio::sync::oneshot;

pub(crate) async fn tail_file<R>(
    path: &str,
    mut output_writer: BufWriter<R>,
    highlighter: highlight_processor::HighlightProcessor,
    line_count: usize,
    mut reached_eof_tx: Option<oneshot::Sender<()>>,
) -> io::Result<()>
where
    R: Write + Send + 'static,
{
    let mut lines = MuxedLines::new()?;
    let mut current_line = 1;
    lines.add_file_from_start(path).await?;

    while let Ok(Some(line)) = lines.next_line().await {
        if current_line == line_count {
            if let Some(tx) = reached_eof_tx.take() {
                tx.send(()).expect("Failed sending to oneshot channel");
            }
        }

        let highlighted_string = highlighter.apply(line.line());

        writeln!(output_writer, "{}", highlighted_string)?;
        output_writer.flush()?;
        current_line += 1;
    }

    Ok(())
}
