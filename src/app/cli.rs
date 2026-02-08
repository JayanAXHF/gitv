use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    pub args: Args,
}

#[derive(clap::Args, Clone)]
pub struct Args {
    pub owner: String,
    pub repo: String,
}
