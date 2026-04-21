# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Contributors to Eclipse OpenSOVD (see CONTRIBUTORS)

load("@cda_crates//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")

# ---------------------------------------------------------------------------
# Build script
# Embeds GIT_COMMIT_HASH and BUILD_DATE into the binary at compile time.
#
# In the Bazel sandbox `git` is not available.  Pass overrides via env:
#   - SOURCE_GIT_SHA  (e.g. set from Bazel workspace-status stamping)
#   - SOURCE_DATE_EPOCH (Unix timestamp)
#
# For a reproducible CI build you can add to .bazelrc:
#   build --workspace_status_command=scripts/workspace_status.sh
# and in that script emit:
#   echo "SOURCE_DATE_EPOCH $(date +%s)"
#   echo "SOURCE_GIT_SHA $(git rev-parse --short HEAD)"
# ---------------------------------------------------------------------------
cargo_build_script(
    name = "cda_main_build_script",
    srcs = ["build.rs"],
    build_script_env = {
        # Fallback values used when Bazel workspace stamping is not configured.
        # Override by setting the env vars listed above in your .bazelrc.
        "SOURCE_DATE_EPOCH": "0",
        "SOURCE_GIT_SHA": "bazel-build",
    },
    crate_name = "build_script_build",
    data = [
        # The build script checks for .git/HEAD to decide when to re-run;
        # expose it as a data dep when present so incremental builds work.
    ],
    edition = "2024",
    deps = all_crate_deps(build = True),
)

# Library target (used by integration-tests crate)
rust_library(
    name = "opensovd_cda_lib",
    srcs = glob(["src/**/*.rs"]),
    aliases = aliases(),
    crate_features = [
        "health",
        "openssl",
    ],
    crate_name = "opensovd_cda_lib",
    edition = "2024",
    proc_macro_deps = all_crate_deps(
        normal_dev = False,
        proc_macro = True,
    ),
    visibility = ["//visibility:public"],
    deps = all_crate_deps(normal = True) + [
        ":cda_main_build_script",
        "//cda-comm-doip:cda-comm-doip",
        "//cda-comm-uds:cda-comm-uds",
        "//cda-core:cda-core",
        "//cda-database:cda-database",
        "//cda-health:cda-health",
        "//cda-interfaces:cda-interfaces",
        "//cda-plugin-security:cda-plugin-security",
        "//cda-sovd:cda-sovd",
        "//cda-tracing:cda-tracing",
    ],
)

# Main executable
rust_binary(
    name = "opensovd-cda",
    srcs = ["src/main.rs"],
    aliases = aliases(),
    crate_features = [
        "health",
        "openssl",
    ],
    crate_name = "opensovd_cda",
    edition = "2024",
    proc_macro_deps = all_crate_deps(
        normal_dev = False,
        proc_macro = True,
    ),
    visibility = ["//visibility:public"],
    deps = all_crate_deps(normal = True) + [
        ":cda_main_build_script",
        ":opensovd_cda_lib",
        "//cda-comm-doip:cda-comm-doip",
        "//cda-comm-uds:cda-comm-uds",
        "//cda-core:cda-core",
        "//cda-database:cda-database",
        "//cda-health:cda-health",
        "//cda-interfaces:cda-interfaces",
        "//cda-plugin-security:cda-plugin-security",
        "//cda-sovd:cda-sovd",
        "//cda-tracing:cda-tracing",
    ],
)

exports_files(["Cargo.toml"])
