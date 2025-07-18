#![allow(unused_imports)]
use code::set11::longest_common_subsequence::longest_common_subsequence_len;
use code::test_case;

#[test]
fn test_empty_strings() {
    test_case!(longest_common_subsequence_len,("", ""), 0);
    test_case!(longest_common_subsequence_len,("abc", ""), 0);
    test_case!(longest_common_subsequence_len,("", "abc"), 0);
}

#[test]
fn test_no_common_subsequence() {
    test_case!(longest_common_subsequence_len,("abc", "xyz"), 0);
}

#[test]
fn test_identical_strings() {
    test_case!(longest_common_subsequence_len,("abcde", "abcde"), 5);
}

#[test]
fn test_partial_match() {
    test_case!(longest_common_subsequence_len,("abcde", "ace"), 3);
    test_case!(longest_common_subsequence_len,("abc", "abc"), 3);
    test_case!(longest_common_subsequence_len,("abc", "ac"), 2);
    test_case!(longest_common_subsequence_len,("AGGTAB", "GXTXAYB"), 4); // GTAB
}

#[test]
fn test_repeated_characters() {
    test_case!(longest_common_subsequence_len,("aaaa", "aa"), 2);
    test_case!(longest_common_subsequence_len,("aabba", "ababa"), 4);
}

#[test]
fn test_longer_strings() {
    let s1 = "abcdefghij";
    let s2 = "cdgi";
    test_case!(longest_common_subsequence_len,(s1, s2), 4); // "cdgi"
}
