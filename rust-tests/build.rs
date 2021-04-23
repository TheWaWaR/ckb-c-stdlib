fn main() {
    cc::Build::new()
        .file("src/tests/ckb_smt.c")
        .static_flag(true)
        .flag("-O3")
        .flag("-fvisibility=hidden")
        .flag("-fdata-sections")
        .flag("-ffunction-sections")
        .include("..")
        .flag("-Wall")
        .flag("-Werror")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-nonnull")
        .define("__SHARED_LIBRARY__", None)
        .define("CKB_STDLIB_NO_SYSCALL_IMPL", None)
        .compile("dl-c-impl");
}
