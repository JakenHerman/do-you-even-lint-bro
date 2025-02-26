use clap::Parser;
use anyhow::{bail, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to scan
    #[arg(short, long, default_value = ".")]
    dir: String,

    /// Linter to check for (e.g., "mypy" or "flake8")
    #[arg(short, long)]
    linter: String,

    /// Optional output file; if not specified, results are printed to stdout
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let pattern = match args.linter.to_lowercase().as_str() {
        "mypy" => r"#\s*type:\s*ignore\[(?P<ignore>[^\]]+)\]",
        "flake8" => r"#\s*noqa(?::\s*(?P<ignore>.*))?",
        _ => bail!("Unsupported linter: {}", args.linter),
    };

    let re = Regex::new(pattern)?;
    let mut counts: HashMap<String, usize> = HashMap::new();

   
    // Traverse the directory recursively and process only .py files.
    for entry in WalkDir::new(&args.dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "py" {
                    let content = fs::read_to_string(path)?;
                    for line in content.lines() {
                        if let Some(captures) = re.captures(line) {
                            if let Some(m) = captures.name("ignore") {
                                let ignore_content = m.as_str();
                                // Split by comma and trim whitespace for each key. Mypy allows you to ignore
                                // multiple warnings at once, so we need to count each one separately.
                                for part in ignore_content.split(',') {
                                    let key = part.trim().to_string();
                                    if !key.is_empty() {
                                        *counts.entry(key).or_insert(0) += 1;
                                    }
                                }
                            } else {
                                *counts.entry(String::new()).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut results: Vec<_> = counts.into_iter().collect();
    results.sort_by_key(|(pattern, _)| pattern.clone());

    let mut output_lines = vec![
        format!("Unique ignored {} patterns found: {}", args.linter, results.len())
    ];

    for (pattern, count) in results {
        let display_pattern = if pattern.is_empty() { "<empty>" } else { &pattern };
        output_lines.push(format!("'{}' ignored {} times", display_pattern, count));
    }
    let output_string = output_lines.join("\n");

    // Write to file if an output file was specified, otherwise print to stdout.
    if let Some(output_path) = args.output {
        let mut file = fs::File::create(output_path)?;
        writeln!(file, "{}", output_string)?;
    } else {
        println!("{}", output_string);
    }


    Ok(())
}
