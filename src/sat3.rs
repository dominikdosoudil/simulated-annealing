use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use rand::Rng;

#[derive(Debug)]
pub(crate) struct Formula {
    pub(crate) vars_n: u32,
    pub(crate) weights: Vec<u32>,
    pub(crate) clauses: Vec<Clause3>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Clause3 {
    pub(crate) a: i64,
    pub(crate) b: i64,
    pub(crate) c: i64,
}

impl FromStr for Clause3 {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.split_whitespace();

        Ok(Clause3 {
            a: i64::from_str(chunks.next().expect("to be valid int"))?,
            b: i64::from_str(chunks.next().expect("to be valid int"))?,
            c: i64::from_str(chunks.next().expect("to be valid int"))?,
        })
    }
}

impl FromStr for Formula {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let mut vars_n = None;
        let mut clauses: Vec<Clause3> = vec![];
        let mut weights: Vec<u32> = vec![];
        for line in lines {
            if line.starts_with("c") {
                continue;
            }
            if line.starts_with("w") {
                weights = line
                    .split_whitespace()
                    .skip(1)
                    .map(u32::from_str)
                    .collect::<Result<Vec<u32>, ParseIntError>>()
                    .expect("to parse all weights correctly")
                    .into_iter()
                    .filter(|x| x > &0u32)
                    .collect::<Vec<u32>>();
                continue;
            }
            if line.starts_with("p") {
                vars_n = Some(
                    u32::from_str(
                        line.split_whitespace()
                            .skip(2)
                            .next()
                            .expect("vars_n be there"),
                    )
                    .expect("to parse vars_n"),
                );
                continue;
            }
            clauses.push(Clause3::from_str(line)?)
        }

        Ok(Formula {
            vars_n: vars_n.unwrap_or_default(),
            weights,
            clauses,
        })
    }
}
#[derive(Debug, Clone)]
pub(crate) struct TruthAssignment {
    pub(crate) assignments: Vec<bool>,
}

impl Display for TruthAssignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.assignments
                .iter()
                .enumerate()
                .map(|(index, assignment)| format!(
                    "{}{}",
                    if *assignment { "" } else { "-" },
                    index + 1
                ))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

fn negate_if_negation(variable: i64, value: bool) -> bool {
    if variable.is_negative() {
        return !value;
    }
    return value;
}

impl TruthAssignment {
    pub(crate) fn satisfies(&self, clause: &Clause3) -> Option<bool> {
        match (
            self.assignments.get((i64::abs(clause.a) - 1) as usize),
            self.assignments.get((i64::abs(clause.b) - 1) as usize),
            self.assignments.get((i64::abs(clause.c) - 1) as usize),
        ) {
            (Some(a), Some(b), Some(c)) => Some(
                negate_if_negation(clause.a, *a)
                    || negate_if_negation(clause.b, *b)
                    || negate_if_negation(clause.c, *c),
            ),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn satisfies_formula(&self, formula: &Formula) -> Option<bool> {
        formula
            .clauses
            .iter()
            .map(|clause| self.satisfies(&clause))
            .collect::<Option<Vec<bool>>>()
            .map(|v| v.iter().all(|clause_satisfied| *clause_satisfied))
    }

    pub(crate) fn satisfied_clauses<'a>(
        &self,
        clauses: impl Iterator<Item = &'a Clause3>,
    ) -> Vec<&'a Clause3> {
        clauses
            .filter(|clause| self.satisfies(&clause).unwrap())
            .collect() // todo: might be optimized
    }

    pub(crate) fn new_random(vars_n: u32) -> TruthAssignment {
        let mut rng = rand::thread_rng();

        TruthAssignment {
            assignments: (0..vars_n).map(|_| rng.gen()).collect(),
        }
    }

    pub(crate) fn flip(&mut self, variable: i64) {
        let index = (i64::abs(variable) - 1) as usize;
        let val = self
            .assignments
            .get(index)
            .expect("variable not out of bounds");
        self.assignments[index] = !(*val);
    }
}

impl From<Vec<bool>> for TruthAssignment {
    fn from(assignments: Vec<bool>) -> Self {
        TruthAssignment { assignments }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_clause_satisfaction() {
        let f = Formula {
            vars_n: 5,
            weights: vec![1, 1, 1, 1, 1],
            clauses: vec![
                Clause3::from_str("-1 2 3").unwrap(),
                Clause3::from_str("2 4 5").unwrap(),
            ],
        };
        assert_eq!(
            TruthAssignment::from(vec![true, true, false, false, true]).satisfies_formula(&f),
            Some(true)
        );
        assert_eq!(
            TruthAssignment::from(vec![true, false, false, false, true]).satisfies_formula(&f),
            Some(false)
        );
    }

    #[test]
    fn filter_satisfied_clauses() {
        let f = Formula {
            vars_n: 5,
            weights: vec![1, 1, 1, 1, 1],
            clauses: vec![
                Clause3::from_str("-1 2 3").unwrap(),
                Clause3::from_str("2 4 5").unwrap(),
            ],
        };

        println!(
            "{:#?}",
            TruthAssignment::from(vec![true, false, false, false, true])
                .satisfied_clauses(f.clauses.iter())
        )
    }
}
