#![allow(clippy::let_and_return)]

<<<<<<< HEAD
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
=======
use fastrand::Rng;
use serde::Deserialize;
>>>>>>> 6ae19f83e8bb866d72392a958515b32aea1621c3

mod config;
use config::{load_config, Responses};

const AFFECTIONATE_TERM_PLACEHOLDER: &str = "AFFECTIONATE_TERM";
const MOMMYS_PRONOUN_PLACEHOLDER: &str = "MOMMYS_PRONOUN";
const MOMMYS_ROLE_PLACEHOLDER: &str = "MOMMYS_ROLE";

enum ResponseType {
    Positive,
    Negative,
}

fn main() {
    // Ideally mommy would use ExitCode but that's pretty new and mommy wants
    // to support more little ones~
    let code = real_main().unwrap_or_else(|e| {
        eprintln!("Error: {:?}", e);
        -1
    });
    std::process::exit(code)
}

fn real_main() -> Result<i32, Box<dyn std::error::Error>> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let mut arg_iter = std::env::args();
    let _cargo = arg_iter.next();
    let _mommy = arg_iter.next();

    let mut cmd = std::process::Command::new(cargo);
    cmd.args(arg_iter);
    let status = cmd.status()?;
    eprintln!("\x1b[1m");
    if status.success() {
        eprintln!("{}", select_response(ResponseType::Positive))
    } else {
        eprintln!("{}", select_response(ResponseType::Negative));
    }
    eprintln!("\x1b[0m");
    Ok(status.code().unwrap_or(-1))
}

fn select_response(response_type: ResponseType) -> String {
<<<<<<< HEAD
    let config = load_config().unwrap();

    let mut rng = StdRng::from_entropy();
=======
    let rng = Rng::new();
>>>>>>> 6ae19f83e8bb866d72392a958515b32aea1621c3

    // Get mommy's options~
    let affectionate_terms = parse_options(config.affectionate_terms);
    let mommys_pronouns = parse_options(config.pronouns);
    let mommys_roles = parse_options(config.roles);

    // Choose what mommy will say~
    let responses: Responses = config.responses.unwrap_or_default();

<<<<<<< HEAD
    let responses_binding = match response_type {
        ResponseType::Positive => responses.positive,
        ResponseType::Negative => responses.negative,
    }
    .unwrap();
    let response = responses_binding
        .choose(&mut rng)
        .expect("non-zero amount of responses");
=======
    let responses = match response_type {
        ResponseType::Positive => &responses.positive,
        ResponseType::Negative => &responses.negative,
    };
    let response = &responses[rng.usize(..responses.len())];
>>>>>>> 6ae19f83e8bb866d72392a958515b32aea1621c3

    // Apply options to the message template~
    let response = apply_template(
        response,
        AFFECTIONATE_TERM_PLACEHOLDER,
        &affectionate_terms,
        &rng,
    );
    let response = apply_template(
        &response,
        MOMMYS_PRONOUN_PLACEHOLDER,
        &mommys_pronouns,
        &rng,
    );
    let response = apply_template(&response, MOMMYS_ROLE_PLACEHOLDER, &mommys_roles, &rng);

    // Done~!
    response
}

fn parse_options(input: String) -> Vec<String> {
    input.split('/').map(|s| s.to_string()).collect()
}

fn apply_template(input: &str, template_key: &str, options: &[String], rng: &Rng) -> String {
    let mut last_position = 0;
    let mut output = String::new();
    for (index, matched) in input.match_indices(template_key) {
        output.push_str(&input[last_position..index]);
        output.push_str(&options[rng.usize(..options.len())]);
        last_position = index + matched.len();
    }
    output.push_str(&input[last_position..]);
    output
}

#[cfg(test)]
#[test]
fn test() {
    // Uncomment if you want a failing test
    // panic!("oops!!");
}
