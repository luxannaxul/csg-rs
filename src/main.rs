use console::Term;
use std::{collections::HashSet, io};

#[derive(Debug)]
struct Production {
    left: String,
    right: HashSet<String>,
}

impl Production {
    fn new(left: String, right: HashSet<String>) -> Self {
        Self { left, right }
    }
}

enum Action {
    Exit,
    GenerateWordsTillDepthN(u32),
}
fn main() {
    let term = Term::stdout();
    let mut productions: Vec<Production> = Vec::new();
    let mut variables: HashSet<char> = HashSet::new();
    let mut terminals: HashSet<char> = HashSet::new();

    // variables
    {
        println!("variables..");
        loop {
            let char = Term::read_char(&term).expect("could not read char from term");
            if char == ' ' || char == '\n' {
                break;
            }
            println!("{}", char);
            variables.insert(char);
        }
        println!();
    }

    // terminals
    {
        println!("terminals..");
        loop {
            let char = Term::read_char(&term).expect("could not read char from term");
            if char == ' ' || char == '\n' {
                break;
            }
            println!("{}", char);
            terminals.insert(char);
        }
        println!()
    }

    assert!(variables.is_disjoint(&terminals));

    // productions
    {
        println!("productions..");
        loop {
            let mut user_input: String = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("error reading from stdin");
            if user_input == '\n'.to_string() {
                break;
            }
            productions.push(parse_production(user_input, &productions));
        }
    }

    // assert only defined chars were used in the productions
    for production in &productions {
        for left_char in production.left.as_bytes() {
            let mut char_is_defined = false;
            for variable in &variables {
                if *left_char == *variable as u8 {
                    char_is_defined = true;
                }
            }
            for terminal in &terminals {
                if *left_char == *terminal as u8 {
                    char_is_defined = true;
                }
            }
            assert!(char_is_defined);
        }
        for right_side in &production.right {
            for right_char in right_side.as_bytes() {
                let mut char_is_defined = false;
                for variable in &variables {
                    if *right_char == *variable as u8 {
                        char_is_defined = true;
                    }
                }
                for terminal in &terminals {
                    if *right_char == *terminal as u8 {
                        char_is_defined = true;
                    }
                }
                assert!(char_is_defined);
            }
        }
    }

    // start
    println!("start variable..");
    let start = Term::read_char(&term).expect("could not read char from term");
    println!("{}", start);

    let mut is_variable = false;
    for variable in &variables {
        if *variable == start {
            is_variable = true;
        }
    }
    assert!(is_variable);

    // action loop
    loop {
        match menu() {
            Action::Exit => break,
            Action::GenerateWordsTillDepthN(n) => {
                let mut starts: HashSet<String> = HashSet::new();
                starts.insert(start.to_string());
                for _ in 0..n {
                    starts = apply_productions_once(&starts, &productions);
                    starts = starts
                        .into_iter()
                        .filter(|word| {
                            if word_is_finished(word, &variables) {
                                print!("\"{word}\" ");
                                false
                            } else {
                                true
                            }
                        })
                        .collect();
                }
            }
        }
        println!("\n");
    }
}

fn word_is_finished(word: &String, variables: &HashSet<char>) -> bool {
    for letter in word.as_bytes() {
        for variable in variables {
            if *letter == *variable as u8 {
                return false;
            }
        }
    }
    true
}

fn apply_productions_once(
    starts: &HashSet<String>,
    productions: &Vec<Production>,
) -> HashSet<String> {
    let mut new_starts: HashSet<String> = HashSet::new();
    for start in starts {
        for production in productions {
            if start.contains(&production.left) {
                for (indice_of_start, _occurrence) in start.match_indices(&production.left) {
                    for right_side in &production.right {
                        let mut start_clone = start.clone();
                        start_clone.replace_range(
                            indice_of_start..indice_of_start + production.left.len(),
                            &right_side,
                        );
                        new_starts.insert(start_clone);
                    }
                }
            }
        }
    }
    new_starts
}

fn menu() -> Action {
    let term = Term::stdout();
    println!();
    println!("options are:\n(0) exit\n(1) generate words of max production depth n");
    let input = Term::read_char(&term).expect("could not read char from term");
    println!();
    match input {
        '0' => Action::Exit,
        '1' => {
            println!("n..");
            let n = Term::read_line(&term)
                .expect("could not read char from term")
                .trim()
                .replace(" ", "")
                .parse();
            println!();
            match n {
                Ok(n) => Action::GenerateWordsTillDepthN(n),
                Err(err) => {
                    println!("{err}");
                    menu()
                }
            }
        }
        _ => menu(),
    }
}

fn parse_production(mut user_input: String, existing_productions: &Vec<Production>) -> Production {
    user_input = user_input.trim().to_string();
    user_input = user_input.replace(" ", "");
    let index_of_arrow = user_input.find("->").expect("no '->' was found");
    assert!(index_of_arrow > 0);
    let left = &user_input[0..index_of_arrow];
    for existing_production in existing_productions {
        assert!(existing_production.left != left);
    }
    let mut right = HashSet::new();
    let right_slice = &user_input[index_of_arrow + 2..];
    right_slice.split("|").for_each(|one_right| {
        right.insert(one_right.to_string());
    });
    assert!(!right.is_empty());
    Production::new(left.to_string(), right)
}
