
//use std::process::Command;
use std::env;
//use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    // Command::new("cargo").args(&["build",
    //                     "-p", "disco",
    //                     "--target=wasm32-unknown-unknown",
    //                     "--target-dir=alt-target",
    //                     "--release",
    //                     ])
    //                    .status().unwrap();

    // Command::new("wasm-bindgen")
    //                   .args(&["crus", "libhello.a", "hello.o"])
    //                   .current_dir(&Path::new(&out_dir))
    //                   .status().unwrap();

}
//     cargo build -p mywasm \
//    \
  

// wasm-bindgen --target=web \
//   --out-dir=final-out-dir \
//   alt-target/wasm32-unknown-unknown/release/mywasm.wasm
