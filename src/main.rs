use std::fs;
use crate::{cli::build_cli, engine::{redactor::redact_text, scanner::scan_text}};
mod db;
mod cli;
mod engine;
mod llm;

fn main() {
    let matches_ = build_cli().get_matches();
    let conn = db::init_db().unwrap();

    match matches_.subcommand() {
        Some(("scan", sub_m)) => {
            let input_file = sub_m.get_one::<String>("input").unwrap();
            let content = fs::read_to_string(input_file).expect("Cannot read the input file..");
            let result = scan_text(&content);
            println!("{:?}", result.matches);
        },
        Some(("redact", sub_m)) => {
            let input_file = sub_m.get_one::<String>("input").unwrap();
            let content = fs::read_to_string(input_file).expect("Unable to read the input file..");
            let output = sub_m.get_one::<String>("output").unwrap();
            let scan = scan_text(&content);
            let redact = redact_text(&scan);
            fs::write(output, redact).expect("Failed to write the file..");
            println!("{:?}", output);
        },
        Some(("call", sub_m)) => {
            let input = sub_m.get_one::<String>("input").unwrap();
            let model = sub_m.get_one::<String>("model").unwrap();
            println!("Calling model {} through prompt: {}", model, input);

            // DATABASE initialization and log interaction
            let response = match llm::call_llm(input, model) {
                Ok(resp) => resp,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            let scan = scan_text(&response);
            let serialize_matches = serde_json::to_string(&scan.matches).unwrap();
            db::log_interaction(&conn, &input, &response, &serialize_matches, &model)
                .expect("Failed to log interation..");
            println!("LLM responded!");
        },
        Some(("report", sub_m)) => {
            let format_ = sub_m.get_one::<String>("format").unwrap();
            println!("Generating report in {} format", format_);
        },
        _ => unreachable!("Clap ensures only valid subcommands are matched")
    }
}
