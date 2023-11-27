use colored::Colorize;
use std::{env, io::Write, process::exit};

pub struct Config {
    pub api_base: String,
    pub shell: String,
}

impl Config {
    pub fn new() -> Self {
        let api_base = env::var("OLLAMA_API_BASE").unwrap_or_else(|_| String::from("http://localhost:11434/api"));
        let shell = env::var("SHELL").unwrap_or_else(|_| String::new());

        Self { api_base, shell }
    }

    pub fn write_to_history(&self, code: &str) {
        let history_file = match self.shell.as_str() {
            "/bin/bash" => std::env::var("HOME").unwrap() + "/.bash_history",
            "/bin/zsh" => std::env::var("HOME").unwrap() + "/.zsh_history",
            _ => return,
        };

        std::fs::OpenOptions::new()
            .append(true)
            .open(history_file)
            .map_or((), |mut file| {
                file.write_all(format!("{code}\n").as_bytes())
                    .unwrap_or_else(|_| {
                        exit(1);
                    });
            });
    }
}
