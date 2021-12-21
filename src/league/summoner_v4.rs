use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
    StatusCode,
};
use serde::Deserialize;

pub use crate::{error::Error, Platform};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SummonerResponse {
    pub id: String,
    pub account_id: String,
    pub puuid: String,
    pub name: String,
    pub profile_icon_id: u16,
    pub revision_date: u64,
    pub summoner_level: u16,
}

#[derive(Debug)]
pub struct SummonerV4 {
    client: Client,
    endpoint: String,
}

impl SummonerV4 {
    pub fn new(key: &str, platform: Platform) -> Self {
        let endpoint = format!(
            "https://{platform}.api.riotgames.com/lol/summoner/v4/summoners",
            platform = platform.to_string().to_ascii_lowercase()
        );

        let mut headers = HeaderMap::new();
        headers.insert("X-Riot-Token", HeaderValue::from_str(key).unwrap());

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self { client, endpoint }
    }

    fn invoke(&self, url: &str) -> Result<SummonerResponse, Error> {
        let request = self.client.get(url);
        let response = request.send();
        if let Err(error) = response {
            return Err(Error::new_message(&format!("Failed:\n{}", error)));
        }

        let response = response.unwrap();

        if response.status() != StatusCode::OK {
            return Err(Error::StatusCode(response.status()));
        }

        match response.json::<SummonerResponse>() {
            Ok(val) => Ok(val),
            Err(err) => Err(Error::new_message(&format!(
                "Couldn't Deserialize:\n{}",
                err
            ))),
        }
    }

    pub fn by_account(&self, encrypted_account_id: &str) -> Result<SummonerResponse, Error> {
        let url = format!("{}/by-account/{}", self.endpoint, encrypted_account_id);
        self.invoke(&url)
    }

    pub fn by_name(&self, summoner_name: &str) -> Result<SummonerResponse, Error> {
        let url = format!("{}/by-name/{}", self.endpoint, summoner_name);
        self.invoke(&url)
    }

    pub fn by_puuid(&self, encrypted_puuid: &str) -> Result<SummonerResponse, Error> {
        let url = format!("{}/by-puuid/{}", self.endpoint, encrypted_puuid);
        self.invoke(&url)
    }

    pub fn by_summoner_id(&self, summoner_name: &str) -> Result<SummonerResponse, Error> {
        let url = format!("{}/{}", self.endpoint, summoner_name);
        self.invoke(&url)
    }
}

#[cfg(test)]
mod tests {
    use std::env::var;

    use dotenv::dotenv;

    use super::*;

    #[test]
    fn test_by_account() {
        dotenv().ok();
        let key = var("API_KEY").unwrap();
        let id = var("ACCOUNT_ID").unwrap();
        let platform = Platform::NA1;
        let api = SummonerV4::new(&key, platform);
        let res = api.by_account(&id).unwrap();
        assert_eq!(res.account_id, id);
    }

    #[test]
    fn test_by_name() {
        dotenv().ok();
        let key = var("API_KEY").unwrap();
        let name = var("NAME").unwrap();
        let platform = Platform::NA1;
        let api = SummonerV4::new(&key, platform);
        let res = api.by_name(&name).unwrap();
        assert_eq!(res.name, name);
    }

    #[test]
    fn test_by_puuid() {
        dotenv().ok();
        let key = var("API_KEY").unwrap();
        let puuid = var("PUUID").unwrap();
        let platform = Platform::NA1;
        let api = SummonerV4::new(&key, platform);
        let res = api.by_puuid(&puuid).unwrap();
        assert_eq!(res.puuid, puuid);
    }
    #[test]
    fn test_by_summoner_id() {
        dotenv().ok();
        let key = var("API_KEY").unwrap();
        let id = var("SUMMONER_ID").unwrap();
        let platform = Platform::NA1;
        let api = SummonerV4::new(&key, platform);
        let res = api.by_summoner_id(&id).unwrap();
        assert_eq!(res.id, id);
    }
}
