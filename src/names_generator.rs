use rand::seq::SliceRandom;

const ADJECTIVES: [&str; 5] = [
    "adjective1",
    "adjective2",
    "adjective3",
    "adjective4",
    "adjective5",
];

const NOUNS: [&str; 5] = [
    "noun1",
    "noun2",
    "noun3",
    "noun4",
    "noun5",
];

pub fn get_random_name() -> String {
    let mut rng = rand::thread_rng();
    format!("{}_{}", ADJECTIVES.choose(&mut rng).unwrap(), NOUNS.choose(&mut rng).unwrap())
}
