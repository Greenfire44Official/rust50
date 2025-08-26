use std::{env::args, fmt};

use anyhow::{Result, bail};
use my_library::{get_input, get_string};

const MAX: usize = 9;
const MAX_VOTERS: usize = 100;

#[derive(Clone, Debug)]
struct Candidate {
    name: String,
    votes: u64,
}

impl Candidate {
    pub fn new(name: String) -> Candidate {
        Candidate {
            name: name,
            votes: 0,
        }
    }
}

impl fmt::Display for Candidate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn main() -> Result<()> {
    // Parse and validate arguments
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        bail!("Invalid input\n\nUsage: {} [candidate ...]\n", args[0]);
    }
    if args.len() > MAX + 1 {
        bail!("Too many candidates! Maximum number of candidates: {MAX}\n");
    }

    // Check for duplicate candidate names
    let mut seen = std::collections::HashSet::new();
    for candidate in &args[1..] {
        if !seen.insert(candidate) {
            bail!("Duplicate candidate detected: {}", candidate);
        }
    }
    let mut candidates: Vec<Candidate> = Vec::new();
    for candidate in &args[1..args.len()] {
        candidates.push(Candidate::new(candidate.to_owned()))
    }
    let votes: usize = loop {
        let input: usize = get_input("Number of voters: ");
        if input <= MAX_VOTERS {
            break input;
        }
        println!("Too many votes, max: {}", MAX_VOTERS);
    };

    for voter in 0..votes {
        println!("Voter {}", voter + 1);
        vote(&mut candidates)
    }

    let winners: Vec<Candidate> = determine_winners(&mut candidates);
    for winner in winners {
        println!("{}", winner.name);
    }
    Ok(())
}

fn vote(candidates: &mut Vec<Candidate>) {
    let index = loop {
        let vote = get_string(&("Vote: "));
        match candidates.iter().position(|c| c.name == vote) {
            Some(i) => {
                break i;
            }
            None => {
                println!("Invalid vote.")
            }
        }
    };
    candidates[index].votes += 1;
}

fn determine_winners(candidates: &mut Vec<Candidate>) -> Vec<Candidate> {
    let mut winners: Vec<Candidate> = vec![candidates[0].clone()];

    for candidate in candidates {
        if candidate.name == winners[0].name {
            continue;
        }
        if candidate.votes > winners[0].votes {
            winners.clear();
            winners.push(candidate.clone());
        } else if candidate.votes == winners[0].votes {
            winners.push(candidate.clone());
        }
    }

    winners
}
