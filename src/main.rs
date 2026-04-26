#![forbid(unsafe_code)]

mod cli;
mod config;
mod highlighter_builder;
mod io;
mod theme;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if let Err(err) = io::run().await {
        eprintln!("{} {err}", nu_ansi_term::Color::Red.paint("Error:"));
        std::process::exit(1);
    }
}
