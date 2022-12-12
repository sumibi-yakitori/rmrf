// type Result<T = (), E = anyhow::Error> = anyhow::Result<T, E>;
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result {
  for path in std::env::args().skip(1) {
    if let Err(e) = trash::delete(path) {
      eprintln!("{:#?}", e);
    }
  }
  Ok(())
}
