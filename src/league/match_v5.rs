use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
    StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub use crate::{error::Error, Region};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
    pub metadata: Metadata,
    pub info: Info,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub data_version: String,
    pub match_id: String,
    pub participants: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub game_creation: i64,
    pub game_duration: i64,
    pub game_end_timestamp: i64,
    pub game_id: i64,
    pub game_mode: String,
    pub game_name: String,
    pub game_start_timestamp: i64,
    pub game_type: String,
    pub game_version: String,
    pub map_id: i64,
    pub participants: Vec<Participant>,
    pub platform_id: String,
    pub queue_id: i64,
    pub teams: Vec<Team>,
    pub tournament_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub assists: i64,
    pub baron_kills: i64,
    pub bounty_level: i64,
    pub champ_experience: i64,
    pub champ_level: i64,
    pub champion_id: i64,
    pub champion_name: String,
    pub champion_transform: i64,
    pub consumables_purchased: i64,
    pub damage_dealt_to_buildings: i64,
    pub damage_dealt_to_objectives: i64,
    pub damage_dealt_to_turrets: i64,
    pub damage_self_mitigated: i64,
    pub deaths: i64,
    pub detector_wards_placed: i64,
    pub double_kills: i64,
    pub dragon_kills: i64,
    pub first_blood_assist: bool,
    pub first_blood_kill: bool,
    pub first_tower_assist: bool,
    pub first_tower_kill: bool,
    pub game_ended_in_early_surrender: bool,
    pub game_ended_in_surrender: bool,
    pub gold_earned: i64,
    pub gold_spent: i64,
    pub individual_position: String,
    pub inhibitor_kills: i64,
    pub inhibitor_takedowns: i64,
    pub inhibitors_lost: i64,
    pub item0: i64,
    pub item1: i64,
    pub item2: i64,
    pub item3: i64,
    pub item4: i64,
    pub item5: i64,
    pub item6: i64,
    pub items_purchased: i64,
    pub killing_sprees: i64,
    pub kills: i64,
    pub lane: String,
    pub largest_critical_strike: i64,
    pub largest_killing_spree: i64,
    pub largest_multi_kill: i64,
    pub longest_time_spent_living: i64,
    pub magic_damage_dealt: i64,
    pub magic_damage_dealt_to_champions: i64,
    pub magic_damage_taken: i64,
    pub neutral_minions_killed: i64,
    pub nexus_kills: i64,
    pub nexus_lost: i64,
    pub nexus_takedowns: i64,
    pub objectives_stolen: i64,
    pub objectives_stolen_assists: i64,
    pub participant_id: i64,
    pub penta_kills: i64,
    pub perks: Perks,
    pub physical_damage_dealt: i64,
    pub physical_damage_dealt_to_champions: i64,
    pub physical_damage_taken: i64,
    pub profile_icon: i64,
    pub puuid: String,
    pub quadra_kills: i64,
    pub riot_id_name: String,
    pub riot_id_tagline: String,
    pub role: String,
    pub sight_wards_bought_in_game: i64,
    #[serde(rename = "spell1Casts")]
    pub spell1casts: i64,
    #[serde(rename = "spell2Casts")]
    pub spell2casts: i64,
    #[serde(rename = "spell3Casts")]
    pub spell3casts: i64,
    #[serde(rename = "spell4Casts")]
    pub spell4casts: i64,
    #[serde(rename = "summoner1Casts")]
    pub summoner1casts: i64,
    #[serde(rename = "summoner1Id")]
    pub summoner1id: i64,
    #[serde(rename = "summoner2Casts")]
    pub summoner2casts: i64,
    #[serde(rename = "summoner2Id")]
    pub summoner2id: i64,
    pub summoner_id: String,
    pub summoner_level: i64,
    pub summoner_name: String,
    pub team_early_surrendered: bool,
    pub team_id: i64,
    pub team_position: String,
    #[serde(rename = "timeCCingOthers")]
    pub time_ccing_others: i64,
    pub time_played: i64,
    pub total_damage_dealt: i64,
    pub total_damage_dealt_to_champions: i64,
    pub total_damage_shielded_on_teammates: i64,
    pub total_damage_taken: i64,
    pub total_heal: i64,
    pub total_heals_on_teammates: i64,
    pub total_minions_killed: i64,
    #[serde(rename = "totalTimeCCDealt")]
    pub total_time_ccdealt: i64,
    pub total_time_spent_dead: i64,
    pub total_units_healed: i64,
    pub triple_kills: i64,
    pub true_damage_dealt: i64,
    pub true_damage_dealt_to_champions: i64,
    pub true_damage_taken: i64,
    pub turret_kills: i64,
    pub turret_takedowns: i64,
    pub turrets_lost: i64,
    pub unreal_kills: i64,
    pub vision_score: i64,
    pub vision_wards_bought_in_game: i64,
    pub wards_killed: i64,
    pub wards_placed: i64,
    pub win: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Perks {
    pub stat_perks: StatPerks,
    pub styles: Vec<Style>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatPerks {
    pub defense: i64,
    pub flex: i64,
    pub offense: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    pub description: String,
    pub selections: Vec<Selection>,
    pub style: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Selection {
    pub perk: i64,
    pub var1: i64,
    pub var2: i64,
    pub var3: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub bans: Vec<Value>,
    pub objectives: Objectives,
    pub team_id: i64,
    pub win: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Objectives {
    pub baron: Baron,
    pub champion: Champion,
    pub dragon: Dragon,
    pub inhibitor: Inhibitor,
    pub rift_herald: RiftHerald,
    pub tower: Tower,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Baron {
    pub first: bool,
    pub kills: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Champion {
    pub first: bool,
    pub kills: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dragon {
    pub first: bool,
    pub kills: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inhibitor {
    pub first: bool,
    pub kills: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiftHerald {
    pub first: bool,
    pub kills: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tower {
    pub first: bool,
    pub kills: i64,
}

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

    pub fn by_puuid(&self, puuid: &str, options: ByPUUIDOptions) -> Result<Vec<String>, Error> {
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

    pub fn match_info(&self, match_id: &str) -> Result<MatchInfo, Error> {
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

        match response.json::<MatchInfo>() {
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
