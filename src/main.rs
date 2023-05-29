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
        let highlighted_string2 = highlight_quotes(highlighted_string.as_str());

        writeln!(output_writer, "{}", highlighted_string2)?;
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

fn highlight_quotes(input: &str) -> String {
    let quote_count: usize = input.chars().filter(|&ch| ch == '"').count();
    if quote_count % 2 != 0 {
        return input.to_string();
    }

    let mut output = String::new();
    let mut inside_quote = false;
    let mut potential_color_code = String::new();

    let yellow = "\x1b[33m";
    let reset = "\x1b[0m";

    for ch in input.chars() {
        if ch == '"' {
            inside_quote = !inside_quote;
            if inside_quote {
                output.push_str(yellow);
                output.push(ch);
            } else {
                output.push(ch);
                output.push_str(reset);
            }
            continue;
        }

        if inside_quote {
            potential_color_code.push(ch);

            if potential_color_code == reset {
                output.push_str(&potential_color_code);
                output.push_str(yellow);
                potential_color_code.clear();
            } else if !reset.starts_with(&potential_color_code) {
                output.push_str(&potential_color_code);
                potential_color_code.clear();
            }
        } else {
            output.push(ch);
        }
    }

    output
}
