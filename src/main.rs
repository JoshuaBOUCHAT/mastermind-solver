use std::{collections::HashSet, io::stdin, usize};

const N: usize = 4;
const NB_COLORS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct Combination {
    colors: [u8; N],
}
impl Combination {
    pub fn new(colors: [u8; N]) -> Self {
        for i in 0..N {
            if colors[i] >= NB_COLORS as u8 {
                panic!("Something that should not happend");
            }
        }
        Self { colors }
    }
}
impl Default for Combination {
    fn default() -> Self {
        let mut colors = [0; N];
        for i in 2..N {
            colors[i] = (i - 1) as u8;
        }

        Self { colors }
    }
}

fn generate_all_combinations() -> Vec<Combination> {
    const TOTAL_ITER: usize = NB_COLORS.pow(N as u32);
    let mut combinations = Vec::new();
    for i in 0..TOTAL_ITER {
        let mut repr = i;
        let mut colors = [0; N];
        for i in 0..N {
            colors[i] = (repr % NB_COLORS) as u8;
            repr /= NB_COLORS;
        }
        combinations.push(Combination::new(colors));
    }
    combinations
}
fn guess_and_answer_to_response(guess: Combination, answer: Combination) -> (u8, u8) {
    let mut neededs = [0u8; NB_COLORS];
    let mut well_placed = 0;

    for i in 0..N {
        if answer.colors[i] != guess.colors[i] {
            neededs[answer.colors[i] as usize] += 1;
        } else {
            well_placed += 1;
        }
    }

    let mut misplaced = 0;
    for i in 0..N {
        if answer.colors[i] != guess.colors[i] && neededs[guess.colors[i] as usize] > 0 {
            neededs[guess.colors[i] as usize] -= 1;
            misplaced += 1;
        }
    }
    (misplaced, well_placed)
}
fn filter_with_response(
    combinations: Vec<Combination>,
    guess: Combination,
    misplaced: u8,
    well_placed: u8,
) -> Vec<Combination> {
    combinations
        .into_iter()
        .filter(|&combination| {
            guess_and_answer_to_response(guess, combination) == (misplaced, well_placed)
        })
        .collect()
}

fn count_after_filter(
    combinations: &[Combination],
    guess: Combination,
    misplaced: u8,
    well_placed: u8,
) -> usize {
    combinations
        .into_iter()
        .copied()
        .filter(|&combination| {
            guess_and_answer_to_response(guess, combination) == (misplaced, well_placed)
        })
        .count()
}

fn find_ideal_combination(
    all_combinations: &[Combination],
    filtered_combinations: &[Combination],
) -> Combination {
    assert!(filtered_combinations.len() != 0);
    let mut best_combination = all_combinations[0];
    let mut min_count = usize::MAX;
    for &guess in all_combinations {
        let mut worst_case = 0;
        for &answer in filtered_combinations {
            let (misplaced, well_placed) = guess_and_answer_to_response(guess, answer);
            let remaining =
                count_after_filter(filtered_combinations, guess, misplaced, well_placed);
            worst_case = worst_case.max(remaining); // plutôt que somme
        }
        if worst_case < min_count {
            println!("find a better combination: {:?}", &guess);
            min_count = worst_case;
            best_combination = guess;
        }
    }
    best_combination
}

fn read_response() -> (u8, u8) {
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("error when reading the cmd line");
    let (misplaced_str, well_placed_str) = buffer.trim().split_once(',').expect("can't split !");
    (
        misplaced_str.parse().expect("can't parse N1"),
        well_placed_str.parse().expect("cant parse N2"),
    )
}

fn main() {
    let all_combinations: Vec<Combination> = generate_all_combinations();
    let mut actual = all_combinations.clone();
    let first_guess = Combination::default();
    println!("Premier passage prenez les couleurs :{:?}", &first_guess);

    let (misplaced, well_placed) = read_response();

    actual = filter_with_response(actual, first_guess, misplaced, well_placed);

    println!("Le nombre de combinaision actuelle:{}", actual.len());

    while actual.len() > 1 {
        let best_combination = find_ideal_combination(&all_combinations, &actual);
        println!("Best combination:{:?}", &best_combination);
        let (misplaced, well_placed) = read_response();
        actual = filter_with_response(actual, best_combination, misplaced, well_placed);
        println!("Le nombre de combinaision actuelle:{}", actual.len());

        if actual.len() == 1 {
            println!("La réposne est: {:?}", actual[0]);
        }
    }
}
