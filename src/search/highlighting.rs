use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub enum Fragment {
    Highlighted(char),
    Normal(char),
}

// I assumed the mpn does not allways start with search query which complicates the code
// I assumed there could be multiple search results in one mpn
// I assumed the in case of multiple ways to highlight we should get the first one (search:aaa mpn:aaaaa should highlight first aaa)

pub fn highlight_search_query_in_mpn(search_query: &str, mpn: &str) -> Vec<Fragment> {
    let mut string_fragments: Vec<Fragment> = Vec::new();
    let mut mpn_chars = mpn.chars().peekable();

    let starting_char = search_query.chars().next();
    let mut i = 0;
    while let Some(mpn_char) = mpn_chars.peek() {
        if mpn_char.eq_ignore_ascii_case(&starting_char.unwrap())
            && is_start_of_highlight(search_query, mpn, i)
        {
            for query_char in search_query.chars() {
                while let Some(mpn_char) = mpn_chars.next() {
                    i += 1;
                    match mpn_char {
                        mpn_char if mpn_char.eq_ignore_ascii_case(&query_char) => {
                            string_fragments.push(Fragment::Highlighted(mpn_char));
                            break;
                        }
                        '-' | '/' | '.' | ',' => {
                            string_fragments.push(Fragment::Normal(mpn_char));
                            continue;
                        }
                        _ => {
                            unreachable!("Wrong characters in verified search query");
                        }
                        
                    }
                }
            }
        } else {
            i += 1;
            string_fragments.push(Fragment::Normal(*mpn_char));
            mpn_chars.next();
        }
    }
    string_fragments
}
fn is_start_of_highlight(search_query: &str, mpn: &str, n: usize) -> bool {
    let mut mpn_chars = mpn.chars().skip(n);
    for query_char in search_query.chars() {
        while let Some(mpn_char) = mpn_chars.next() {
            match mpn_char {
                mpn_char if mpn_char.eq_ignore_ascii_case(&query_char) => {
                    break;
                }
                '-' | '/' | '.' | ',' => {
                    continue;
                }
                _ => {
                    return false;
                }
            }
        }
    }
    return true;
}

#[cfg(test)]
mod highlighting_tests {
    use super::*;

    #[test]
    fn test_highlight_simple() {
        let search_query = "abc";
        let mpn = "a-bcde";
        let expected_fragments = vec![
            Fragment::Highlighted('a'),
            Fragment::Normal('-'),
            Fragment::Highlighted('b'),
            Fragment::Highlighted('c'),
            Fragment::Normal('d'),
            Fragment::Normal('e'),
        ];
        assert_eq!(
            highlight_search_query_in_mpn(search_query, mpn),
            expected_fragments
        );
        return;
    }
    #[test]
    fn test_highlight_partial_repetition() {
        let search_query = "abc";
        let mpn = "abababcde";
        let expected_fragments = vec![
            Fragment::Normal('a'),
            Fragment::Normal('b'),
            Fragment::Normal('a'),
            Fragment::Normal('b'),
            Fragment::Highlighted('a'),
            Fragment::Highlighted('b'),
            Fragment::Highlighted('c'),
            Fragment::Normal('d'),
            Fragment::Normal('e'),
        ];
        assert_eq!(
            highlight_search_query_in_mpn(search_query, mpn),
            expected_fragments
        );
        return;
    }
    #[test]
    fn test_highlight_same_letter_repetition() {
        let search_query = "aabc";
        let mpn = "aaab-cd";
        let expected_fragments = vec![
            Fragment::Normal('a'),
            Fragment::Highlighted('a'),
            Fragment::Highlighted('a'),
            Fragment::Highlighted('b'),
            Fragment::Normal('-'),
            Fragment::Highlighted('c'),
            Fragment::Normal('d'),
        ];
        assert_eq!(
            highlight_search_query_in_mpn(search_query, mpn),
            expected_fragments
        );
        return;
    }
    #[test]
    fn multiple_highlights() {
        let search_query = "ab";
        let mpn = "abcab";
        let expected_fragments = vec![
            Fragment::Highlighted('a'),
            Fragment::Highlighted('b'),
            Fragment::Normal('c'),
            Fragment::Highlighted('a'),
            Fragment::Highlighted('b'),
        ];
        assert_eq!(
            highlight_search_query_in_mpn(search_query, mpn),
            expected_fragments
        );
        return;
    }
    #[test]
    fn single_character() {
        let search_query = "-";
        let mpn = "-";
        let expected_fragments = vec![Fragment::Highlighted('-')];
        assert_eq!(
            highlight_search_query_in_mpn(search_query, mpn),
            expected_fragments
        );
        return;
    }
}
