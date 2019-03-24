# Rust compiler installation
`$ curl https://sh.rustup.rs -sSf | sh`

# Choosing right compiler version
`$ rustup default nightly`

# Compilation
`cargo build --release`

# Usage
`./target/release/reader ./sample/path/ --files 130 --sort`

You can use short version of options
`--files` = `-f`
`--sort`  = `-s`
