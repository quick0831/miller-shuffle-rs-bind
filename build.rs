fn main() {
    cc::Build::new()
        .file("miller_src/MillerShuffle.c")
        .compile("miller_shuffle");

    println!("cargo:rerun-if-changed=miller_src/MillerShuffle.c");
}
