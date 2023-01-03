use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub(crate) enum TailCutMethod {
    RelativeDeviation,
    RelativeChange,
}

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[arg(short, long, default_value_t = 300)]
    pub(crate) tries_max: u32,

    #[arg(short, long, default_value_t = 10)]
    pub(crate) flips_max: u32,

    #[arg(short, long)]
    pub(crate) input: String,

    #[arg(short, long, default_value_t = 0.995)]
    pub(crate) cooling_ratio: f64,

    #[arg(short, long, default_value_t = 5.)]
    pub(crate) min_temperature: f64,

    #[arg(short, long, default_value_t = 4)]
    pub(crate) penalty_multiplier: i64,

    #[arg(short, long, default_value_t = 2500)]
    pub(crate) tail_cut_length: usize,

    #[arg(short, long, value_enum, default_value_t = TailCutMethod::RelativeDeviation)]
    pub(crate) tail_cut_method: TailCutMethod,
}
