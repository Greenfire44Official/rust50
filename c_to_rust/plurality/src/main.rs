use std::env::args;

use anyhow::{Result, bail};
use my_library::{get_input, get_string};

const MAX: usize = 9;

#[derive(Clone)]
struct Candidate {
    name: String,
    votes: u64,
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
    let votes: u64 = get_input("Number of voters: ");
    for _ in 1..=votes {
        let index = loop {
            let vote = get_string("Vote: ");
            match candidates.iter().position(|mut c| c.name == vote) {
                Some(mut i) => {
                    break i;
                }
                None => {
                    continue;
                }
            }
        };
        candidates[index].votes += 1;
    }
    let mut winners: Vec<Candidate> = vec![candidates[0].clone()];
    for candidate in candidates {
        match candidate.votes {
            x if x > winners[0].votes => {
                winners.clear();
                winners.push(candidate);
            }
            x if x == winners[0].votes &&  candidate.name != winners[0].name => winners.push(candidate),
            _ => {}
        }
    }
    for winner in winners {
        println!("{}", winner.name);
    }
    Ok(())
}
