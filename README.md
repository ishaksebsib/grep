# grep

A simple grep tool implemented in Rust, capable of performing both case-sensitive and case-insensitive searches in text files.

## Features

- **Case-sensitive search**
- **Case-insensitive search** (enabled by setting the `CASE_SENSITIVE` environment variable)

## Installation

To install the tool, clone the repository and build the project:

```sh
git clone https://github.com/ishaksebsib/grep.git
cd grep
cargo build --release
```

## Usage

Run the tool with the query and file name as arguments:

```sh
./target/release/grep <QUERY> <FILE_NAME>
```

Example:

```sh
./target/release/grep "search_term" example.txt
```

To enable case-insensitive search, set the `CASE_SENSITIVE` environment variable:

```sh
CASE_SENSITIVE=1 ./target/release/grep "search_term" example.txt
```

## Running Tests

You can run the tests included in the project using the following command:

```sh
cargo test
```

## Contributing

Feel free to open issues or submit pull requests. Contributions are welcome!

## License

This project is licensed under the MIT License.
