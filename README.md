
# Discords Fastest Token Checker

This Rust project allows you to efficiently check the validity of Discord tokens by making concurrent requests to the Discord API. It uses the Tokio runtime for asynchronous execution and the Reqwest library for making HTTP requests.

## Prerequisites

-   Rust ([https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install))
-   `Cargo` (Rust's package manager) is installed along with Rust.

## Usage

1.  Clone the repository or download the source code.
2.  Create a file named `tokens.txt` in the project directory and add your Discord tokens, each on a separate line.
3.  Open a terminal and navigate to the project directory.

### Building and Running

To build and run the project, use the following commands in your terminal:
```sh
cargo build --release
cargo run --release
```

### Using Pre-built Releases

Alternatively, you can download pre-built releases from the "Releases" section of this repository. Extract the release archive, create a `tokens.txt` file in the extracted directory, add your tokens, and then run the executable.

## Output

The program will asynchronously check each token's validity by making requests to the Discord API. Valid tokens will be printed in the console output.

## Important Notes

-   Do not use this tool for malicious purposes. Always respect Discord's terms of service and guidelines.
-   The tool is provided as-is and may require updates to work with future changes in the Discord API.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/GSstarGamer/Discords-fastest-token-checker/blob/master/LICENSE) file for details.
