use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    GradeRace,
    Wakuban,
    Result,
}
