// This is free and unencumbered software released into the public domain.

use cfg_aliases::cfg_aliases;

fn main() {
    // See: https://github.com/katharostech/cfg_aliases
    cfg_aliases! {
        android: { target_os = "android" },
        darwin: { any(
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos")
        },
        ios: { target_os = "ios" },
        linux: { target_os = "linux" },
        macos: { target_os = "macos" },
        tvos: { target_os = "tvos" },
        wasm: { target_family = "wasm" },
        watchos: { target_os = "watchos" },
    }
}
