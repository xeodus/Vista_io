use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("vista.io")
        .version("0.1.0")
        .author("Kaustab Bharasa kaustab08@gmail.com")
        .about("Track, scan, redact, and analyse leaked sensitive info through LLM prompts")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("scan")
            .about("Scan text for leaked sensitive info")
            .arg(
                Arg::new("input")
                .help("Input files to scan")
                .value_name("FILE")
                .required(true)
            ),
        )
        .subcommand(
            Command::new("redact")
            .about("Redact leaked sensitive info in text")
            .arg(
                Arg::new("input")
                .help("Input files to redact")
                .value_name("FILE")
                .required(true)
            )
            .arg(
                Arg::new("output")
                .help("Output file to save redact result")
                .value_name("FILE")
                .required(true)
            ),
        )
        .subcommand(
            Command::new("call")
            .about("Call LLM through prompt")
            .arg(
                Arg::new("input")
                .help("prompt file or raw string")
                .value_name("FILE_OR_TEXT")
                .required(true)
            )
            .arg(
                Arg::new("model")
                .long("model")
                .help("Call model: GPT-4")
                .value_name("MODEL")
                .default_value("GPT-4")
                .required(true)
            ),
        )
        .subcommand(
            Command::new("report")
            .about("Generate the report of findings")
            .arg(
                Arg::new("format")
                .long("format")
                .help("Format: json | markdown")
                .required(true)
                .value_parser(["json", "markdown"])
            )
        )
}