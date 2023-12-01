#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use bat::PrettyPrinter;
use clap::Parser;
use colored::Colorize;
use config::Config;
use question::{Answer, Question};
use reqwest::blocking::Client;
use serde_json::json;
use spinners::{Spinner, Spinners};
use std::process::Command;

mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Description of the command to execute
    prompt: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let config = Config::new();

    let client = Client::new();
    
    let mut should_run = false;
    let mut code = String::new();

    while !should_run {
        let mut sp_cmd_gen = Spinner::new(Spinners::BouncingBar, "Generating your command...".into());
        let api_addr = format!("{}/generate", config.api_base);
        let model = format!("{}", config.model);
        let response = client
            .post(api_addr)
            .json(&json!({
                "model": model,
                "stream": false,
                "system": "You are a helpful assistant who is specialized in generating one-liner shell command based on user prompt. Reply only the command and absolutely nothing else. The assistant, only",
                "prompt": build_prompt(&cli.prompt.join(" ")),
            }))
            .send()
            .unwrap();

        let status_code = response.status();
        if status_code.is_client_error() {
            let response_body = response.json::<serde_json::Value>().unwrap();
            let error_message = response_body["error"]["message"].as_str().unwrap();
            sp_cmd_gen.stop_and_persist(
                "✖".red().to_string().as_str(),
                format!("API error: \"{error_message}\"").red().to_string(),
            );
            std::process::exit(1);
        } else if status_code.is_server_error() {
            sp_cmd_gen.stop_and_persist(
                "✖".red().to_string().as_str(),
                format!("Please check if Ollama is up and running. Status code: {status_code}")
                    .red()
                    .to_string(),
            );
            std::process::exit(1);
        }

        code = response.json::<serde_json::Value>().unwrap()["response"]
            .as_str()
            .unwrap()
            .trim()
            .to_string();

        sp_cmd_gen.stop_and_persist(
            "✔".green().to_string().as_str(),
            "Got some code!".green().to_string(),
        );

        PrettyPrinter::new()
            .input_from_bytes(code.as_bytes())
            .language("bash")
            .grid(true)
            .print()
            .unwrap();

        let answer = Question::new(
            ">> Run the generated program? [Y/n/r]"
                .bright_black()
                .to_string()
                .as_str(),
        )
        .default(Answer::YES)
        .accept("y")
        .accept("n")
        .accept("r")
        .until_acceptable()
        .ask()
        .expect("Couldn't ask question.");
        match answer {
            Answer::YES => should_run = true,
            Answer::RESPONSE(response) => {
                match response.to_lowercase().as_str() {
                    "y" => should_run = true,
                    "r" => should_run = false,
                    _ => {
                        std::process::exit(0);
                    },
                }
            },
            _ => should_run = false,
        };
    }

    if should_run {
        config.write_to_history(code.as_str());
        let mut sp_cmd_run = Spinner::new(Spinners::BouncingBar, "Executing...".into());

        let output = Command::new("bash")
            .arg("-c")
            .arg(code.as_str())
            .output()
            .unwrap_or_else(|_| {
                sp_cmd_run.stop_and_persist(
                    "✖".red().to_string().as_str(),
                    "Failed to execute the generated program.".red().to_string(),
                );
                std::process::exit(1);
            });

        if !output.status.success() {
            sp_cmd_run.stop_and_persist(
                "✖".red().to_string().as_str(),
                "The program threw an error.".red().to_string(),
            );
            println!("{}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }

        sp_cmd_run.stop_and_persist(
            "✔".green().to_string().as_str(),
            "Command ran successfully".green().to_string(),
        );

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn build_prompt(prompt: &str) -> String {
    if prompt.trim().is_empty() {
        eprintln!("Error: The prompt is empty.");
        std::process::exit(1);
    }

    let os_hint = if cfg!(target_os = "macos") {
        " (on macOS)"
    } else if cfg!(target_os = "linux") {
        " (on Linux)"
    } else {
        ""
    };

    format!("{prompt}{os_hint}:\n```bash\n#!/bin/bash\n")
}
