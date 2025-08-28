use std::{env::args, fmt};

use anyhow::{Error, Result, bail};
use my_library::{get_input, get_string};

const MAX: usize = 9;
const MAX_VOTERS: usize = 100;

#[derive(Clone)]
struct Candidate {
    name: String,
    eliminated: bool,
}

impl Candidate {
    pub fn new(name: String) -> Candidate {
        Candidate {
            name: name,
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

#[derive(Debug)]
struct Pair {
    winner: String,
    loser: String,
    margin: usize,
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

    // Initialize candidates and pairs
    let mut candidates: Vec<Candidate> = Vec::new();
    let mut pairs: Vec<Pair> = Vec::new();
    for candidate in &args[1..] {
        candidates.push(Candidate::new(candidate.to_owned()))
    }

    // Get number of voters
    let votes: usize = loop {
        let input: usize = get_input("Number of voters: ");
        if input <= MAX_VOTERS {
            break input;
        }
        println!("Too many votes, max: {}", MAX_VOTERS);
    };

    // voter_preferences[voter][rank] -> candidate index
    let mut voter_preferences: Vec<Vec<usize>> = vec![vec![0; candidates.len()]; votes];

    // Collect votes
    for voter in 0..votes {
        println!("Voter {}", voter + 1);
        match register_ranked_vote(voter, &mut candidates, &mut voter_preferences) {
            Err(e) => bail!(e),
            _ => {}
        };
    }

    // Determine and print winner
    let winner: Candidate = determine_winners(&mut candidates, &voter_preferences, &mut pairs);
    println!("{}", winner);
    Ok(())
}

// Get ranked vote
fn register_ranked_vote(
    voter: usize,
    candidates: &mut Vec<Candidate>,
    voter_preferences: &mut Vec<Vec<usize>>,
) -> Result<(), Error> {
    let mut seen = std::collections::HashSet::new();
    for rank in 0..candidates.len() {
        let index = loop {
            print!("Rank {}: ", rank + 1);
            let vote = get_string("");
            match candidates.iter().position(|c| c.name == vote) {
                Some(i) => {
                    if !seen.insert(i) {
                        bail!("Duplicate vote detected for candidate: {}", vote);
                    }
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

// Run Tideman algorithm and return winner
fn determine_winners(
    candidates: &mut Vec<Candidate>,
    voter_preferences: &Vec<Vec<usize>>,
    pairs: &mut Vec<Pair>,
) -> Candidate {
    populate_pairs(candidates, voter_preferences, pairs);
    lock_pairs(candidates, pairs);

    candidates
        .iter()
        .find(|c| !c.eliminated)
        .expect("No condorcet winner was found. Pair locking likely failed.")
        .clone()
}

// Build pairs from voter preferences
fn populate_pairs(
    candidates: &mut Vec<Candidate>,
    voter_preferences: &Vec<Vec<usize>>,
    pairs: &mut Vec<Pair>,
) {
    let num_candidates = candidates.len();

    // For each voter, for each pair of candidates (i, j) where i is ranked above j
    for voter in voter_preferences {
        for i in 0..num_candidates {
            let current = voter[i];
            for j in (i + 1)..num_candidates {
                let bellow = voter[j];

                // Find if this pair already exists
                if let Some(p) = pairs.iter_mut().find(|p| {
                    p.winner == candidates[current].name && p.loser == candidates[bellow].name
                }) {
                    p.margin += 1;
                } else {
                    pairs.push(Pair {
                        winner: candidates[current].name.to_owned(),
                        loser: candidates[bellow].name.to_owned(),
                        margin: 1,
                    });
                }
            }
        }
    }

    // Sort pairs by margin (descending)
    pairs.sort_by(|a, b| b.margin.cmp(&a.margin));
}

// Lock pairs into the graph, avoiding cycles
fn lock_pairs(candidates: &mut Vec<Candidate>, pairs: &Vec<Pair>) {
    let n = candidates.len();
    // locked[winner][loser] == true means winner is locked over loser
    let mut locked = vec![vec![false; n]; n];

    // DFS helper for cycle detection
    fn creates_cycle(locked: &Vec<Vec<bool>>, start: usize, target: usize) -> bool {
        if start == target {
            return true;
        }
        for (i, &locked_edge) in locked[start].iter().enumerate() {
            if locked_edge && creates_cycle(locked, i, target) {
                return true;
            }
        }
        false
    }

    // Lock pairs if no cycle is created
    for pair in pairs {
        let winner = candidates
            .iter()
            .position(|c| c.name == pair.winner)
            .unwrap();
        let loser = candidates
            .iter()
            .position(|c| c.name == pair.loser)
            .unwrap();

        if !creates_cycle(&locked, loser, winner) {
            locked[winner][loser] = true;
        }
    }

    // Find candidate with no incoming edges (the winner)
    let mut winner_index = None;
    'outer: for i in 0..n {
        for j in 0..n {
            if locked[j][i] {
                continue 'outer;
            }
        }
        winner_index = Some(i);
        break;
    }

    // Eliminate all except the winner
    for (i, candidate) in candidates.iter_mut().enumerate() {
        candidate.eliminated = Some(i) != winner_index;
    }
}
