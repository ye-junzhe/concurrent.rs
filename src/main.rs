mod backend;

use backend::model::entry;

fn main() -> std::io::Result<()> {
    Ok(entry())
}
