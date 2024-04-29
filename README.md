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

## Speed

Running it on [this 25MB JSON file][1] gave the following results:

```console
$ hyperfine 'formatjson large.json'
Benchmark 1: formatjson large.json
  Time (mean ± σ):     400.1 ms ±   9.3 ms    [User: 359.4 ms, System: 31.9 ms]
  Range (min … max):   390.1 ms … 419.6 ms    10 runs
```

Averaging 400 milliseconds on my M1 MacBook Air, which is about 38 times faster
than pretter, which took 15.27 seconds:

```console
$ time npx prettier --check large.json
Checking formatting...
[warn] large.json
[warn] Code style issues found in the above file. Run Prettier to fix.
npx prettier --check large.json  26.14s user 11.57s system 246% cpu 15.279 total
```

[1]: https://raw.githubusercontent.com/json-iterator/test-data/master/large-file.json
