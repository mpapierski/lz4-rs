extern crate cc;

use std::env;

fn main() {
    let mut compiler = cc::Build::new();
    compiler
        .file("liblz4/lib/lz4.c")
        .file("liblz4/lib/lz4frame.c")
        .file("liblz4/lib/lz4hc.c")
        .file("liblz4/lib/xxhash.c")
        // We always compile the C with optimization, because otherwise it is 20x slower.
        .opt_level(3);
    match env::var("TARGET").unwrap().as_str()
    {
      "i686-pc-windows-gnu" => {
        compiler
            .flag("-fno-tree-vectorize");
      },
      _ => {}
    }
    compiler.define("LZ4_FORCE_MEMORY_ACCESS", "0");
    compiler.define("LZ4_DEBUG", "2");
    compiler.define("LZ4F_HEAPMODE", "0");
    compiler.compile("liblz4.a");
}
