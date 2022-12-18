//! A simple `rm -rf` command alternative.
//! This command moves directories and files to the trash instead of deleting them.
//! The path passed as an argument can be a file or a folder, and multiple arguments can be passed.
//!
//!
//! # Install
//!
//! ```sh
//! cargo install rmrf
//! ```
//!
//!
//! # Usage
//!
//! ```sh
//! rmrf <path>
//! ```

// type Result<T = (), E = anyhow::Error> = anyhow::Result<T, E>;
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result {
  for path in std::env::args().skip(1) {
    // The reason for using the `trash::delete` method instead of `trash::delete_all`
    // is that you can continue if an error occurs.
    if let Err(e) = trash::delete(path) {
      eprintln!("{:#?}", e);
    }
  }
  Ok(())
}
