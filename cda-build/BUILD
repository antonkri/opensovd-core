# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Contributors to Eclipse OpenSOVD (see CONTRIBUTORS)

load("@rules_rust//rust:defs.bzl", "rust_library")

# cda-build is a *build-time helper library* (used in build.rs scripts of other
# crates).  It has no external runtime dependencies.
rust_library(
    name = "cda-build",
    srcs = glob(["src/**/*.rs"]),
    edition = "2024",
    visibility = ["//visibility:public"],
    proc_macro_deps = [
        "@cda_crates//:rustversion",
    ],
)

exports_files(["Cargo.toml"])
