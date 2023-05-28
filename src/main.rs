use linemux::MuxedLines;
use regex::Regex;
use std::collections::VecDeque;
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
    let yellow_color = "\x1b[33m"; // Yellow
    let reset_color = "\x1b[0m";

    let re_string = Regex::new(r#"'[^']*'|"[^"]*""#).unwrap();
    let re_color_reset = Regex::new(r"\x1b\[0m").unwrap();

    // Check if there's an uneven amount of quotes
    if re_string.find_iter(input).count() % 2 != 0 {
        return input.to_owned();
    }

    let mut result = String::new();
    let mut start = 0;
    for mat in re_string.find_iter(input) {
        result.push_str(&input[start..mat.start()]); // Before the match
        let string = &input[mat.start()..mat.end()]; // The matched string

        // Apply yellow color to string part
        result.push_str(yellow_color);

        // If the string part contains a color reset, replace it with itself plus yellow color
        let string = if re_color_reset.is_match(string) {
            re_color_reset
                .replace_all(string, format!("{}{}", reset_color, yellow_color).as_str())
                .to_string()
        } else {
            string.to_string()
        };

        result.push_str(&string);
        result.push_str(reset_color);

        start = mat.end(); // Continue after the match
    }
    result.push_str(&input[start..]); // After the last match

    result
}
