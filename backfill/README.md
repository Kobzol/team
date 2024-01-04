# Backfill script
Usage:
1) Fill in GitHub token in `src/bin/fetch-repos`
2) Run the following:
```bash
$ cargo run --bin fetch-repos > repos.json
```

Create missing repos:
```bash
$ cargo run --bin create-repo-files
```
