use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
    StatusCode,
};
// use serde::Deserialize;
use serde_json::Map;

pub use crate::{error::Error, Region};

pub type Data = Map<String, serde_json::Value>;

#[derive(Debug)]
pub struct ByPUUIDOptions {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub queue: Option<u32>,
    pub match_type: Option<String>,
    pub start: Option<u32>,
    pub count: Option<u8>,
}

#[derive(Debug)]
pub struct MatchV5 {
    client: Client,
    endpoint: String,
}

impl MatchV5 {
    pub fn new(key: &str, region: Region) -> Self {
        let endpoint = format!(
            "https://{region}.api.riotgames.com/lol/match/v5/matches",
            region = region.to_string().to_ascii_lowercase()
        );

        let mut headers = HeaderMap::new();
        headers.insert("X-Riot-Token", HeaderValue::from_str(key).unwrap());

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self { client, endpoint }
    }

    fn stringify_options(options: ByPUUIDOptions) -> String {
        let mut s = String::new();
        s.push('?');
        if let Some(val) = options.start {
            let opt = format!("start={}", val);
            s.push_str(&opt);
        }
        if let Some(val) = options.count {
            if !s.ends_with('&') && !s.ends_with('?') {
                s.push('&');
            }
            let opt = format!("count={}", val);
            s.push_str(&opt);
        }
        if let Some(val) = options.queue {
            if !s.ends_with('&') && !s.ends_with('?') {
                s.push('&');
            }
            let opt = format!("queue={}", val);
            s.push_str(&opt);
        }
        if let Some(val) = options.match_type {
            if !s.ends_with('&') && !s.ends_with('?') {
                s.push('&');
            }
            let opt = format!("type={}", val);
            s.push_str(&opt);
        }
        if let Some(val) = options.start_time {
            if !s.ends_with('&') && !s.ends_with('?') {
                s.push('&');
            }
            let opt = format!("startTime={}", val);
            s.push_str(&opt);
        }
        if let Some(val) = options.end_time {
            if !s.ends_with('&') && !s.ends_with('?') {
                s.push('&');
            }
            let opt = format!("endTime={}", val);
            s.push_str(&opt);
        }
        s
    }

    pub fn by_puuid(self, puuid: &str, options: ByPUUIDOptions) -> Result<Vec<String>, Error> {
        let url = format!(
            "{}/by-puuid/{}/ids{}",
            self.endpoint,
            puuid,
            Self::stringify_options(options)
        );
        let request = self.client.get(&url);
        let response = request.send();
        if let Err(error) = response {
            return Err(Error::new_message(&format!("Failed:\n{}", error)));
        }

        let response = response.unwrap();

        if response.status() != StatusCode::OK {
            return Err(Error::StatusCode(response.status()));
        }

        match response.json::<Vec<String>>() {
            Ok(val) => Ok(val),
            Err(err) => Err(Error::Message(err.to_string())),
        }
    }

    pub fn match_info(&self, match_id: &str) -> Result<Data, Error> {
        let url = format!("{}/{}", self.endpoint, match_id);
        let request = self.client.get(&url);
        let response = request.send();
        if let Err(error) = response {
            return Err(Error::new_message(&format!("Failed:\n{}", error)));
        }

        let response = response.unwrap();

        if response.status() != StatusCode::OK {
            return Err(Error::StatusCode(response.status()));
        }

        match response.json::<Data>() {
            Ok(val) => Ok(val),
            Err(err) => Err(Error::Message(err.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::var;

    use dotenv::dotenv;

    use super::*;

    #[test]
    fn test_by_puuid() {
        dotenv().ok();
        let key = var("API_KEY").unwrap();
        let puuid = var("PUUID").unwrap();
        let region = Region::AMERICAS;
        let api = MatchV5::new(&key, region);
        let options = ByPUUIDOptions {
            start_time: None,
            end_time: None,
            queue: None,
            match_type: None,
            start: None,
            count: None,
        };
        let res = api.by_puuid(&puuid, options).unwrap();
        println!("{:?}", res);
    }

    #[test]
    fn test_get_match_info() {
        dotenv().ok();
        let key = var("API_KEY").unwrap();
        let match_id = var("MATCH_ID").unwrap();
        let region = Region::AMERICAS;
        let api = MatchV5::new(&key, region);
        let res = api.match_info(&match_id).unwrap();
        println!("{:#?}", res);
    }
}
