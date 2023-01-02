extern crate core;

mod args;
mod sat3;
mod visualisation;

use crate::args::Args;
use crate::sat3::{Formula, TruthAssignment};
use clap::Parser;
use rand::rngs::ThreadRng;
use rand::{rngs, thread_rng, Rng, SeedableRng};
use std::fs;
use std::str::FromStr;

const INITIAL_TEMP: f64 = 50.;
const MIN_TEMP: f64 = 5.;
const EQUILIBRIUM: u64 = 100;
const COOL_RATIO: f64 = 0.995;
const INIT_HEAT_RATIO: f64 = 10.;

const DEBUG: bool = false;

macro_rules! if_debug {
    ($e:expr) => {
        if DEBUG {
            $e
        }
    };
}

fn frozen(t: f64) -> bool {
    t <= MIN_TEMP
}

/// aka try
fn next_state(random_generator: &mut impl Rng, state: TruthAssignment) -> TruthAssignment {
    let index: i64 = random_generator.gen_range(1..(state.assignments.len() + 1)) as i64;
    let mut next_state: TruthAssignment = state.clone();
    next_state.flip(index);
    next_state
}

fn cool_down(t: f64) -> f64 {
    t * COOL_RATIO
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

    let total_weight = (f.weights.iter().sum::<u32>() / f.weights.len() as u32) as i64;
    let value = value_calculator_factory(4_i64 * total_weight);
    let mut t = compute_initial_temperature(&mut rng, &f, &value);
    let mut state = TruthAssignment::new_random(f.vars_n, &mut rng);
    let mut best = state.clone();
    let mut value_history: Vec<f32> = vec![];

    println!("inital temp: {}", t);

    println!("Starting SA");
    while !frozen(t) {
        for _ in 0..EQUILIBRIUM {
            if_debug!(print!("{} ", value(&state, &f)));
            let state_value = value(&state, &f);
            value_history.push(state_value as f32);
            let new_state = next_state(&mut rng, state.clone());
            let new_state_value = value(&new_state, &f);
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
        }
        if_debug!(println!("\nEquilibrium. Cooling down."));
        t = cool_down(t);
    }
    println!("{} {} {} 0", &args.input, value(&best, &f), best);
    println!(
        "Satisfied clauses: {} of {}",
        best.satisfied_clauses(f.clauses.iter()).len(),
        f.clauses.len()
    );

    visualisation::draw_values(&value_history)
}
