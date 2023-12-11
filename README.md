# Advent of Code 2023

[![CI](https://img.shields.io/github/actions/workflow/status/gadomski/advent-of-code-2023/ci.yml?style=for-the-badge)](https://github.com/gadomski/advent-of-code-2023/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/gadomski/advent-of-code-2023?style=for-the-badge)](./LICENSE)

This year's self-imposed rules:

- Never unwrap (except in tests)
- No warnings
- Never break part 1 to solve part 2
- Minimal dependencies
  - **anyhow** for easy error handling, because who wants to muck around with that?
  - **num-integer** for least-common multiple, because again who wants to implement that algorithm from scratch?

## Usage

```shell
cargo run
```

There are one or more slow-running days where we resorted to brute force.
Run those with `--all`:

```shell
cargo run -- --all
```

## Testing

Each example is tested, and each part's answer is tested via a doctest:

```shell
cargo test
```

## License

Licensed under Apache License, Version 2.0 ([LICENSE](./LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0>).
