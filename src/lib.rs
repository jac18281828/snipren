/**
 * Match if two filenames differ only by extension.
 *
 * This handles the specific case where the base name (before the last dot)
 * is identical and only the extension differs.
 *
 * Examples:
 * old: data.json
 * new: data.csv
 * -> match (same base "data", different extension)
 *
 * old: report.txt
 * new: report.md
 * -> match (same base "report", different extension)
 *
 * old: file.tar.gz
 * new: file.tar.bz2
 * -> match (same base "file.tar", different extension)
 *
 * old: all_aero_pools.txt
 * new: all_aero_pools.csv
 * -> match (same base, different extension)
 *
 * old: data.json
 * new: metadata.json
 * -> no match (different base)
 *
 * Note: This may overlap with matches_expansion in some cases (e.g., config.yml
 * -> config.yaml), but that's OK - we use OR logic in the rename tool.
 */
pub fn matches_extension_change(old: &str, new: &str) -> bool {
    // Can't be the same file
    if old == new {
        return false;
    }

    // Both must have extensions (at least one dot)
    let old_dot_pos = old.rfind('.');
    let new_dot_pos = new.rfind('.');

    // If either has no dot, this is not an extension change
    if old_dot_pos.is_none() || new_dot_pos.is_none() {
        return false;
    }

    let old_dot = old_dot_pos.unwrap();
    let new_dot = new_dot_pos.unwrap();

    // Extract base names (everything before the last dot)
    let old_base = &old[..old_dot];
    let new_base = &new[..new_dot];

    // Base names must be identical
    if old_base != new_base {
        return false;
    }

    // Extensions must differ
    let old_ext = &old[old_dot..];
    let new_ext = &new[new_dot..];

    old_ext != new_ext
}

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
    use super::{matches_expansion, matches_extension_change};

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
        // Same base, different extension - not an expansion (but is an extension change)
        assert!(!matches_expansion("route_report.csv", "route_report.txt"));
        assert!(!matches_expansion("data.json", "data.yaml"));
        assert!(!matches_expansion("file.md", "file.txt"));
    }

    #[test]
    fn test_extension_change() {
        // Same base, different extension - should match
        assert!(matches_extension_change(
            "route_report.csv",
            "route_report.txt"
        ));
        assert!(matches_extension_change("data.json", "data.yaml"));
        assert!(matches_extension_change("file.md", "file.txt"));
        assert!(matches_extension_change(
            "all_aero_pools.txt",
            "all_aero_pools.csv"
        ));
    }

    #[test]
    fn test_extension_change_no_extension() {
        // These are now expansions, not extension changes
        // Extension change requires both files to have extensions
        assert!(!matches_extension_change("Makefile", "Makefile.bak"));
        assert!(!matches_extension_change("README", "README.md"));
        assert!(!matches_extension_change("LICENSE.txt", "LICENSE"));

        // But they should match as expansions
        assert!(matches_expansion("Makefile", "Makefile.bak"));
        assert!(matches_expansion("README", "README.md"));
    }

    #[test]
    fn test_extension_change_complex_extensions() {
        // Complex/multiple dot extensions
        assert!(matches_extension_change("file.tar.gz", "file.tar.bz2"));
        assert!(matches_extension_change(
            "app.config.json",
            "app.config.yaml"
        ));
    }

    #[test]
    fn test_not_extension_change_different_base() {
        // Different base names - should not match
        assert!(!matches_extension_change("data.json", "metadata.json"));
        assert!(!matches_extension_change("test.txt", "testing.txt"));
        assert!(!matches_extension_change("file.csv", "file_v2.csv"));
    }

    #[test]
    fn test_not_extension_change_identical() {
        // Identical files should not match
        assert!(!matches_extension_change("file.txt", "file.txt"));
        assert!(!matches_extension_change("data.json", "data.json"));
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

    // ============================================================================
    // COVERAGE TESTS
    // These tests ensure that common rename patterns are covered by at least
    // one of the matching functions (expansion OR extension_change).
    // Some cases may match both - that's OK since we use OR logic in rn.rs.
    // ============================================================================

    #[test]
    fn test_coverage_extension_changes() {
        // Pure extension changes - should match extension_change
        // (may also match expansion in some cases like .yml -> .yaml)
        let test_cases = vec![
            ("file.txt", "file.csv"),
            ("data.json", "data.yaml"),
            ("report.md", "report.pdf"),
            ("image.png", "image.jpg"),
            ("archive.tar.gz", "archive.tar.bz2"),
            ("all_aero_pools.txt", "all_aero_pools.csv"),
        ];

        for (old, new) in test_cases {
            let matches_ext = matches_extension_change(old, new);
            let matches_exp = matches_expansion(old, new);

            assert!(
                matches_ext || matches_exp,
                "{} -> {} should match at least one function",
                old,
                new
            );

            // Should definitely match extension_change
            assert!(
                matches_ext,
                "{} -> {} should match extension_change",
                old, new
            );
        }
    }

    #[test]
    fn test_coverage_expansions() {
        // Pure expansions - should match expansion
        let test_cases = vec![
            ("file.txt", "file_backup.txt"),
            ("data.json", "data_v2.json"),
            ("report.csv", "report_final.csv"),
            ("test.log", "test_debug.log"),
            ("route.csv", "router.csv"),
            ("app.config.json", "app.config.prod.json"),
            ("file.txt", "file_new_version.txt"),
            ("data.json", "data.json_backup"), // expansion after extension
            ("script.sh", "script.sh.bak"),    // adding .bak after .sh
            ("Makefile", "Makefile.bak"),      // adding extension to file without one
            ("Makefile", "Makefile_backup"),   // adding suffix
            ("Makefile", "Makefile~"),         // backup pattern
            ("README", "README.md"),           // adding extension
        ];

        for (old, new) in test_cases {
            let matches_ext = matches_extension_change(old, new);
            let matches_exp = matches_expansion(old, new);

            assert!(
                matches_ext || matches_exp,
                "{} -> {} should match at least one function",
                old,
                new
            );

            // Should definitely match expansion
            assert!(matches_exp, "{} -> {} should match expansion", old, new);
        }
    }

    #[test]
    fn test_coverage_neither_matches() {
        // Some pairs should match NEITHER function
        let test_cases = vec![
            ("file.txt", "file.txt"),                 // identical
            ("data.json", "metadata.json"),           // different base
            ("test.log", "prod_test.log"),            // prefix addition (not expansion)
            ("old.csv", "new.csv"),                   // completely different
            ("route_report.csv", "route-report.csv"), // separator change (base differs before dot)
            ("long_file.txt", "long.txt"),            // shorter (not expansion)
        ];

        for (old, new) in test_cases {
            let matches_ext = matches_extension_change(old, new);
            let matches_exp = matches_expansion(old, new);

            assert!(
                !matches_ext && !matches_exp,
                "{} -> {} should NOT match any function (ext={}, exp={})",
                old,
                new,
                matches_ext,
                matches_exp
            );
        }
    }

    #[test]
    fn test_coverage_comprehensive() {
        // Test that all common patterns are covered
        let test_cases = vec![
            // Extension changes
            ("file.txt", "file.csv", true),
            ("data.json", "data.yaml", true),
            ("config.yml", "config.yaml", true), // May match both
            // Expansions
            ("file.txt", "file_v2.txt", true),
            ("data.json", "data_backup.json", true),
            // Backup patterns
            ("Makefile", "Makefile.bak", true),
            ("Makefile", "Makefile~", true),
            ("README", "README.md", true),
            // Neither
            ("file.txt", "file.txt", false),
            ("data.json", "metadata.json", false),
            ("old.txt", "new.txt", false),
        ];

        for (old, new, should_match) in test_cases {
            let matches_ext = matches_extension_change(old, new);
            let matches_exp = matches_expansion(old, new);
            let matches_any = matches_ext || matches_exp;

            assert_eq!(
                matches_any,
                should_match,
                "{} -> {} should{} match (ext={}, exp={})",
                old,
                new,
                if should_match { "" } else { " NOT" },
                matches_ext,
                matches_exp
            );
        }
    }

    // ============================================================================
    // AMBIGUOUS CASE TESTS
    // Tests for edge cases and potential user mistakes documented in README
    // ============================================================================

    #[test]
    fn test_ambiguous_prefix_expansion_rejected() {
        // Prefix additions should NOT match (prevents metadata.json matching data.json)
        assert!(!matches_expansion("test.log", "production_test.log"));
        assert!(!matches_expansion("data.json", "metadata.json"));
        assert!(!matches_expansion("file.txt", "myfile.txt"));

        // These should also not match extension change (different base)
        assert!(!matches_extension_change("test.log", "production_test.log"));
        assert!(!matches_extension_change("data.json", "metadata.json"));
        assert!(!matches_extension_change("file.txt", "myfile.txt"));
    }

    #[test]
    fn test_ambiguous_substring_expansion_allowed() {
        // Substring expansions ARE allowed (test -> tester, test -> testing)
        // The "exactly one match" requirement in rn.rs prevents problems
        assert!(matches_expansion("test.txt", "testing.txt"));
        assert!(matches_expansion("test.txt", "tester.txt"));
        assert!(matches_expansion("file.md", "filename.md"));
        assert!(matches_expansion("route.csv", "router.csv"));
    }

    #[test]
    fn test_ambiguous_overlapping_patterns() {
        // Some cases match both patterns - this is OK, we use OR logic
        let overlapping_cases = vec![
            ("config.yml", "config.yaml"), // yml is prefix of yaml
        ];

        for (old, new) in overlapping_cases {
            let matches_ext = matches_extension_change(old, new);
            let matches_exp = matches_expansion(old, new);

            // Should match at least one
            assert!(
                matches_ext || matches_exp,
                "{} -> {} should match at least one pattern",
                old,
                new
            );
        }
    }

    #[test]
    fn test_ambiguous_separator_changes_rejected() {
        // Changing separator types should be rejected (different characters at same position)
        assert!(!matches_expansion("test_file.txt", "test-file.txt"));
        assert!(!matches_expansion("route_report.csv", "route.report.csv"));

        // Also not extension changes (base differs before dot)
        assert!(!matches_extension_change("test_file.txt", "test-file.txt"));
        assert!(!matches_extension_change(
            "route_report.csv",
            "route.report.csv"
        ));
    }

    #[test]
    fn test_ambiguous_no_extension_to_extension() {
        // Files without extension -> with extension should match expansion, not extension_change
        assert!(matches_expansion("README", "README.md"));
        assert!(!matches_extension_change("README", "README.md"));

        assert!(matches_expansion("Makefile", "Makefile.bak"));
        assert!(!matches_extension_change("Makefile", "Makefile.bak"));

        assert!(matches_expansion("LICENSE", "LICENSE.txt"));
        assert!(!matches_extension_change("LICENSE", "LICENSE.txt"));
    }

    #[test]
    fn test_ambiguous_extension_to_no_extension() {
        // Removing extension: forward direction doesn't match
        assert!(!matches_expansion("LICENSE.txt", "LICENSE"));
        assert!(!matches_extension_change("LICENSE.txt", "LICENSE"));

        assert!(!matches_expansion("README.md", "README"));
        assert!(!matches_extension_change("README.md", "README"));

        // But the REVERSE direction works (README -> README.md is expansion)
        // So `rn README` when README.md exists will work (bidirectional matching in rn.rs)
        assert!(matches_expansion("README", "README.md"));
        assert!(matches_expansion("LICENSE", "LICENSE.txt"));
    }
}
