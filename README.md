# Taletski.com

Code for my personal website.

## Context
Everything is written in Rust with Axum and Tower for HTTP server functionality and HTML/JS/Tailwind for front-end.
Rust is by far an overkill for such a simple static website. I am just working on making Rust my go-to language, as I am excited about the language syntax, features, and capabilities and want to explore its strengths and weaknesses more in depth in all kinds of projects.

## Development

Set up the dev environment with
```bash
bash ./scripts/bootstrap-dev.sh
```
You are all set, now you have:
- A proper `.env` file for local debug.
- A dev server with live rebuilds on file changes. Run it with `cargo dev`.
- Pre-commit formatting and linting. Fires automatically on commit.
- Commit message lint for conventional commits. Fires automatically on commit.

## Testing

### Unit tests
Run `cargo test`.

### Local E2E
You'll notice this repo has a full local E2E testing setup. Yeah, it might be a bit much, but here's why it's there:
1. I hate manually re-testing things, so I automated it.
2. I plan on refactoring this constantly as I get better with Rust.
3. I was curious about how E2E testing works in Rust.
4. Frankly, I'm a stickler for production-grade code.

#### Prerequisites
1. A chromedriver compatible with your local browser. You can check the latest version [here](https://googlechromelabs.github.io/chrome-for-testing/).
    Make sure to download the version corresponding to your installed Chrome version.
    After download, make chromedriver available on PATH. Easiest way to do it on MacOS is to `mv .../path/to/chromedriver /usr/local/bin/chromedriver`.

#### Running the tests
Run local E2E with `cargo test -p e2e`.
