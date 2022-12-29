extern crate core;

mod args;
mod sat3;
mod visualisation;

use crate::args::Args;
use crate::sat3::{Formula, TruthAssignment};
use clap::Parser;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::fs;
use std::str::FromStr;

const INITIAL_TEMP: f64 = 500.;
const MIN_TEMP: f64 = 5.;
const EQUILIBRIUM: u64 = 100;
const COOL_RATIO: f64 = 0.995;

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
fn next_state(random_generator: &mut ThreadRng, state: TruthAssignment) -> TruthAssignment {
    let index: i64 = random_generator.gen_range(1..(state.assignments.len() + 1)) as i64;
    let mut next_state: TruthAssignment = state.clone();
    next_state.flip(index);
    next_state
}

fn cool_down(t: f64) -> f64 {
    t * COOL_RATIO
}

fn value(state: &TruthAssignment, formula: &Formula) -> i64 {
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

    truthy_variable_weight_sum - (1000i64 * unsatisfied_clauses_len)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut rng = thread_rng();
    let instance_serialized = fs::read_to_string(&args.input).expect("reading input file ok");
    let f = Formula::from_str(&instance_serialized).expect("parse formula");

    let mut t = INITIAL_TEMP;
    let mut state = TruthAssignment::new_random(f.vars_n);
    let mut best = state.clone();
    let mut value_history: Vec<f32> = vec![];

    println!("Starting SA");
    while !frozen(t) {
        for _ in 0..EQUILIBRIUM {
            if_debug!(print!("{} ", value(&state, &f)));
            value_history.push(value(&state, &f) as f32);
            let new_state = next_state(&mut rng, state.clone());
            let state_value = value(&state, &f);
            let new_state_value = value(&new_state, &f);
            if new_state_value > state_value {
                state = new_state;
            } else if rng.gen_range(0.0..1.0)
                < (-1.0 * ((state_value - new_state_value) as f64) / t).exp()
            {
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

    visualisation::draw_values(&value_history)
}
