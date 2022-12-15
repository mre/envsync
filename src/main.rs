use clap::{command, Parser};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(
        value_name = "FILE",
        default_value = ".env",
        help = "Sets the .env file to create a .env.sample for (defaults to .env)"
    )]
    env_file: String,
    #[clap(
        short,
        long,
        value_name = "FILE",
        help = "Sets the env.sample file to use (defaults to name of .env file with .env replaced by .env.sample)"
    )]
    sample_file: Option<String>,
    #[clap(
        short,
        long,
        value_name = "VAR=VALUE",
        help = "Sets an example value for a specific environment variable"
    )]
    example: Vec<String>,
}



/// Scrubs sensitive data from a .env file and generates an env.sample file
fn scrub_env_file(
    env_file_name: &String,
    sample_file_name: &String,
    examples: &HashMap<String, String>,
) {
    let mut sample_content = String::new();

    let env_file = fs::read_to_string(env_file_name).unwrap();

    for line in env_file.lines() {
        if !line.starts_with("#") && !line.trim().is_empty() {
            let env_var = line.split('=').collect::<Vec<&str>>()[0];
            let example_value = match examples.get(env_var) {
                Some(value) => value.to_string(),
                None => format!("<{}>", env_var),
            };
            sample_content.push_str(&format!("{}={}\n", env_var, example_value));
        } else {
            sample_content.push_str(&format!("{}\n", line));
        }
    }

    let mut sample_file = File::create(sample_file_name).unwrap();
    sample_file.write_all(sample_content.as_bytes()).unwrap();
}

fn main() {
    let args = Args::parse();

    let sample_file_name = match args.sample_file {
        Some(file_name) => file_name,
        None => args.env_file.replace(".env", ".env.sample"),
    };

    let mut examples = HashMap::new();
    for example in args.example {
        let example_parts = example.split('=').collect::<Vec<&str>>();
        examples.insert(example_parts[0].to_string(), example_parts[1].to_string());
    }

    scrub_env_file(&args.env_file, &sample_file_name, &examples);
}
