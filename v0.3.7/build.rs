use cc::Build;
use std::env;
use walkdir::WalkDir;

fn main() {
    let mut cfg = Build::new();
    cfg.compiler("clang");
    cfg.flag("-nostdlibinc");
    cfg.flag("-fno-stack-protector");
    cfg.flag("-fno-profile-instr-generate");
    cfg.flag("-fno-coverage-mapping");
    cfg.define("COMPILER_RT_HAS_ATOMICS", "1");

    let sources = vec![
        "c/profile/InstrProfiling.c",
        "c/profile/InstrProfilingBuffer.c",
        "c/profile/InstrProfilingInternal.c",
        "c/profile/InstrProfilingMerge.c",
        "c/profile/InstrProfilingPlatformLinux.c",
        "c/profile/InstrProfilingPlatformOther.c",
        "c/profile/InstrProfilingPlatformWindows.c",
        "c/profile/InstrProfilingWriter.c",
        "c/profile/InstrProfilingValue.c",
        "c/profile/InstrProfilingVersionVar.c",
    ];

    let target = env::var("TARGET").unwrap_or_default();
    if target.ends_with("-uefi") {
        cfg.define("MINICOV_UEFI", "1");
    }

    for source in &sources {
        cfg.file(source);
    }

    cfg.compile("llvm_profiler_runtime");

    for entry in WalkDir::new("c") {
        println!(
            "cargo:rerun-if-changed={}",
            entry.unwrap().path().to_str().unwrap()
        );
    }
}
