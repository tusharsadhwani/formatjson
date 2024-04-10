# formatjson

A simple JSON validator and formatter written in Rust.

For documentation check [the docs.rs page](https://docs.rs/formatjson).

## Installation

```bash
cargo install formatjson
```

## Usage

- As a command-line tool:

  ```console
  $ formatjson example.json
  Successfully formatted example.json
  $ echo '{"foo": "bar"}' | formatjson
  {
    "foo": "bar"
  }
  ```

- As a library:

  ```rust
  fn main() {
      let json = formatjson::format_json(r#"{"foo":"bar"}"#).unwrap();
      println!("{}", json);
  }
  ```

  ```console
  $ cargo run
  {
    "foo": "bar"
  }
  ```
