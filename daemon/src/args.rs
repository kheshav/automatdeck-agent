use clap::Parser;
use derive_getters::Getters;

#[derive(Parser, Debug, Getters)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Number of times to greet
    //#[clap(short, long, default_value_t = 1)]
    //count: u8,

   #[clap(subcommand)]
    subcmd: SubCommand,

}

#[derive(Parser,Debug)]
pub enum SubCommand {
    #[clap()]
    Diagnose(Diagnose),
    Launch(Launch),
}

/// Perform diagnosis
#[derive(Parser,Debug,Getters)]
pub struct Diagnose {
    /// Print debug info
    #[clap(long)]
    debug: bool,

   /// List new requests
   #[clap(long)]
   list_new_requests: bool,

}


/// Launch ad-agent to execute tasks
#[derive(Parser,Debug,Getters)]
pub struct Launch {
    /// Launch agent in debug mode
    #[clap(short,long)]
    debug: bool,

}

