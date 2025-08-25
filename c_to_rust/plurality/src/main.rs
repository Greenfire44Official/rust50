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

impl fmt::Display for Candidate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use the write! macro to write the desired output to the formatter 'f'.
        // This macro works similarly to println!, but writes to the provided buffer.
        write!(f, "{} with {} votes", self.name, self.votes)
    }
}

fn main() -> Result<()> {
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        bail!("Invalid input\n\nUsage: {} [canididates]\n", args[0]);
    }
    if args.len() > MAX + 1 {
        bail!("Too many candidates! Maximum number of cadidates: {MAX}\n");
    }
    let mut candidates: Vec<Candidate> = Vec::new();
    for candidate in &args[1..args.len()] {
        candidates.push(Candidate {
            name: candidate.to_owned(),
            votes: 0,
        });
    }
    let votes: usize = loop {
        let input: usize = get_input("Number of voters: ");
        if input <= MAX_VOTERS {
            break input;
        }
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
