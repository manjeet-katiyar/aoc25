# [Advent of Code 2025](https://adventofcode.com/2025)

This repository contains my solutions for Advent of Code 2025, implemented in Rust.

## Running a Solution

To run the solution for a specific day (e.g., Day 01), navigate to the root of the repository and use the following command:

```bash
cargo run -p day01
```

## Adding a New Day

To add a solution for a new day:

1.  **Create a new binary crate** for the day from the project root:

    ```bash
    cargo new day01 --bin
    ```

2.  **Add the `utils` dependency** to the new day's `Cargo.toml`:

    ```toml
    [dependencies]
    utils = { path = "../utils" }
    ```
