use linemux::MuxedLines;
use std::io::{BufRead, BufWriter, Write};
use std::process::Command;
use std::{fs::File, io};
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

    // ... Other parts of your program ...
    // sleep for 3 seconds
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Wait for the tail_file task to complete
    // let _ = tail_file_task.await;
    open_file_with_less(output_path.to_str().unwrap());
}

async fn tail_file<R>(path: &str, mut output_writer: BufWriter<R>) -> io::Result<()>
where
    R: Write + Send + 'static,
{
    let mut lines = MuxedLines::new()?;
    lines.add_file_from_start(path).await?;

    while let Ok(Some(line)) = lines.next_line().await {
        writeln!(output_writer, "{}", line.line())?;
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
