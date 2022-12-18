[![Crates.io](https://img.shields.io/crates/v/rmrf.svg)](https://crates.io/crates/rmrf)
[![Documentation](https://docs.rs/rmrf/badge.svg)](https://docs.rs/rmrf)
[![License](https://img.shields.io/crates/l/rmrf.svg)](LICENSE)
[![Workflow Status](https://github.com/sumibi-yakitori/rmrf/workflows/Rust/badge.svg)](https://github.com/sumibi-yakitori/rmrf/actions?query=workflow%3A%22Rust%22)

# rmrf

A simple `rm -rf` command alternative.
This command moves directories and files to the trash instead of deleting them.
The path passed as an argument can be a file or a folder, and multiple arguments can be passed.


## Install

```sh
cargo install rmrf
```


## Usage

```sh
rmrf <path>
```
