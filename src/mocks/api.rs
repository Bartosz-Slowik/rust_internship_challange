use random_number::rand::distributions::Alphanumeric;
use random_number::rand::{thread_rng, Rng};

// This mock API is provided for easier testing and context. There should be no need to
// modify anything here.

pub struct PartSearchAPI {}

#[derive(Debug)]
pub struct Part {
    mpn: String,
}

impl PartSearchAPI {
    pub fn search(&self, search_query: &str, _api_key: &str) -> Vec<Part> {
        let n: u8 = random!(..=10);
        let mut parts = Vec::new();
        for _ in 0..n {
            parts.push(generate_random_part_starting_with(&search_query))
        }

        info!(
            "List of mpns received from mock api: {:?}",
            parts.iter().map(|p| &p.mpn)
        );

        parts
    }
}

fn generate_random_part_starting_with(start_string: &str) -> Part {
    Part {
        mpn: format!(
            "{}{}",
            start_string,
            thread_rng()
                .sample_iter(Alphanumeric)
                .take(5)
                .map(char::from)
                .collect::<String>()
        ),
    }
}

#[cfg(test)]
mod tests {
    mod unit {
        use crate::mocks::api::PartSearchAPI;

        #[test]
        fn test_mpn_querying() {
            let mock_api = PartSearchAPI {};
            let search_query = "abc";

            let returned_parts = mock_api.search(search_query, "MOCKAPIKEY");

            assert!(returned_parts
                .iter()
                .all(|part| part.mpn.contains(search_query)))
        }
    }
}
