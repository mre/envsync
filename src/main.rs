use anyhow::{Context, Result};
use clap::{command, Parser};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(
        value_name = "FILE",
        default_value = ".env",
        help = "Sets the .env file to create a .env.sample for (defaults to .env)"
    )]
    env_file: PathBuf,
    #[clap(
        short,
        long,
        value_name = "FILE",
        help = "Sets the env.sample file to use (defaults to name of .env file with .env replaced by .env.sample)"
    )]
    sample_file: Option<PathBuf>,
    #[clap(
        short,
        long,
        value_name = "VAR=VALUE",
        help = "Sets an example value for a specific environment variable"
    )]
    example: Vec<String>,
}

fn create_sample_content(env_file: String, examples: &HashMap<String, String>) -> String {
    let mut sample_content = String::new();
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
    sample_content
}

/// Scrubs sensitive data from a .env file and generates an env.sample file
fn scrub_env_file(
    env_file_name: PathBuf,
    sample_file_name: PathBuf,
    examples: &HashMap<String, String>,
) -> Result<()> {
    let env_file = fs::read_to_string(env_file_name).context("Could not read .env file")?;
    let sample_content = create_sample_content(env_file, examples);

    let mut sample_file =
        File::create(sample_file_name).context("Cannot create env.sample file")?;
    sample_file
        .write_all(sample_content.as_bytes())
        .context("Cannot write to env.sample file")?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let sample_file_name = match args.sample_file {
        Some(file_name) => file_name,
        None => sample_file_path(&args.env_file)?,
    };

    println!("Creating sample env file: {}", sample_file_name.display());

    let mut examples = HashMap::new();
    for example in args.example {
        let example_parts = example.split('=').collect::<Vec<&str>>();
        examples.insert(example_parts[0].to_string(), example_parts[1].to_string());
    }

    scrub_env_file(args.env_file, sample_file_name, &examples)
        .context("Error while creating sample env file")?;
    Ok(())
}

/// Returns the path to the sample file based on the path to the env file
///
/// # Example
///
/// ```rust
/// let env_file = PathBuf::from("/path/to/.env");
/// let sample_file = sample_file_path(env_file);
/// assert_eq!(sample_file, PathBuf::from("/path/to/.env.sample"));
/// ```
fn sample_file_path(env_file: &PathBuf) -> Result<PathBuf> {
    let env_file_name = env_file
        .file_name()
        .context("Cannot get filename from env_file")?
        .to_str()
        .context("Cannot convert env_file name to string")?;

    let sample_file_name = format!("{}.sample", env_file_name);

    let mut path = env_file.clone();
    path.set_file_name(sample_file_name);

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_create_sample_content() {
        let env_file = "DB_HOST=localhost\nDB_USER=root\nDB_PASSWORD=secret\nDB_NAME=database";

        let mut examples = HashMap::new();
        examples.insert("DB_HOST".to_string(), "localhost".to_string());

        let sample_content = create_sample_content(env_file.to_string(), &examples);

        assert_eq!(
            sample_content,
            "DB_HOST=localhost\nDB_USER=<DB_USER>\nDB_PASSWORD=<DB_PASSWORD>\nDB_NAME=<DB_NAME>\n"
        );
    }
}
