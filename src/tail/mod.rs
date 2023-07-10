use linemux::MuxedLines;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
use std::path::Path;
use std::{io, process};

use crate::highlight_processor;
use tokio::sync::oneshot;

pub(crate) async fn tail_file<R>(
    path: &str,
    follow: bool,
    mut output_writer: BufWriter<R>,
    highlighter: highlight_processor::HighlightProcessor,
    mut reached_eof_tx: Option<oneshot::Sender<()>>,
) -> io::Result<()>
where
    R: Write + Send + 'static,
{
    let input_path = Path::new(&path);
    if !input_path.exists() {
        eprintln!(
            "Error: File '{}' does not exist",
            input_path.to_str().unwrap()
        );
        process::exit(1);
    }

    let line_count = count_lines(input_path, follow);

    let mut lines = MuxedLines::new()?;
    let mut current_line = 1;
    lines.add_file_from_start(path).await?;

    while let Ok(Some(line)) = lines.next_line().await {
        if current_line == line_count {
            if let Some(reached_eof) = reached_eof_tx.take() {
                reached_eof
                    .send(())
                    .expect("Failed sending EOF signal to oneshot channel");
            }
        }

        let highlighted_string = highlighter.apply(line.line());

        writeln!(output_writer, "{}", highlighted_string)?;
        output_writer.flush()?;
        current_line += 1;
    }

    Ok(())
}

fn count_lines<P: AsRef<Path>>(file_path: P, follow: bool) -> usize {
    if follow {
        return 1;
    }

    let file = File::open(file_path).expect("Could not open file");
    let reader = io::BufReader::new(file);

    reader.lines().count()
}
