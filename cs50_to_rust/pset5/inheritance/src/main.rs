use rand::{self, Rng};

const ALLELES: [char; 3] = ['A', 'B', 'O'];
const GENERATIONS: u32 = 3;
const GENERATION_NAMES: [&str; 3] = ["Child", "Parent", "Grandparent"];

struct Person {
    alleles: [char; 2],
    parents: Vec<Person>,
}

fn main() {
    let child = make_root_child(GENERATIONS);
    println!("{}", child.get_tree(0))
}

fn get_random_allele() -> char {
    ALLELES[rand::rng().random_range(0..3)]
}

fn make_root_child(generations: u32) -> Person {
    if generations == 1 {
        Person::new()
    } else {
        Person::from_parents(
            make_root_child(generations - 1),
            make_root_child(generations - 1),
        )
    }
}

impl Person {
    fn new() -> Self {
        Person {
            alleles: [get_random_allele(), get_random_allele()],
            parents: Vec::new(),
        }
    }
    fn from_parents(left: Person, right: Person) -> Self {
        Person {
            alleles: [
                left.alleles[rand::rng().random_range(0..2)],
                right.alleles[rand::rng().random_range(0..2)],
            ],
            parents: vec![left, right],
        }
    }
    fn get_tree(self, generation: usize) -> String {
        let generation_name = match generation {
            x if (0..GENERATION_NAMES.len()).contains(&x) => GENERATION_NAMES[x].to_owned(),
            _ => {
                
                let mut slices = vec!["Great-"];
                slices.extend(std::iter::repeat("great-").take(generation - 3));
                slices.push("grandparent");
                let mut name = String::new();
                for slice in slices {
                    name += slice
                }
                name
            }
        };
        let mut out = format!(
            "{}{} (Generation {generation}): blood type {}\n",
            vec![' '; 4 * generation].iter().collect::<String>(),
            generation_name,
            self.alleles.iter().collect::<String>()
        );
        if self.parents.len() != 0 {
            for parent in self.parents {
                out += parent.get_tree(generation + 1).as_str()
            }
        }
        out
    }
}
