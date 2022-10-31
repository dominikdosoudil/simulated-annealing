use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[arg(short, long, default_value_t = 300)]
    pub(crate) tries_max: u32,

    #[arg(short, long, default_value_t = 10)]
    pub(crate) flips_max: u32,

    #[arg(short, long)]
    pub(crate) input: String,
}
