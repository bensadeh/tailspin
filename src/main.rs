use linemux::MuxedLines;
use regex::Regex;
use std::io;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;
use tempfile::NamedTempFile;

#[tokio::main]
async fn main() {
    let input = "golang/debug/1.log";

    let output_file = NamedTempFile::new().unwrap();
    let output_path = output_file.path().to_path_buf();
    let output_writer = BufWriter::new(output_file);

    tokio::spawn(async move {
        tail_file(input, output_writer)
            .await
            .expect("TODO: panic message");
    });

    open_file_with_less(output_path.to_str().unwrap());

    cleanup(output_path);
}

fn cleanup(output_path: PathBuf) {
    if let Err(err) = std::fs::remove_file(output_path) {
        eprintln!("Failed to remove the temporary file: {}", err);
    }
}

async fn tail_file<R>(path: &str, mut output_writer: BufWriter<R>) -> io::Result<()>
where
    R: Write + Send + 'static,
{
    let mut lines = MuxedLines::new()?;
    lines.add_file_from_start(path).await?;

    while let Ok(Some(line)) = lines.next_line().await {
        let highlighted_string = highlight_numbers_in_blue(line.line());

        writeln!(output_writer, "{}", highlighted_string)?;
        output_writer.flush()?;
    }

    Ok(())
}

fn open_file_with_less(path: &str) {
    let output = Command::new("less").arg(path).status();

    match output {
        Ok(status) => {
            if !status.success() {
                eprintln!("Failed to open file with less");
            }
        }
        Err(err) => {
            eprintln!("Failed to execute pager command: {}", err);
        }
    }
}

fn highlight_numbers_in_blue(input: &str) -> String {
    let number_regex = Regex::new(r"\b\d+\b").expect("Invalid regex pattern");

    let highlighted = number_regex.replace_all(input, |caps: &regex::Captures<'_>| {
        format!("\x1B[34m{}\x1B[0m", &caps[0])
    });

    highlighted.into_owned()
}
