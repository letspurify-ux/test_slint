# Iced Oracle Query Console (TOAD-like)

A mock TOAD-style Oracle query console built with Rust + Iced. The UI includes a connection panel, SQL editor, and result list powered by mocked data.

## Run

```bash
cargo run
```

## Notes

- The app uses mock data for query results and status updates.
- Hook up Oracle connectivity by replacing the `RunQuery` handler in `src/main.rs`.
- Uses Rust 2021 edition for compatibility with recent Cargo releases.
