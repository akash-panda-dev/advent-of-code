# Project Commands and Guidelines

## Commands
- Run solution: `just run <day> <part>` (e.g., `just run 1 1`)
- Run tests: `just test <day> <part>` (e.g., `just test 1 1`)
- Run single test: `cargo test -p day<day> part<part>::test_name`
- Lint: `just lint <day>` or `cargo clippy -p day<day>`
- Benchmark: `just bench <day> <part>`
- Create new day: `just create <day>`
- Watch + auto-run: `just work <day> <part>`

## Code Guidelines
- Error handling: Use `anyhow::Result` for error returns
- Tracing: Add `#[tracing::instrument]` to functions
- Modules: Organize code into `part1` and `part2` modules
- Types: Prefer strong typing; use appropriate numeric types
- Formatting: Follow rustfmt conventions
- Naming: Use snake_case for functions/variables, CamelCase for types
- Testing: Add test modules where needed (with rstest when useful)
- Performance: Consider using Rayon for parallelism when beneficial