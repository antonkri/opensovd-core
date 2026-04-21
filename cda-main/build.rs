/*
 * SPDX-License-Identifier: Apache-2.0
 * SPDX-FileCopyrightText: 2025 The Contributors to Eclipse OpenSOVD (see CONTRIBUTORS)
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Apache License Version 2.0 which is available at
 * https://www.apache.org/licenses/LICENSE-2.0
 */

/*
 * SPDX-License-Identifier: Apache-2.0
 * SPDX-FileCopyrightText: 2025 The Contributors to Eclipse OpenSOVD (see CONTRIBUTORS)
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Apache License Version 2.0 which is available at
 * https://www.apache.org/licenses/LICENSE-2.0
 */

use std::{env, process::Command};

fn main() {
    // Re-run on local changes or new commits
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");

    let build_date = if let Ok(source_date_epoch) = env::var("SOURCE_DATE_EPOCH") {
        // During integration tests, allow invalid or missing SOURCE_DATE_EPOCH.
        // Especially when running the tests in gitlab CI, the variable
        // maybe defined through the integration tests but is empty.
        // As it doesn't matter for the tests, we assume a default.
        // For non-test build, we want to ensure that the variable is valid.
        #[cfg(feature = "integration-tests")]
        let epoch = source_date_epoch.parse::<i64>().unwrap_or({
            println!("cargo:warning=SOURCE_DATE_EPOCH not specified, using empty");
            0i64
        });
        #[cfg(not(feature = "integration-tests"))]
        let epoch = source_date_epoch.parse::<i64>().expect(&format!(
            "SOURCE_DATE_EPOCH is not a valid integer: {source_date_epoch}"
        ));
        epoch_to_iso8601(epoch)
    } else {
        get_git_date()
    };

    let commit_hash_str = if let Ok(sha) = env::var("SOURCE_GIT_SHA") {
        sha
    } else {
        match Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
        {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout).trim().to_owned()
            }
            _ => {
                panic!("Failed to get commit hash");
            }
        }
    };

    // Make env variables available to the crate
    println!("cargo:rustc-env=BUILD_DATE={build_date}");
    println!("cargo:rustc-env=GIT_COMMIT_HASH={commit_hash_str}");
}

/// Convert a Unix timestamp (seconds since 1970-01-01 00:00:00 UTC) to an
/// ISO 8601 string (`YYYY-MM-DDTHH:MM:SSZ`). Pure stdlib, no external crates.
fn epoch_to_iso8601(epoch: i64) -> String {
    let secs_per_day: i64 = 86400;
    let time_of_day = epoch.rem_euclid(secs_per_day);
    let days = epoch.div_euclid(secs_per_day);
    let hour = time_of_day / 3600;
    let min = (time_of_day % 3600) / 60;
    let sec = time_of_day % 60;
    let (year, month, day) = days_to_ymd(days);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{min:02}:{sec:02}Z")
}

/// Convert days since the Unix epoch (1970-01-01) to a (year, month, day) triple.
/// Uses the algorithm from <https://howardhinnant.github.io/date_algorithms.html>.
fn days_to_ymd(z: i64) -> (i32, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = y + if m <= 2 { 1 } else { 0 };
    (y as i32, m, d)
}

fn get_git_date() -> String {
    let git_output = Command::new("git")
        .args(["log", "-1", "--format=%aI"])
        .output()
        .expect("Failed to get build date via git");

    assert!(
        git_output.status.success(),
        "Git command failed: {}",
        String::from_utf8_lossy(&git_output.stderr)
    );

    // git --format=%aI outputs strict ISO 8601 / RFC 3339, e.g.
    // "2025-04-20T12:34:56+02:00" or "2025-04-20T12:34:56Z".
    // We keep the local time but normalise the suffix to Z for the
    // display string (this is build metadata, not safety-critical).
    let git_date = String::from_utf8_lossy(&git_output.stdout)
        .trim()
        .to_owned();
    if git_date.len() >= 19 {
        format!("{}Z", &git_date[..19])
    } else {
        git_date
    }
}
