# Taletski.com

My personal website built with Rust.
This is by far not the best stack choice, but I do it for fun and for learning as I am working on making Rust my go-to language. 

## Testing

### Unit tests
Just run `cargo test`;

### Local E2E and Screenshot Testing
The repo features full local end-to-end testing. This may seem an overkill, but there are reasons to have it:
1. I am way too lazy to re-test things manually;
2. I will refactor this project aggressively from time to time while I am learning more about Rust;
3. I wanted to try out E2E setup in Rust ecosystem;

#### Prerequisites
1. A chromedriver compatible with your local browser. You can check the latest version [here](https://googlechromelabs.github.io/chrome-for-testing/).
    Make sure to download the version corresponding to your installed Chrome version.
    After download, make chromedriver available on PATH. Easiest way to do it on MacOS is to `mv .../path/to/chromedriver /usr/local/bin/chromedriver`. 

#### Running the tests
Run local E2E with `cargo test -p e2e`.
