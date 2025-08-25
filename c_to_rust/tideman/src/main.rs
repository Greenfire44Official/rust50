use std::{env::args, fmt};

use anyhow::{Error, Result, bail};
use my_library::{get_input, get_string};

const MAX: usize = 9;
const MAX_VOTERS: usize = 100;

#[derive(Clone, Debug)]
struct Candidate {
    name: String,
    votes: u64,
    eliminated: bool,
}

impl fmt::Display for Candidate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use the write! macro to write the desired output to the formatter 'f'.
        // This macro works similarly to println!, but writes to the provided buffer.
        let mut r = write!(f, "{} with {} votes", self.name, self.votes);
        if self.eliminated {
            r = write!(f, " (eliminated)")
        }
        r
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
            eliminated: false,
        });
    }
    let votes: usize = loop {
        let input: usize = get_input("Number of voters: ");
        if input <= MAX_VOTERS {
            break input;
        }
    };

    // Usage: voter_preferences[voter][candidate index rank]
    let mut voter_preferences: Vec<Vec<usize>> = vec![vec![0; candidates.len()]; votes];

    for voter in 0..votes {
        println!("Voter {}", voter + 1);
        match register_ranked_vote(voter, &mut candidates, &mut voter_preferences) {
            Err(e) => bail!(e),
            _ => {}
        };
    }

    let winners: Vec<Candidate> = determine_winners(&mut candidates, &voter_preferences);
    for winner in winners {
        println!("{}", winner);
    }

    /* DEBUG */
    println!("{:?}", voter_preferences);
    for candidate in candidates {
        println!("{}", candidate);
    }
    /* DEBUG */

    Ok(())
}

fn register_ranked_vote(
    voter: usize,
    candidates: &mut Vec<Candidate>,
    voter_preferences: &mut Vec<Vec<usize>>,
) -> Result<(), Error> {
    for rank in 0..candidates.len() {
        let index = loop {
            print!("Rank {}: ", rank + 1);
            let vote = get_string(&(""));
            match candidates.iter().position(|c| c.name == vote) {
                Some(i) => {
                    break i;
                }
                None => {
                    bail!("Invalid vote.")
                }
            }
        };
        voter_preferences[voter][rank] = index;
    }
    Ok(())
}

fn determine_winners(
    candidates: &mut Vec<Candidate>,
    voter_preferences: &Vec<Vec<usize>>,
) -> Vec<Candidate> {
    // Register votes.
    for voter in voter_preferences {
        for (rank, candidate_index) in voter.iter().enumerate() {
            if !candidates[*candidate_index].eliminated {
                candidates[*candidate_index].votes += (candidates.len() - rank) as u64;
            }
        }
    }

    let mut winners: Vec<Candidate> = vec![candidates[0].clone()];
    for candidate in candidates {
        println!("Checking candidate: {candidate}");
        println!("{:?}", winners);
        if candidate.name == winners[0].name {
            continue;
        }
        if candidate.votes > winners[0].votes {
            println!("New highest votes encountered, clearing...");
            winners.clear();
            winners.push(candidate.clone());
        } else if candidate.votes == winners[0].votes {
            winners.push(candidate.clone());
        }
    }

    winners
}
