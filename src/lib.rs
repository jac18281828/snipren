/**
 * Match if `new` is an expansion of `old`, meaning that `new` starts with the start of `old`
 *
 *
 * Examples:
 * old: route_report.csv
 * new: route_report_before.csv
 * -> match
 *
 * old: route-report.csv
 * new: route-report-2023-01.csv
 * -> match
 *
 * old: route.report.csv
 * new: route.report.v2.csv
 * -> match
 *
 * old: route_report.csv
 * new: route-report_before.csv
 * -> no match (different separator)
 *
 * old: route_report.csv
 * new: route_report.csv.bak
 * -> no match (different ending)
 */

pub fn matches_expansion(old: &str, new: &str) -> bool {
    // Two-pointer approach: squeeze from both ends
    // i1, i2: forward pointers
    // j1, j2: backward pointers (at extension boundary)

    // Find extension positions (last dot)
    let j1 = old.rfind('.').unwrap_or(old.len());
    let j2 = new.rfind('.').unwrap_or(new.len());

    // If new_base equals old (just adding extension like .bak)
    if &new[..j2] == old {
        return false;
    }

    // The old base must be shorter than new base for expansion
    if j1 >= j2 {
        return false;
    }

    // Forward scan: old base must be prefix of new base
    let old_base = &old[..j1];
    let new_base = &new[..j2];

    // New base must start with old base AND be longer (expansion)
    new_base.starts_with(old_base) && new_base.len() > old_base.len()
}

#[cfg(test)]
mod tests {
    use super::matches_expansion;

    #[test]
    fn test_expansion_with_underscore() {
        assert!(matches_expansion(
            "route_report.csv",
            "route_report_before.csv"
        ));
        assert!(matches_expansion("data.json", "data_backup.json"));
        assert!(matches_expansion("file.txt", "file_v2.txt"));
    }

    #[test]
    fn test_expansion_with_dash() {
        assert!(matches_expansion(
            "route-report.csv",
            "route-report-2023-01.csv"
        ));
        assert!(matches_expansion("config-dev.yml", "config-dev-local.yml"));
    }

    #[test]
    fn test_expansion_with_dot() {
        assert!(matches_expansion("route.report.csv", "route.report.v2.csv"));
        assert!(matches_expansion("app.config.json", "app.config.prod.json"));
    }

    #[test]
    fn test_expansion_without_separator() {
        // No separator boundary requirement - these should match
        // Safety comes from "exactly one file matches" requirement
        assert!(matches_expansion("route.csv", "router.csv"));
        assert!(matches_expansion("test.txt", "testing.txt"));
        assert!(matches_expansion("file.md", "filename.md"));
    }

    #[test]
    fn test_not_expansion_just_extension_change() {
        // Same base, different extension - not an expansion
        assert!(!matches_expansion("route_report.csv", "route_report.txt"));
        assert!(!matches_expansion("data.json", "data.yaml"));
        assert!(!matches_expansion("file.md", "file.txt"));
    }

    #[test]
    fn test_not_expansion_adding_extension_only() {
        // Adding extension to full filename - not an expansion
        assert!(!matches_expansion(
            "route_report.csv",
            "route_report.csv.bak"
        ));
        assert!(!matches_expansion("data.json", "data.json.old"));
        assert!(!matches_expansion("script.sh", "script.sh.backup"));
    }

    #[test]
    fn test_not_expansion_different_prefix() {
        // New doesn't start with old - not an expansion
        assert!(!matches_expansion("route_report.csv", "new_report.csv"));
        assert!(!matches_expansion("data.json", "metadata.json"));
        assert!(!matches_expansion("file.txt", "other.txt"));
    }

    #[test]
    fn test_not_expansion_mixing_separators() {
        // Changing separator type in the middle is still not a prefix match
        assert!(!matches_expansion(
            "route_report.csv",
            "route-report_before.csv"
        ));
        assert!(!matches_expansion("test_file.txt", "test-file_new.txt"));
    }

    #[test]
    fn test_not_expansion_shorter_base() {
        // New base is shorter than old base - not an expansion
        assert!(!matches_expansion("route_report_v2.csv", "route.csv"));
        assert!(!matches_expansion("long_filename.txt", "long.txt"));
    }

    #[test]
    fn test_expansion_no_extension() {
        // Files without extensions
        assert!(matches_expansion("Makefile", "Makefile_backup"));
        assert!(matches_expansion("README", "README_old"));
        assert!(!matches_expansion("LICENSE", "NOTICE"));
    }

    #[test]
    fn test_expansion_multiple_dots_in_name() {
        // Multiple dots in filename (not all are extensions)
        assert!(matches_expansion("app.config.json", "app.config.prod.json"));
        assert!(matches_expansion(
            "file.tar.gz",
            "file.tar.gz.backup.tar.gz"
        ));
    }

    #[test]
    fn test_not_expansion_identical_files() {
        // Identical filenames should not match
        assert!(!matches_expansion("file.txt", "file.txt"));
        assert!(!matches_expansion("data.json", "data.json"));
    }

    #[test]
    fn test_expansion_edge_cases() {
        // Single character expansions
        assert!(matches_expansion("a.txt", "ab.txt"));
        assert!(matches_expansion("x.csv", "xy.csv"));

        // Very long expansions
        assert!(matches_expansion(
            "f.txt",
            "f_with_very_long_suffix_added.txt"
        ));
    }

    #[test]
    fn test_not_expansion_substring_but_different_extension_position() {
        // New contains old as substring but extension differs
        assert!(!matches_expansion("test.old.csv", "test.csv"));
        assert!(!matches_expansion("file.backup.txt", "file.txt"));
    }
}
