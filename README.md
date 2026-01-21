# Slint Oracle Query Console (TOAD-like)

A mock TOAD-style Oracle query console built with Rust + Slint. The UI includes a connection panel, SQL editor, and result table powered by mocked data.

## Run

```bash
cargo run
```

## Notes

- The app uses mock data for query results and status updates.
- Hook up Oracle connectivity by replacing the `run_query` callback in `src/main.rs`.
- Uses Rust 2021 edition to stay compatible with Cargo 1.82.
