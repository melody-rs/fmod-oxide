use std::{fs, path::PathBuf};

#[derive(Debug, Default)]
pub struct VersionCallbacks;

static DETECTED_VERSION: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(i64::MAX);

impl bindgen::callbacks::ParseCallbacks for VersionCallbacks {
    fn int_macro(&self, name: &str, value: i64) -> Option<bindgen::callbacks::IntKind> {
        if name == "FMOD_VERSION" {
            DETECTED_VERSION.store(value, std::sync::atomic::Ordering::Relaxed);
        }

        None
    }
}

#[cfg(windows)]
fn find_fmod_directory() -> PathBuf {
    if let Some(override_dir) = std::env::var_os("FMOD_SYS_FMOD_DIRECTORY").map(PathBuf::from) {
        if override_dir.exists() {
            return path;
        }
    }

    for drive in ["C", "D"] {
        let test_path = PathBuf::from(format!(
            "{drive}:/Program Files (x86)/FMOD SoundSystem/FMOD Studio API Windows"
        ));
        if test_path.exists() {
            return test_path;
        }
    }

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    for path in [
        out_dir.join("FMOD Studio API Windows"),
        out_dir.join("FMOD SoundSystem"),
    ] {
        if path.exists() {
            return path;
        }
    }

    // try to find it in _our_ crate root (useful for hacking on this fmod-audio-sys crate)
    let in_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fmod");
    if in_dir.exists() {
        return in_dir;
    }

    panic!(
        "fmod directory not found; set FMOD_SYS_FMOD_DIRECTORY to the path of the fmod installation"
    );
}

#[cfg(not(windows))]
fn find_fmod_directory() -> PathBuf {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let out_path = out_dir.join("fmod");
    if out_path.exists() {
        return out_path;
    }

    println!("cargo:rerun-if-env-changed=FMOD_SYS_FMOD_DIRECTORY");
    if let Some(fmod_dir) = std::env::var_os("FMOD_SYS_FMOD_DIRECTORY") {
        let fmod_dir = PathBuf::from(fmod_dir);
        if !fmod_dir.is_absolute() {
            panic!(
                "FMOD_SYS_FMOD_DIRECTORY should be an absolute path, but it is a relative path: {}",
                fmod_dir.display()
            );
        }
        if fmod_dir.exists() {
            return fmod_dir;
        } else {
            panic!(
                "FMOD_SYS_FMOD_DIRECTORY set to {fmod_dir:?}, but fmod directory not found there",
            );
        }
    }

    // try to find it in _our_ crate root (useful for hacking on this fmod-audio-sys crate)
    let in_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fmod");
    if in_dir.exists() {
        return in_dir;
    }

    panic!(
        "fmod directory not found; set FMOD_SYS_FMOD_DIRECTORY to the path of the fmod installation"
    );
}

fn main() {
    // skip generating bindings in docs.rs, as we use the packaged "documentation.rs" instead
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }
    #[cfg(feature = "force-docs-bindings")]
    return;

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    let docs_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("docs");
    fs::create_dir_all(&docs_dir).expect("Failed to create docs directory");

    let build_is_windows = std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|env| env == "windows");
    let build_is_wasm = std::env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|env| env == "wasm32");
    let build_is_emscripten =
        std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|env| env == "emscripten");
    let build_is_macos = std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|env| env == "macos");
    let build_is_linux = std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|env| env == "linux");

    let cross_compile_api_dir = if build_is_windows {
        Some("windows")
    } else if build_is_wasm {
        Some("html5")
    } else if build_is_macos {
        Some("macos")
    } else if build_is_linux {
        Some("linux")
    } else {
        None
    };

    let build_is_x86 = std::env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|env| env == "x86");
    let build_is_x86_64 = std::env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|env| env == "x86_64");
    let build_is_arm = std::env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|env| env == "arm");
    let build_is_arm64 = std::env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|env| env == "aarch64");
    let fmod_dir = find_fmod_directory();
    assert!(fmod_dir.exists(), "fmod directory not present");

    let mut api_dir = None;
    if let Some(cross_compile_api_dir) = cross_compile_api_dir {
        let maybe_api_dir = fmod_dir.join(cross_compile_api_dir).join("api");
        // if it does, target that
        if maybe_api_dir.exists() {
            api_dir = Some(maybe_api_dir);
        }
    }
    // otherwise just default to fmod/api
    let api_dir = api_dir.unwrap_or_else(|| fmod_dir.join("api"));

    assert!(api_dir.exists(), "fmod api dir does not exist");

    let api_dir_display = api_dir.display();
    println!("cargo:rerun-if-changed={api_dir_display}/core/inc");
    println!("cargo:rerun-if-changed={api_dir_display}/studio/inc");

    let mut bindgen = bindgen::builder()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(VersionCallbacks))
        .clang_arg(format!("-I{api_dir_display}/core/inc"))
        .newtype_enum("FMOD_RESULT")
        .must_use_type("FMOD_RESULT")
        .new_type_alias("FMOD_BOOL")
        .derive_partialeq(true)
        .derive_eq(true)
        .impl_partialeq(true)
        .derive_hash(true)
        .derive_default(true)
        .prepend_enum_name(false) // fmod already does this
        .header("src/wrapper.h");

    #[cfg(feature = "studio")]
    {
        bindgen = bindgen
            .clang_arg(format!("-I{api_dir_display}/studio/inc"))
            .clang_arg("-DFMOD_STUDIO_ENABLED");
    }
    let include_debug = cfg!(any(debug_assertions, feature = "force-debug"));
    let debug_char = if include_debug { "L" } else { "" };

    if build_is_wasm {
        bindgen = bindgen.clang_arg("-fvisibility=default")
    }

    // On macOS the fmod library uses @rpath to find the dylib and the following doesn't work:
    // println!("cargo:rustc-link-args='-rpath {api_dir_display}/core/lib'");
    // Therefore, as workaround, copy the libraries to OUT_DIR before the build.
    // Note: you will probably have to run `xattr -d com.apple.quarantine` on all the `.dylib`s
    // in the fmod installation folder.
    #[cfg(feature = "link-fmod")]
    if build_is_macos {
        let corelib = format!("libfmod{debug_char}.dylib");
        fs::copy(
            api_dir.join("core").join("lib").join(&corelib),
            out_dir.join(&corelib),
        )
        .expect("failed to copy core lib");

        let studiolib = format!("libfmodstudio{debug_char}.dylib");
        fs::copy(
            api_dir.join("studio").join("lib").join(&studiolib),
            out_dir.join(&studiolib),
        )
        .expect("failed to copy studio lib");
    }

    // due to some weird shenanigans I can't figure out how to turn off, the linker searches for lib<library name> instead of just accepting the library name
    #[cfg(feature = "link-fmod")]
    if build_is_wasm {
        let old_lib_path = format!("studio/lib/upstream/w32/fmodstudio{debug_char}_wasm.a");
        let new_lib_path = format!("studio/lib/upstream/w32/libfmodstudio{debug_char}_wasm.a");
        fs::copy(api_dir.join(old_lib_path), api_dir.join(new_lib_path))
            .expect("failed to copy studio lib");
    }

    // FIXME: We should be setting this var ourselves.
    // Using std::env::set_var doesn't work, nor does doing it through cargo:rustc-env.
    #[cfg(feature = "link-fmod")]
    if build_is_emscripten {
        let needed_emcc_flags = "-s EXPORTED_RUNTIME_METHODS=ccall,cwrap,setValue,getValue";
        let has_needed_args = match std::env::var("EMCC_CFLAGS") {
            Ok(value) => value.contains(needed_emcc_flags),
            Err(_) => false,
        };
        if !has_needed_args {
            println!("cargo::error=EMCC_CFLAGS must include {needed_emcc_flags:?}!")
        }
    }

    #[cfg(feature = "link-fmod")]
    if build_is_wasm {
        // studio includes core on this platform, so no need to link against it
        println!("cargo:rustc-link-search={api_dir_display}/studio/lib/upstream/w32");
    } else if build_is_macos {
        println!("cargo:rustc-link-search={api_dir_display}/core/lib");
        println!("cargo:rustc-link-search={api_dir_display}/studio/lib");
    } else {
        let target_arch = if build_is_x86_64 && !build_is_windows {
            "x86_64"
        } else if build_is_x86_64 {
            "x64"
        } else if build_is_x86 {
            "x86"
        } else if build_is_arm64 {
            "arm64"
        } else if build_is_arm {
            "arm"
        } else {
            todo!()
        };
        println!("cargo:rustc-link-search={api_dir_display}/core/lib/{target_arch}");
        println!("cargo:rustc-link-search={api_dir_display}/studio/lib/{target_arch}");
    }

    #[cfg(feature = "link-fmod")]
    if build_is_wasm {
        #[cfg(not(feature = "studio"))]
        // studio includes core on this platform, so no need to link against it
        println!("cargo:rustc-link-lib=fmod{debug_char}_wasm");
        #[cfg(feature = "studio")]
        // studio includes core on this platform, so no need to link against it
        println!("cargo:rustc-link-lib=fmodstudio{debug_char}_wasm");
    } else if build_is_windows {
        println!("cargo:rustc-link-lib=fmod{debug_char}_vc");
        #[cfg(feature = "studio")]
        println!("cargo:rustc-link-lib=fmodstudio{debug_char}_vc");
    } else {
        println!("cargo:rustc-link-lib=fmod{debug_char}");
        #[cfg(feature = "studio")]
        println!("cargo:rustc-link-lib=fmodstudio{debug_char}");
    }

    let bindings = bindgen.generate().expect("failed to generate bindings");
    let out_path = out_dir.join("bindings.rs");

    bindings
        .write_to_file(out_path)
        .expect("failed to write bindings");

    let version_number = DETECTED_VERSION.load(std::sync::atomic::Ordering::Relaxed);
    if version_number == i64::MAX {
        panic!("Failed to determine FMOD version!");
    }

    let minor = version_number & 0xFF;
    let major = (version_number >> 8) & 0xFF;
    let product = version_number >> 16;

    println!("cargo::metadata=version_number={version_number}");
    println!("cargo::metadata=minor={minor}");
    println!("cargo::metadata=minor={major}");
    println!("cargo::metadata=minor={product}");

    println!("cargo::rustc-env=FMOD_DIR={}", fmod_dir.display());
    println!("cargo::rustc-env=FMOD_API_DIR={}", api_dir.display());

    let docs_path = docs_dir.join("documentation.rs");

    bindings
        .write_to_file(docs_path)
        .expect("failed to write docs");

    println!("cargo:rerun-if-changed=src/channel_control.cpp");
    println!("cargo:rerun-if-changed=src/channel_control.h");

    // wrapper does not use the stdlib
    let mut build = cc::Build::new();

    build
        .cpp(true)
        .cpp_link_stdlib(None)
        .cpp_set_stdlib(None)
        .include(format!("{api_dir_display}/core/inc"))
        .file("src/channel_control.cpp");

    if build_is_emscripten {
        build.flag_if_supported("-Wunused-command-line-argument"); // why is this raised?
    }

    if build_is_windows {
        let target = if build_is_x86_64 {
            "x86_64-pc-windows-msvc"
        } else if build_is_x86 {
            "i686-pc-windows-msvc"
        } else {
            todo!()
        };
        let tool = cc::windows_registry::find_tool(target, "cl.exe").expect("failed to find cl");
        build.compiler(tool.path());
    }

    build.compile("channel_control_wrapper");
}
