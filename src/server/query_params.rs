use serde::Deserialize;
use warp::filters::query;

use std::collections::HashMap;


#[derive(Deserialize)]
pub struct RenderQuery {
    pub query: Option<String>,
}

impl RenderQuery {
    pub fn get_color_subs(self) -> HashMap<String,String> {
        let mut ret = HashMap::<String,String>::new();
        match self.query {
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

pub struct GifRenderQuery {
    pub query: Option<String>,
    render_type: GifRenderType
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GifRenderType {
    Forward,
    Backward,
    Both,
    None
}

impl GifRenderQuery {
    pub fn new(query_string: String) -> Self {
        let mut render_type = GifRenderType::Forward;
        let pairs = query_string.split("&");
        for pair in pairs {
            let p_split: Vec::<&str> = pair.split("=").collect();
            if p_split[0] == "render_type" {
                match p_split[1] {
                    "forward" => render_type = GifRenderType::Forward,
                    "backward" => render_type = GifRenderType::Backward,
                    "both" => render_type = GifRenderType::Both,
                    _ => render_type = GifRenderType::Forward
                }
            }
        }
        GifRenderQuery{
            query: Some(query_string),
            render_type: render_type
        }
    }

    // fn parse(mut self) {
    //     match self.query {
    //         Some(search) => {
    //             // Get every pair of subs and put in hashmap
    //             let pairs = search.split("&");
    //             for pair in pairs {
    //                 let p_split: Vec::<&str> = pair.split("=").collect();
    //                 if p_split[0] == "render_type" {
    //                     match p_split[1] {
    //                         "forward" => self.render_type = GifRenderType::Forward,
    //                         "backward" => self.render_type = GifRenderType::Backward,
    //                         "both" => self.render_type = GifRenderType::Both,
    //                         _ => self.render_type = GifRenderType::Forward
    //                     }
    //                 }
    //             }
    //         },
    //         None => {}
    //     }
    //     self.parsed = true;
    // }

    pub fn get_render_type(&self) -> GifRenderType {
        self.render_type
    }
}

#[cfg(test)]
mod search_query_tests {
    use super::RenderQuery;

    #[test]
    fn test_get_hashmap_color_subs() {
        let sq = RenderQuery{
            query: None
        };
        let hm = sq.get_color_subs();
        assert_eq!(hm.len(), 0);

        let sq = RenderQuery {
            query: Some("123456=fffefd".to_string())
        };
        let hm = sq.get_color_subs();
        assert_eq!(hm.len(), 1);
        assert_eq!(hm.contains_key("123456"), true);
        assert_eq!(hm.get("123456"), Some(&"fffefd".to_string()));

        let sq = RenderQuery {
            query: Some("123456=fffefd&123456=ffffff&654321=aaaaaa".to_string())
        };
        let hm = sq.get_color_subs();
        assert_eq!(hm.len(), 2);
        assert_eq!(hm.contains_key("123456"), true);
        assert_eq!(hm.get("123456"), Some(&"fffefd".to_string()));
        assert_eq!(hm.contains_key("654321"), true);
        assert_eq!(hm.get("654321"), Some(&"aaaaaa".to_string()));
    }

    // TODO: Add in GifRenderQuery tests
}