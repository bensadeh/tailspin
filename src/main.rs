use linemux::MuxedLines;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut lines = MuxedLines::new()?;

    // Register some files to be tailed, whether they currently exist or not.
    lines.add_file_from_start("golang/debug/1.log").await?;

    // Wait for `Line` event, which contains the line captured for a given
    // source path.

    while let Ok(Some(line)) = lines.next_line().await {
        println!("{}", line.line());
    }

    Ok(())
}
