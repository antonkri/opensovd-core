# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Contributors to Eclipse OpenSOVD (see CONTRIBUTORS)
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
#
# This program and the accompanying materials are made available under the
# terms of the Apache License Version 2.0 which is available at
# https://www.apache.org/licenses/LICENSE-2.0

exports_files([
    "Cargo.toml",
    "Cargo.lock",
    "cargo-bazel-lock.json",
    "integration-tests/Cargo.toml",
])

# Alias for the main CDA binary so you can run:
#   bazel build //:opensovd-cda
#   bazel run   //:opensovd-cda -- --databases-path ./testcontainer/odx
alias(
    name = "opensovd-cda",
    actual = "//cda-main:opensovd-cda",
    visibility = ["//visibility:public"],
)
