extern crate core;

mod args;
mod sat3;
mod visualisation;

use crate::args::{Args, TailCutMethod};
use crate::sat3::{Formula, TruthAssignment};
use clap::Parser;
use rand::{/*rngs,*/ thread_rng, Rng};
use std::fs;
use std::path::Path;
use std::str::FromStr;

const EQUILIBRIUM: u64 = 100;
const INIT_HEAT_RATIO: f64 = 10.;

const DEBUG: bool = false;

macro_rules! if_debug {
    ($e:expr) => {
        if DEBUG {
            $e
        }
    };
}

fn frozen_factory(min_temperature: f64) -> impl Fn(f64) -> bool {
    move |t: f64| t <= min_temperature
}

/// aka try
fn next_state(random_generator: &mut impl Rng, state: TruthAssignment) -> TruthAssignment {
    let index: i64 = random_generator.gen_range(1..(state.assignments.len() + 1)) as i64;
    let mut next_state: TruthAssignment = state.clone();
    next_state.flip(index);
    next_state
}

fn fridge_factory(cooling_ratio: f64) -> impl Fn(f64) -> f64 {
    move |t: f64| t * cooling_ratio
}

fn value_calculator_factory(penalty_multiplier: i64) -> impl Fn(&TruthAssignment, &Formula) -> i64 {
    return move |state: &TruthAssignment, formula: &Formula| {
        let truthy_variable_weight_sum: i64 = state
            .assignments
            .iter()
            .enumerate()
            .map(|(index, assignment)| {
                if *assignment {
                    *formula
                        .weights
                        .get(index)
                        .expect("variable weight be present") as i64
                } else {
                    0 as i64
                }
            })
            .sum();
        let unsatisfied_clauses_len =
            (formula.clauses.len() - state.satisfied_clauses(formula.clauses.iter()).len()) as i64;

        truthy_variable_weight_sum - (penalty_multiplier * unsatisfied_clauses_len)
    };
}

fn compute_initial_temperature(
    rng: &mut impl Rng,
    f: &Formula,
    value_calculator: impl Fn(&TruthAssignment, &Formula) -> i64,
) -> f64 {
    let mut t = 1_f64;
    loop {
        let mut acceptance_ratios: Vec<f64> = vec![];
        while acceptance_ratios.len() < 10 {
            let state = TruthAssignment::new_random(f.vars_n, rng);
            let state_value = value_calculator(&state, &f);
            let next_state_value = value_calculator(&next_state(rng, state.clone()), &f);
            if next_state_value >= state_value {
                continue;
            }
            acceptance_ratios.push(acceptance_ratio((state_value - next_state_value) as f64, t));
        }
        if acceptance_ratios.iter().sum::<f64>() / acceptance_ratios.len() as f64 > 0.5 {
            break;
        }
        t += INIT_HEAT_RATIO;
    }
    t
}

fn acceptance_ratio(value_decrease: f64, t: f64) -> f64 {
    (-1.0 * value_decrease / t).exp()
}

fn accept(rng: &mut impl Rng, value_decrease: f64, t: f64) -> bool {
    rng.gen_range(0.0..1.0) < acceptance_ratio(value_decrease, t)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut rng = thread_rng();
    // let mut rng = rngs::StdRng::seed_from_u64(0);
    let instance_serialized = fs::read_to_string(&args.input).expect("reading input file ok");
    let f = Formula::from_str(&instance_serialized).expect("parse formula");

    let avg_weight = (f.weights.iter().sum::<u32>() / f.weights.len() as u32) as i64;
    let value = value_calculator_factory(args.penalty_multiplier * avg_weight);
    let cool_down = fridge_factory(args.cooling_ratio);
    let mut t = compute_initial_temperature(&mut rng, &f, &value);
    let frozen = frozen_factory(t / 10_f64.powi(args.min_temperature));
    let mut state = TruthAssignment::new_random(f.vars_n, &mut rng);
    let mut best = state.clone();
    let mut value_history: Vec<f32> = vec![];
    let mut deviation_history: Vec<f32> = vec![];

    // println!("inital temp: {}", t);

    // println!("Starting SA");
    let mut lifes = args.tail_cut_length;
    while !frozen(t) && lifes > 0 {
        for _ in 0..EQUILIBRIUM {
            if_debug!(print!("{} ", value(&state, &f)));
            let state_value = value(&state, &f);
            value_history.push(state_value as f32);
            let new_state = next_state(&mut rng, state.clone());
            let new_state_value = value(&new_state, &f);

            if args.tail_cut_method == TailCutMethod::RelativeChange {
                let relative_change =
                    (new_state_value as f64 - state_value as f64) / state_value as f64;
                // println!(
                //     "values: {} | {} ",
                //     new_state_value as f64, state_value as f64
                // );
                // println!("value change: {}", relative_change);
                if (relative_change as f64).abs() < 1. {
                    lifes -= 1;
                } else {
                    lifes = args.tail_cut_length;
                }
            }

            if new_state_value > state_value {
                state = new_state;
            } else if accept(&mut rng, (state_value - new_state_value) as f64, t) {
                state = new_state;
            }
            let state_value = value(&state, &f);
            let best_value = value(&best, &f);
            if state_value > best_value {
                best = state.clone();
            }

            if args.tail_cut_method == TailCutMethod::RelativeDeviation {
                let n = args.tail_cut_length;
                let last_values = value_history.iter().skip(value_history.len() - n);
                let last_values_avg = last_values.clone().sum::<f32>() / n as f32;
                let std_deviation = (last_values
                    .map(|x| (*x - last_values_avg).powf(2.))
                    .sum::<f32>()
                    / (n - 1) as f32)
                    .sqrt();
                let relative_deviation = std_deviation / last_values_avg;
                deviation_history.push(std_deviation);
                if_debug!(println!(
                    "{} / {} = {}",
                    std_deviation, last_values_avg, relative_deviation,
                ));
                if last_values_avg > 0. && relative_deviation < 0.00001 {
                    break;
                }
            }
        }
        if_debug!(println!("\nEquilibrium. Cooling down."));
        t = cool_down(t);
    }
    // println!("{} {} {} 0", &args.input, value(&best, &f), best);
    let satisfied_clauses_n = best.satisfied_clauses(f.clauses.iter()).len();
    let clauses_n = f.clauses.len();
    println!("{} {}", value(&best, &f), satisfied_clauses_n);
    // println!(
    //     "Satisfied clauses: {} of {}",
    //     satisfied_clauses_n, clauses_n
    // );

    let file_name = Path::new(&args.input)
        .file_name()
        .expect("file exists")
        .to_str()
        .unwrap();
    let f_name = format!("{}.png", file_name);
    let plot_title = format!(
        "{}  satisfied: {}/{} [p_m={}]",
        &file_name, satisfied_clauses_n, clauses_n, args.penalty_multiplier
    );
    visualisation::draw_values(f_name.as_str(), &plot_title, &value_history)
    // visualisation::draw_values("plot2.png", &args.input, &deviation_history)
}
