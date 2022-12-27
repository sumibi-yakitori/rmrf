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

use argh::FromArgs;

// type Result<T = (), E = anyhow::Error> = anyhow::Result<T, E>;
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

/// rmfm
#[derive(Debug, FromArgs)]
struct Cli {
  /// whether to perform a normal delete operation when the operation to trash a file or folder fails
  #[argh(switch, short = 'f')]
  force: bool,

  /// paths
  #[argh(positional)]
  paths: Vec<String>,
}

fn main() -> Result {
  let cli: Cli = argh::from_env();

  for path in cli.paths {
    match std::fs::canonicalize(path) {
      Ok(path) => {
        // The reason for using the `trash::delete` method instead of `trash::delete_all`
        // is that you can continue if an error occurs.
        if let Err(e) = trash::delete(&path) {
          if cli.force {
            if path.is_file() {
              if let Err(e) = std::fs::remove_file(path) {
                eprintln!("{:#?}", e);
              }
            }
            else if path.is_dir() {
              if let Err(e) = std::fs::remove_dir_all(path) {
                eprintln!("{:#?}", e);
              }
            }
          }
          else {
            eprintln!("{:#?}", e);
          }
        }
      }
      Err(e) => eprintln!("{:#?}", e),
    }
  }
  Ok(())
}
