/**
 * Match if `new` is an expansion of `old`, meaning that characters are added
 * in the middle or at the end of `old` to create `new`.
 *
 * Uses a two-pointer "vice" algorithm that squeezes from both ends:
 * - Forward pointers find where strings start to differ
 * - Backward pointers find where strings start to differ from the end
 * - Valid expansion: all of `old` is consumed, with added text in middle or end
 *
 * Examples:
 * old: route_report.csv
 * new: route_report_before.csv
 * -> match (expansion in middle)
 *
 * old: route-report.csv
 * new: route-report-2023-01.csv
 * -> match (expansion in middle)
 *
 * old: data.json
 * new: data.json_backup
 * -> match (expansion at end - backup pattern)
 *
 * old: route_report.csv
 * new: route-report_before.csv
 * -> no match (different separator)
 *
 * old: data.json
 * new: metadata.json
 * -> no match (no prefix match - expansion at start)
 */
pub fn matches_expansion(old: &str, new: &str) -> bool {
    // Two-pointer "vice" approach: squeeze from both ends
    // i1: pointer moving forward in old
    // i2: pointer moving forward in new
    // j1: pointer moving backward in old (starts at end)
    // j2: pointer moving backward in new (starts at end)

    let old_chars: Vec<char> = old.chars().collect();
    let new_chars: Vec<char> = new.chars().collect();
    let old_len = old_chars.len();
    let new_len = new_chars.len();

    // New must be longer than old for expansion
    if new_len <= old_len {
        return false;
    }

    // Forward scan: find where characters start to differ
    let mut i1 = 0;
    let mut i2 = 0;

    while i1 < old_len && i2 < new_len && old_chars[i1] == new_chars[i2] {
        i1 += 1;
        i2 += 1;
    }

    // Backward scan: find where characters start to differ from the end
    let mut j1 = old_len;
    let mut j2 = new_len;

    while j1 > i1 && j2 > i2 && old_chars[j1 - 1] == new_chars[j2 - 1] {
        j1 -= 1;
        j2 -= 1;
    }

    // Requirements for valid expansion:
    // 1. i1 == j1: All of old was consumed (no unmatched middle section in old)
    // 2. i1 > 0: Must have some prefix match (expansion not at the very start)
    // This allows expansion either in the middle or at the end, but not at the start

    i1 == j1 && i1 > 0
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
    fn test_expansion_after_extension() {
        // Adding suffix after extension - valid expansion pattern for backups
        assert!(matches_expansion(
            "route_report.csv",
            "route_report.csv_backup"
        ));
        assert!(matches_expansion("data.json", "data.json_old"));
        assert!(matches_expansion("script.sh", "script.sh_backup"));
        assert!(matches_expansion("config.yml", "config.yml_2024"));
    }

    #[test]
    fn test_not_expansion_adding_dotted_extension() {
        // Adding .extension (with dot) is still allowed as it's a prefix match
        // The tool relies on "exactly one match" to prevent ambiguity
        assert!(matches_expansion(
            "route_report.csv",
            "route_report.csv.bak"
        ));
        assert!(matches_expansion("data.json", "data.json.old"));
    }

    #[test]
    fn test_not_expansion_different_prefix() {
        // New doesn't start with old - not an expansion (no prefix match)
        assert!(!matches_expansion("route_report.csv", "new_report.csv"));
        assert!(!matches_expansion("data.json", "metadata.json"));
        assert!(!matches_expansion("file.txt", "other.txt"));
        assert!(!matches_expansion("test.log", "production_test.log"));
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

    #[test]
    fn test_real_world_bombas_debug_case() {
        // Real-world bug: bombas_debug_main.log should match bombas_debug.log
        assert!(matches_expansion(
            "bombas_debug.log",
            "bombas_debug_main.log"
        ));

        // But should NOT match these unrelated files
        assert!(!matches_expansion(".dockerignore", "bombas_debug_main.log"));
        assert!(!matches_expansion(".gitignore", "bombas_debug_main.log"));
        assert!(!matches_expansion(".env", "bombas_debug_main.log"));

        // Test the reverse direction - longer file compared to shorter should not match
        assert!(!matches_expansion(
            "bombas_debug_main.log",
            "bombas_debug.log"
        ));
        assert!(!matches_expansion("bombas_debug_main.log", ".dockerignore"));
        assert!(!matches_expansion("bombas_debug_main.log", ".gitignore"));
        assert!(!matches_expansion("bombas_debug_main.log", ".env"));
    }

    #[test]
    fn test_unicode_filenames() {
        // Unicode characters in filenames
        assert!(matches_expansion("データ.txt", "データ_backup.txt"));
        assert!(matches_expansion("файл.log", "файл_main.log"));
        assert!(matches_expansion("文件.csv", "文件_最终.csv"));
        assert!(matches_expansion("🎉.json", "🎉_party.json"));

        // Should not match different Unicode prefixes
        assert!(!matches_expansion("データ.txt", "メタデータ.txt"));
        assert!(!matches_expansion("файл.log", "новыйфайл.log"));
    }
}
