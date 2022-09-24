use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    GradeRace {
        #[clap(default_value = "0")]
        year: u16,
    },
    Wakuban,
    Result,
}
