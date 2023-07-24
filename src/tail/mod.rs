use crate::highlight_processor;

use linemux::MuxedLines;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
use std::path::Path;
use std::process;
use std::process::Stdio;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::oneshot::Sender;

pub async fn tail_command_output<R>(
    mut output_writer: BufWriter<R>,
    highlighter: highlight_processor::HighlightProcessor,
    mut reached_eof_tx: Option<Sender<()>>,
    command: &str,
) -> io::Result<()>
where
    R: Write + Send + 'static,
{
    send_eof_message(&mut reached_eof_tx);

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Could not capture standard output.")
        })?;

    let mut reader = BufReader::new(output).lines();

    loop {
        match reader.next_line().await {
            Ok(Some(line)) => {
                let highlighted_string = highlighter.apply(&line);
                writeln!(output_writer, "{}", highlighted_string)?;
                output_writer.flush()?;
            }
            Ok(None) => {}
            Err(err) => {
                eprintln!("Error reading from command output: {}", err);
                break;
            }
        }
    }

    Ok(())
}

pub async fn tail_stdin<R>(
    mut output_writer: BufWriter<R>,
    highlighter: highlight_processor::HighlightProcessor,
    follow: bool,
    mut reached_eof_tx: Option<Sender<()>>,
) -> io::Result<()>
where
    R: Write + Send + 'static,
{
    if follow {
        send_eof_message(&mut reached_eof_tx);
    }

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    loop {
        match reader.next_line().await {
            Ok(Some(line)) => {
                let highlighted_string = highlighter.apply(&line);
                writeln!(output_writer, "{}", highlighted_string)?;
                output_writer.flush()?;
            }
            Ok(None) => {
                if !follow {
                    send_eof_message(&mut reached_eof_tx);
                    break;
                }
            }
            Err(err) => {
                eprintln!("Error reading from stdin: {}", err);
                break;
            }
        }
    }

    Ok(())
}

fn send_eof_message(reached_eof_tx: &mut Option<Sender<()>>) {
    if let Some(reached_eof) = reached_eof_tx.take() {
        reached_eof
            .send(())
            .expect("Failed sending EOF signal to oneshot channel");
    }
}

pub(crate) async fn tail_file<R>(
    file_path: &str,
    follow: bool,
    mut output_writer: BufWriter<R>,
    highlighter: highlight_processor::HighlightProcessor,
    mut reached_eof_tx: Option<Sender<()>>,
) -> io::Result<()>
where
    R: Write + Send + 'static,
{
    let input_path = Path::new(&file_path);
    check_file_exists(input_path);

    let line_count = count_lines(input_path, follow);

    let mut lines = MuxedLines::new()?;
    let mut current_line = 1;
    lines.add_file_from_start(file_path).await?;

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

fn check_file_exists(path: &Path) {
    match path.try_exists() {
        Ok(true) => (),
        Ok(false) => {
            eprintln!("Error: File '{}' does not exist", path.to_str().unwrap());
            process::exit(1);
        }
        Err(err) => {
            eprintln!(
                "Error: Could not check if file '{}' exists: {}",
                path.to_str().unwrap(),
                err
            );
            process::exit(1);
        }
    }
}

fn count_lines<P: AsRef<Path>>(file_path: P, follow: bool) -> usize {
    if follow {
        return 1;
    }

    let file = File::open(file_path).expect("Could not open file");
    let reader = std::io::BufReader::new(file);

    reader.lines().count()
}
