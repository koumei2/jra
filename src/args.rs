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

//SUBCOMMANDS の説明を追加
// 中止の時対応 reqct
// ２着同着時の対応 react
