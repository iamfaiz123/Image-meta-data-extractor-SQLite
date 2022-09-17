!/bin/bash

# rocket is unstable and uses many compiler features which are only present in nightly rust compilers
rustup default nightly
rustup override set nightly
rustup update && cargo update

cargo run
