use std::{env, path::PathBuf, process::Command};

#[derive(Debug)]
struct Callbacks;

// NOTE: Capability bits don't fit in an MSVC enum, so give them the flags type instead of whatever bindgen infers from each value.
impl bindgen::callbacks::ParseCallbacks for Callbacks {
    fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
        name.starts_with("NagaCapabilities_")
            .then(|| bindgen::callbacks::IntKind::Custom {
                name: "NagaCapabilitiesFlags",
                is_signed: false,
            })
    }

    fn item_name(&self, item_info: bindgen::callbacks::ItemInfo) -> Option<String> {
        (matches!(item_info.kind, bindgen::callbacks::ItemKind::Var)
            && item_info.name.starts_with("NagaCapabilities_"))
        .then(|| format!("NagaCapabilities_{}", item_info.name))
    }
}

fn main() {
    let mut builder = bindgen::Builder::default()
        .header("naga.h")
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(Callbacks))
        .clang_arg("-DNAGA_FFI_NO_METHODS")
        // HACK: Well... we shouldn't need to do this.
        .blocklist_item("max_align_t");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("emscripten") {
        let output = if cfg!(windows) {
            Command::new("cmd")
                .args(["/C", "em-config", "CACHE"])
                .output()
        } else {
            Command::new("em-config").arg("CACHE").output()
        };
        match output {
            Ok(output) if output.status.success() => {
                let cache =
                    String::from_utf8(output.stdout).expect("em-config output is not UTF-8");
                builder = builder.clang_arg(format!("--sysroot={}/sysroot", cache.trim()));
            }
            _ => println!(
                "cargo:warning=Couldn't run `em-config CACHE`; is emsdk activated? Falling back to BINDGEN_EXTRA_CLANG_ARGS."
            ),
        }
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
