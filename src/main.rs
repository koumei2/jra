use clap::Parser;

mod args;
mod common;
mod result;
mod wakuban;

const JRA_URL: &str = "https://www.jra.go.jp/";
const JRA_ACCESS_URL: &str = "https://www.jra.go.jp/JRADB/accessD.html";

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    match args.command {
        args::Command::GradeRace => println!("grade race"),
        args::Command::Wakuban => wakuban::get().await,
        args::Command::Result => result::get().await,
    };
}
