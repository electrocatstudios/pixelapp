use serde::Deserialize;

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

impl GifRenderQuery {
    pub fn _default() -> Self {
        GifRenderQuery {
            query: None,
            render_type: GifRenderType::Forward
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GifRenderType {
    Forward,
    Backward,
    Both,
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
                    "forwards" => render_type = GifRenderType::Forward,
                    "fwd" => render_type = GifRenderType::Forward,
                    "f" => render_type = GifRenderType::Forward,
                    "backward" => render_type = GifRenderType::Backward,
                    "backwards" => render_type = GifRenderType::Backward,
                    "bwd" => render_type = GifRenderType::Backward,
                    "b" => render_type = GifRenderType::Backward,
                    "both" => render_type = GifRenderType::Both,
                    "bth" => render_type = GifRenderType::Both,
                    _ => render_type = GifRenderType::Forward
                }
            }
        }
        GifRenderQuery{
            query: Some(query_string),
            render_type: render_type
        }
    }

    pub fn get_render_type(&self) -> GifRenderType {
        self.render_type
    }

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


pub struct VideoFrameQuery {
    pub start: usize,
    pub end: usize,
    pub diff: usize
}
impl VideoFrameQuery {
    pub fn from_query(max_frame: usize, query: String) -> Self {
        let pairs = query.split("&");
        
        let mut start = 1;
        let mut end = max_frame;
        for pair in pairs {
            let p_split: Vec::<&str> = pair.split("=").collect();
            if p_split[0].to_lowercase() == "start" {
                start = p_split[1].parse::<usize>().unwrap();
            }
            if p_split[0].to_lowercase() == "end" {
                end = p_split[1].parse::<usize>().unwrap();
            }
        }
        if start > end {
            end = start;
        }
        if start > max_frame {
            start = max_frame;
            end = max_frame;
        }
        VideoFrameQuery {
            start: start,
            end: end,
            diff: end-start
        }
    }

    pub fn default() -> Self {
        VideoFrameQuery {
            start: 0,
            end: 0,
            diff: 0
        }
    }
}
#[cfg(test)]
mod search_query_tests {
    use super::{RenderQuery, VideoFrameQuery};

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

    #[test]
    fn test_get_video_frame_query() {
        let vq = VideoFrameQuery::from_query(123, "".to_string());
        assert_eq!(vq.start, 1);
        assert_eq!(vq.end, 123);

        let vq = VideoFrameQuery::from_query(234, "start=4&end=56".to_string());
        assert_eq!(vq.start, 4);
        assert_eq!(vq.end, 56);

        let vq = VideoFrameQuery::from_query(234, "start=400&end=232".to_string());
        assert_eq!(vq.start, 234); // If end is in the past it uses start - but then start is ahead of max
        assert_eq!(vq.end, 234);    // therefore both are just the max_frame

        let vq = VideoFrameQuery::from_query(496, "start=421".to_string());
        assert_eq!(vq.start, 421);
        assert_eq!(vq.end, 496);

        let vq = VideoFrameQuery::from_query(496, "end=421".to_string());
        assert_eq!(vq.start, 1);
        assert_eq!(vq.end, 421);
    }
}