use clap::Parser;

mod args;
mod common;
mod graderace;
mod result;
mod wakuban;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    match args.command {
        args::Command::GradeRace { year } => graderace::get(year).await,
        args::Command::Wakuban => wakuban::get().await,
        args::Command::Result => result::get().await,
    };
}
