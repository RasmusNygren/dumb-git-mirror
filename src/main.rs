use anyhow::{Context, Result};
use serde::Deserialize;
use std::{env, fs, path::Path, process::Command};
use tempfile::TempDir;

#[derive(Debug, Deserialize)]
struct Config {
    mirrors: Vec<Mirror>,
}

#[derive(Debug, Deserialize)]
struct Mirror {
    from: String,
    to: String,
}

fn run_git_command(args: &[&str], cwd: &Path) -> Result<std::process::Output> {
    Command::new("git")
        .arg("-C")
        .arg(cwd)
        .args(args)
        .output()
        .context("Git command failed")
}

fn update_mirror(from: &str, to: &str) -> Result<()> {
    let tempdir = TempDir::new().context("Failed to create temporary directory")?;
    let out_path = "aknownname";
    let to_remote_name = "target";

    run_git_command(&["clone", from, out_path], tempdir.path()).context("Git clone failed")?;

    let git_path = tempdir.path().join(out_path);
    run_git_command(&["remote", "add", to_remote_name, to], &git_path)
        .context("Git remote add failed")?;

    run_git_command(&["fetch", to_remote_name], &git_path).context("Git fetch failed")?;

    run_git_command(&["push", to_remote_name, "HEAD", "--tags"], &git_path)
        .context("Git push failed")?;

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 || args[1] != "--filename" {
        println!("Usage: {} --filename <file_name>", args[0]);
        std::process::exit(1);
    }

    let file_name = &args[2];
    let file = fs::read_to_string(file_name).context(format!("Failed to read {}", file_name))?;

    let contents: Config = serde_yaml::from_str(&file)
        .context(format!("Failed to deserialize {} as yaml", file_name))?;

    for target in &contents.mirrors {
        update_mirror(&target.from, &target.to)?;
        println!("Updated {} -> {}", target.from, target.to);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*; // Import the main module
    use serde_yaml;

    #[test]
    fn test_parser_valid() {
        let yaml = r#"
mirrors:
  - from: "https://github.com/example/from1.git"
    to: "https://github.com/example/to1.git"
  - from: "https://github.com/example/from2.git"
    to: "https://github.com/example/to2.git"
"#;

        let parsed_config: Config = serde_yaml::from_str(yaml).expect("Failed to parse YAML");

        assert_eq!(
            parsed_config.mirrors[0].from,
            "https://github.com/example/from1.git"
        );
        assert_eq!(
            parsed_config.mirrors[0].to,
            "https://github.com/example/to1.git"
        );
        assert_eq!(
            parsed_config.mirrors[1].from,
            "https://github.com/example/from2.git"
        );
        assert_eq!(
            parsed_config.mirrors[1].to,
            "https://github.com/example/to2.git"
        );
    }

    #[test]
    fn test_parser_invalid() {
        let yaml = r#"
mirrors:
  - from: "https://github.com/example/repo1.git"
"#;

        let parsed_config: Result<Config, _> = serde_yaml::from_str(yaml);
        assert!(parsed_config.is_err());
    }
}
