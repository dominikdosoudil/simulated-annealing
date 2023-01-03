use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub(crate) enum TailCutMethod {
    RelativeDeviation,
    RelativeChange,
}

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[arg(short, long)]
    pub(crate) input: String,

    #[arg(short, long, default_value_t = 0.995)]
    pub(crate) cooling_ratio: f64,

    #[arg(short, long, default_value_t = 2)]
    pub(crate) min_temperature: i32,

    #[arg(short, long, default_value_t = 4)]
    pub(crate) penalty_multiplier: i64,

    #[arg(long, default_value_t = 3000)]
    pub(crate) tail_cut_length: usize,

    #[arg(long, value_enum, default_value_t = TailCutMethod::RelativeDeviation)]
    pub(crate) tail_cut_method: TailCutMethod,
}
