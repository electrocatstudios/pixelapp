use serde::Deserialize;

use std::collections::HashMap;


#[derive(Deserialize)]
pub struct SearchQuery {
    pub search: Option<String>,
}

impl SearchQuery {
    pub fn get_color_subs(self) -> HashMap<String,String> {
        let mut ret = HashMap::<String,String>::new();
        match self.search {
            Some(search) => {
                // Get every pair of subs and put in hashmap
                let pairs = search.split("&");
                for pair in pairs {
                    let p_split: Vec::<&str> = pair.split("=").collect();
                    if p_split.len() != 2 {
                        continue;
                    }
                    if p_split[0].len() != 6 || p_split[1].len() != 6 {
                        continue;
                    }
                    if ret.contains_key(p_split[0]) {
                        continue;
                    }
                    ret.insert(p_split[0].to_string(), p_split[1].to_string());
                }
                ret
            },
            None => ret
        }
    }
}

#[cfg(test)]
mod search_query_tests {
    use super::SearchQuery;

    #[test]
    fn test_get_hashmap_color_subs() {
        let sq = SearchQuery{
            search: None
        };
        let hm = sq.get_color_subs();
        assert_eq!(hm.len(), 0);

        let sq = SearchQuery {
            search: Some("123456=fffefd".to_string())
        };
        let hm = sq.get_color_subs();
        assert_eq!(hm.len(), 1);
        assert_eq!(hm.contains_key("123456"), true);
        assert_eq!(hm.get("123456"), Some(&"fffefd".to_string()));

        let sq = SearchQuery {
            search: Some("123456=fffefd&123456=ffffff&654321=aaaaaa".to_string())
        };
        let hm = sq.get_color_subs();
        assert_eq!(hm.len(), 2);
        assert_eq!(hm.contains_key("123456"), true);
        assert_eq!(hm.get("123456"), Some(&"fffefd".to_string()));
        assert_eq!(hm.contains_key("654321"), true);
        assert_eq!(hm.get("654321"), Some(&"aaaaaa".to_string()));
    }
}