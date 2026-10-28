//! The shared libraries the system has.

use std::collections::BTreeSet;

use substrate_installer_domain::platforms::SystemLibrary;
use substrate_process::command;

/// Where `ldconfig` lives when it is not on `PATH` (it is `root`'s).
const LDCONFIG_FALLBACK: &str = "/sbin/ldconfig";

/// The libraries `ldconfig -p` lists; empty off Linux and where `ldconfig`
/// cannot be run. The installer only asks for the few a catalog names, so a
/// system it cannot scan is treated as lacking them.
#[must_use]
pub fn scan_libraries() -> BTreeSet<SystemLibrary> {
    if !cfg!(target_os = "linux") {
        return BTreeSet::new();
    }
    ["ldconfig", LDCONFIG_FALLBACK]
        .into_iter()
        .find_map(|program| command(program).arg("-p").output().ok())
        .filter(|output| output.status.success())
        .map(|output| parse_library_cache(&String::from_utf8_lossy(&output.stdout)))
        .unwrap_or_default()
}

/// The libraries in the text `ldconfig -p` prints: each line is a soname,
/// its flags and the path (`libgtk-3.so.0 (libc6,x86-64) => /usr/lib/...`);
/// the first line (`2048 libs found in cache ...`) is a count.
#[must_use]
pub fn parse_library_cache(text: &str) -> BTreeSet<SystemLibrary> {
    text.lines()
        .filter_map(|line| line.split_whitespace().next())
        .filter(|word| word.contains(".so"))
        .filter_map(|word| SystemLibrary::new(word).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const CACHE: &str = "2 libs found in cache `/etc/ld.so.cache'\n\
        \tlibgtk-3.so.0 (libc6,x86-64) => /usr/lib/x86_64-linux-gnu/libgtk-3.so.0\n\
        \tlibsoup-3.0.so.0 (libc6,x86-64) => /usr/lib/x86_64-linux-gnu/libsoup-3.0.so.0\n";

    #[test]
    fn it_reads_the_sonames_and_skips_the_count() {
        let libraries = parse_library_cache(CACHE);
        let names: Vec<_> = libraries.iter().map(SystemLibrary::as_str).collect();
        assert_eq!(names, ["libgtk-3.so.0", "libsoup-3.0.so.0"]);
    }

    #[test]
    fn nothing_listed_is_nothing_found() {
        assert!(parse_library_cache("").is_empty());
    }
}
