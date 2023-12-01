# Copilot, for your terminal using Ollama

A CLI tool that generates shell scripts from a human readable description with locally running LLMs.

## Requrements

- Working [Ollama](https://ollama.ai/) setup
- Desired model pulled (default: codellama) (ie. `ollama pull codellama`)
- Make sure you have the latest version of rust installed (use [rustup](https://rustup.rs/)).

## Setup and Installation

Clone this repo and build with `cargo build`, then you can find built binary in `target/debug/plz`.

## Usage

This fork is updated to talk to locally running LLMs using Ollama. The model defaults to `codellama`.

You can configure which model to use by setting `PLZ_MODEL_NAME` env var. You can also configure base API URL by setting `OLLAMA_API_BASE` env var. (default: http://localhost:11434/api)

If you don't like the generated command in the first attempt, you can chose `r` option to regenerate.

To get a full overview of all available options, run `plz --help`

```sh
$ plz --help
Generates bash scripts from the command line

Usage: plz [PROMPT]

Arguments:
  [PROMPT]  Description of the command to execute

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
