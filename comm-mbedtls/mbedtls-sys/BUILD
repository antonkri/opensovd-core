# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Contributors to Eclipse OpenSOVD (see CONTRIBUTORS)

# NOTE: This crate is OPTIONAL.  It is only needed when the `mbedtls` feature
# is enabled on cda-comm-doip.  The default build uses openssl instead.
#
# mbedtls-sys downloads and compiles the mbedtls C library via build.rs.
# Integrating that into Bazel requires:
#   1. Providing the mbedtls source tree (e.g. via http_archive in MODULE.bazel).
#   2. Writing a cc_library for the C source, or letting cargo_build_script
#      compile it (requires network access or pre-fetched sources in the sandbox).
#
# Stub target – uncomment and fill in when mbedtls support is needed.
#
# load("@rules_rust//cargo:defs.bzl", "cargo_build_script")
# load("@rules_rust//rust:defs.bzl", "rust_library")
#
# cargo_build_script(
#     name = "mbedtls_sys_build_script",
#     srcs  = ["build.rs"],
#     data  = glob(["csrc/**", "patches/**", "wrapper.h"]),
#     build_script_env = {
#         # Point at a local extract of the mbedtls source if network is
#         # unavailable inside the Bazel sandbox.
#         # "MBEDTLS_DIR": "/path/to/mbedtls-4.0.0",
#     },
#     edition = "2024",
#     deps = ["@cda_crates//:cc", "@cda_crates//:const_format", "@cda_crates//:libc"],
# )
#
# rust_library(
#     name = "mbedtls-sys",
#     srcs  = glob(["src/**/*.rs"]),
#     edition = "2024",
#     crate_name = "mbedtls_sys",
#     visibility = ["//visibility:public"],
#     deps = [":mbedtls_sys_build_script", "@cda_crates//:libc"],
# )

exports_files(["Cargo.toml"])
