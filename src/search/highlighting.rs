// Type to mark characters as highlighted

#[derive(Debug)]
enum Fragment {
    Highlighted(char),
    Normal(char),
}

// This is the function doing the highlighting
// Takes search_query and mpn and returns vector of mpn characters with highlighting
fn highlight_search_query_in_mpn(search_query: &str, mpn: &str) -> Vec<Fragment> {
    let mut string_fragments: Vec<Fragment> = Vec::new();
    let mut mpn_chars = mpn.chars();

    for query_char in search_query.chars() {
        while let Some(mpn_char) = mpn_chars.next() {
            if mpn_char == query_char {
                string_fragments.push(Fragment::Highlighted(mpn_char));
                break;
            } else {
                match mpn_char {
                    '-' | '/' | '.' | ',' | ' ' => {
                        string_fragments.push(Fragment::Normal(mpn_char));
                        break;
                    }
                    _ => {
                        string_fragments = Vec::new();
                    }
                }
            }
        }
    }
    string_fragments
}

#[cfg(test)]
mod highlighting_tests {
    use super::*;

    #[test]
    fn test_highlight_search_query_in_mpn() {
        let search_query = "abc";
        let mpn = "a-bcde";
        let expected_fragments = vec![
            Fragment::Highlighted('a'),
            Fragment::Normal('-'),
            Fragment::Highlighted('b'),
            Fragment::Normal('c'),
            Fragment::Normal('d'),
            Fragment::Normal('e'),
        ];
        assert_eq!(
            highlight_search_query_in_mpn(search_query, mpn),
            expected_fragments
        );

        let search_query = "xyz";
        let mpn = "a-bcde";
        let expected_fragments = vec![];
        assert_eq!(
            highlight_search_query_in_mpn(search_query, mpn),
            expected_fragments
        );

        let search_query = "a";
        let mpn = "a";
        let expected_fragments = vec![Fragment::Highlighted('a')];
        assert_eq!(
            highlight_search_query_in_mpn(search_query, mpn),
            expected_fragments
        );

        let search_query = "a";
        let mpn = "b";
        let expected_fragments = vec![];
        assert_eq!(
            highlight_search_query_in_mpn(search_query, mpn),
            expected_fragments
        );
    }
}
