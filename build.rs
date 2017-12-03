extern crate cc;

use std::env;
use std::path::Path;
use cc::Build;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src = Path::new(&dir).join("c");
    let fs  = &["decode.c", "identify.c", "quirc.c", "version_db.c"];
    let fs  = fs.iter().map(|f| src.join(f));
    Build::new().include(&src).files(fs).compile("quirc");
}
