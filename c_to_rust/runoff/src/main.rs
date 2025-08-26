use std::{cmp::Ordering, env::args, fmt};

use anyhow::{Error, Result, bail};
use my_library::{get_input, get_string};

const MAX: usize = 9;
const MAX_VOTERS: usize = 100;

#[derive(Clone, Debug)]
struct Candidate {
    name: String,
    votes: usize,
    eliminated: bool,
}

impl Candidate {
    pub fn new(name: String) -> Candidate {
        Candidate {
            name: name,
            votes: 0,
            eliminated: false,
        }
    }
}

impl fmt::Display for Candidate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = self.name.clone();
        if self.eliminated {
            display += " (eliminated)";
        }
        write!(f, "{}", display)
    }
}

fn main() -> Result<()> {
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        bail!("Invalid input\n\nUsage: {} [canididate ...]\n", args[0]);
    }
    if args.len() > MAX + 1 {
        bail!("Too many candidates! Maximum number of cadidates: {MAX}\n");
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

    // Usage: voter_preferences[voter][rank] -> candidate index
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
        println!("{}", winner.name);
    }

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

fn count_votes(candidates: &mut Vec<Candidate>, voter_preferences: &Vec<Vec<usize>>) {
    'vote: for voter in voter_preferences {
        let mut top_choice: usize = 0; // Default value for compiler, but we are skipping the vote if no top choice is found.
        for (index, candidate_index) in voter.iter().enumerate() {
            if candidates[*candidate_index].eliminated {
                if index == voter.len() - 1 {
                    /*
                    This should be impossible since we force voters to rank ALL candidates.
                    However, in a real runoff election voters could vote for as many candidates as they wish.
                    */
                    continue 'vote;
                }
                continue;
            }
            top_choice = *candidate_index;
            break;
        }
        candidates[top_choice].votes += 1;
    }
}

fn determine_winners(
    candidates: &mut Vec<Candidate>,
    voter_preferences: &Vec<Vec<usize>>,
) -> Vec<Candidate> {
    for _ in 0..=(candidates.len() * 2) {
        count_votes(candidates, voter_preferences);

        let mut winners: Vec<Candidate> = Vec::new();
        for candidate in candidates.iter() {
            if candidate.eliminated {
                continue;
            }

            if candidate.votes > (voter_preferences.len() / 2) {
                winners.push(candidate.clone());
            }
        }

        if winners.len() == 0 {
            match check_tie(candidates) {
                Some(winners) => return winners,
                None => {}
            }
            eliminate_lowest(candidates);
            continue;
        }

        return winners;
    }
    vec![]
}

fn check_tie(candidates: &mut Vec<Candidate>) -> Option<Vec<Candidate>> {
    let mut active: Vec<Candidate> = Vec::new();
    for candidate in candidates.iter() {
        if !candidate.eliminated {
            active.push(candidate.clone());
        }
    }
    let mut most_voted: Vec<Candidate> = Vec::new();
    for candidate in candidates.iter() {
        if candidate.eliminated {
            continue;
        }
        if most_voted.len() == 0 {
            most_voted.push(candidate.clone());
            continue;
        }
        match candidate.votes.cmp(&most_voted[0].votes) {
            Ordering::Greater => {
                most_voted.clear();
                most_voted.push(candidate.clone());
            }
            Ordering::Equal => most_voted.push(candidate.clone()),
            _ => {}
        }
    }

    if active.len() == most_voted.len() {
        return Some(most_voted);
    }
    None
}

fn eliminate_lowest(candidates: &mut Vec<Candidate>) {
    let mut lowest: Vec<usize> = Vec::new();
    for (index, candidate) in candidates.iter().enumerate() {
        if candidate.eliminated {
            continue;
        }
        if lowest.len() == 0 {
            lowest.push(index);
            continue;
        }
        match candidate.votes.cmp(&candidates[lowest[0]].votes) {
            Ordering::Less => {
                lowest.clear();
                lowest.push(index);
            }
            Ordering::Equal => lowest.push(index),
            _ => {}
        }
    }
    for index in lowest {
        candidates[index].eliminated = true;
    }
    for candidate in candidates {
        candidate.votes = 0;
    }
}
