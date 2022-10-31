extern crate core;

mod args;
mod sat3;

use crate::args::Args;
use crate::sat3::{Clause3, Formula, TruthAssignment};
use clap::Parser;
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::str::FromStr;

const EPS: f64 = 0.0001;
const C_B: f64 = 2.3;

fn main() {
    let args = Args::parse();

    let instance_serialized = fs::read_to_string(args.input).expect("reading input file ok");
    let f = Formula::from_str(&instance_serialized).expect("parse formula");

    let mut rng = thread_rng();
    for try_i in 0..args.tries_max {
        let mut t = TruthAssignment::new_random(f.vars_n);

        for flip_i in 0..args.flips_max {
            if t.satisfies_formula(&f)
                .expect("formula does not contain variable not present in truthy assignment")
            {
                println!(
                    "{} {} 0 0",
                    flip_i + try_i * args.flips_max,
                    args.tries_max * args.flips_max // t
                );
                return;
            }
            // println!(
            //     "Satisfied clauses: {}/{}",
            //     t.satisfied_clauses(f.clauses.iter()).len(),
            //     f.clauses.len()
            // );

            let mut unsatisfied_clause: Option<&Clause3> = None;
            while unsatisfied_clause.is_none() {
                let random_clause = f.clauses.choose(&mut rng).unwrap();
                if !t
                    .satisfies(random_clause)
                    .expect("clause does not contain variable not present in truth assignment")
                {
                    unsatisfied_clause = Some(&random_clause);
                }
            }
            let weighted_distribution = WeightedIndex::new(vec![
                prob(unsatisfied_clause.unwrap().a, &f, &mut t),
                prob(unsatisfied_clause.unwrap().b, &f, &mut t),
                prob(unsatisfied_clause.unwrap().c, &f, &mut t),
            ])
            .unwrap();
            // println!("{:?}", weights);
            t.flip(match weighted_distribution.sample(&mut rng) {
                0 => unsatisfied_clause.unwrap().a,
                1 => unsatisfied_clause.unwrap().b,
                2 => unsatisfied_clause.unwrap().c,
                _ => panic!("case may not happen"),
            });
            // println!("unsatisfied clause : {:#?}", unsatisfied_clause);
        }
    }
    println!(
        "{} {} 0 0",
        args.tries_max * args.flips_max,
        args.tries_max * args.flips_max,
    );
}

fn prob(x: i64, f: &Formula, t: &mut TruthAssignment) -> f64 {
    let currently_satisfied = t.satisfied_clauses(f.clauses.iter());
    t.flip(x);
    let broken =
        currently_satisfied.len() - t.satisfied_clauses(currently_satisfied.into_iter()).len();
    t.flip(x);
    // println!("ceased: {}", broken);
    // let fixed = usize::max(0, possibly_satisfied - currently_satisfied);

    return 1. / (EPS + (broken as f64)).powf(C_B);
}

#[cfg(test)]
mod test {
    use crate::{prob, Clause3, Formula, TruthAssignment};
    use std::str::FromStr;

    #[test]
    fn compute_var_flip_weight() {
        let f = Formula {
            vars_n: 5,
            clauses: vec![
                Clause3::from_str("-1 2 3").unwrap(),
                Clause3::from_str("2 4 5").unwrap(),
            ],
        };
        let mut t = TruthAssignment::from(vec![true, false, false, false, true]);

        println!("{}", prob(-1, &f, &mut t));
        println!("{}", prob(5, &f, &mut t));
    }
}
