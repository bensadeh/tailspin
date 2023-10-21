mod cli;
mod color;
mod config;
mod highlight_processor;
mod highlight_utils;
mod highlighters;
mod io;
mod line_info;
mod regex;
mod theme;
mod theme_io;
mod types;

use crate::highlight_processor::HighlightProcessor;
use crate::io::controller::get_io_and_presenter;
use crate::io::presenter::Present;
use crate::io::reader::AsyncLineReader;
use crate::io::writer::AsyncLineWriter;
use crate::theme::Theme;
use crate::types::Config;
use color_eyre::eyre::Result;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = cli::get_args_or_exit_early();
    let theme = theme_io::load_theme(args.config_path.clone());
    let config = config::create_config_or_exit_early(args);

    run(theme, config).await;

    Ok(())
}

pub async fn run(theme: Theme, config: Config) {
    let (reached_eof_tx, reached_eof_rx) = oneshot::channel::<()>();
    let (io, presenter) = get_io_and_presenter(config, Some(reached_eof_tx)).await;

    let highlighter = highlighters::Highlighters::new(&theme.groups);
    let highlight_processor = HighlightProcessor::new(highlighter);

    tokio::spawn(process_lines(io, highlight_processor));

    reached_eof_rx
        .await
        .expect("Could not receive EOF signal from oneshot channel");

    presenter.present();
}

async fn process_lines<T: AsyncLineReader + AsyncLineWriter + Unpin + Send>(
    mut io: T,
    highlight_processor: HighlightProcessor,
) {
    while let Ok(Some(line)) = io.next_line().await {
        let highlighted_line = highlight_processor.apply(&line);
        io.write_line(&highlighted_line).await.unwrap();
    }
}
