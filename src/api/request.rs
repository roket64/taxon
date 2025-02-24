use log::debug;
use reqwest::{Client, Request, Url};

use super::schema::ProfileKind;
use super::schema::WorldStateKind;

pub struct UrlBuilder {}

impl UrlBuilder {
    pub fn get_profile_req_path(kind: &ProfileKind, username: &str) -> Vec<String> {
        match kind {
            // ../profile/{username}
            ProfileKind::Profile => {
                vec!["profile".to_string(), format!("{}", username.to_string())]
            }
            // ../profile/{username}/stats
            ProfileKind::Stats => {
                vec![
                    "profile".to_string(),
                    format!("{}", username.to_string()),
                    "stats".to_string(),
                ]
            }
        }
    }

    pub fn get_worldstate_req_path(kind: &WorldStateKind) -> Vec<String> {
        match kind {
            WorldStateKind::WorldState => decl_req_path!(),
            WorldStateKind::Alerts => decl_req_path!("alerts"),
            WorldStateKind::Arbitration => decl_req_path!("arbitration"),
            WorldStateKind::ArchonHunt => decl_req_path!("archonHunt"),
            WorldStateKind::CambionDrift => decl_req_path!("cambionCycle"),
            WorldStateKind::CetusState => decl_req_path!("cetusCycle"),
            WorldStateKind::ConclaveChallenge => decl_req_path!("conclaveChallenge"),
            WorldStateKind::ConstructionProgress => decl_req_path!("constructionProgress"),
            WorldStateKind::DailyDeal => decl_req_path!("dailyDeals"),
            WorldStateKind::DeepArchimedea => decl_req_path!("deepArchimedea"),
            WorldStateKind::EarthRotation => decl_req_path!("earthCycle"),
            WorldStateKind::Events => decl_req_path!("events"),
            WorldStateKind::Fissures => decl_req_path!("fissures"),
            WorldStateKind::FlashSales => decl_req_path!("flashSales"),
            WorldStateKind::GlobalUpgrades => decl_req_path!("globalUpgrades"),
            WorldStateKind::Invasion => decl_req_path!("invasions"),
            WorldStateKind::Kuva => decl_req_path!("kuva"),
            WorldStateKind::NewsItems => decl_req_path!("news"),
            WorldStateKind::Nightwave => decl_req_path!("nightwave"),
            WorldStateKind::PersistentEnemy => decl_req_path!("persistentEnemies"),
            WorldStateKind::Riven => decl_req_path!("rivens"),
            WorldStateKind::SentientOutpost => decl_req_path!("sentientOutposts"),
            WorldStateKind::SanctuaryStatus => decl_req_path!("simaris"),
            WorldStateKind::Sortie => decl_req_path!("sortie"),
            WorldStateKind::SteelPath => decl_req_path!("steelPath"),
            WorldStateKind::SyndicateMissionNodes => decl_req_path!("syndicateMissions"),
            WorldStateKind::Timestamp => decl_req_path!("timestamp"),
            WorldStateKind::OrbVallis => decl_req_path!("vallisCycle"),
            WorldStateKind::Varzia => decl_req_path!("vaultTrader"),
            WorldStateKind::VoidTrader => decl_req_path!("voidTrader"),
            WorldStateKind::VoidTraders => decl_req_path!("voidTraders"),
        }
    }

    pub fn build_request_url_test(
        base: &str,
        // paths: &[&str],
        paths: Vec<String>,
        query: Option<&str>,
    ) -> Result<Url, Box<dyn std::error::Error>> {
        let mut url = Url::parse(base)?;
        for path in paths {
            url.path_segments_mut().unwrap().extend([path]);
        }
        url.set_query(query);
        debug!("Url built: {:#?}", &url);
        Ok(url)
    }

    //TODO: the type of arguments should be generalized
    pub fn build_request_url(
        base: &str,
        paths: &[&str],
        query: Option<&str>,
    ) -> Result<Url, Box<dyn std::error::Error>> {
        let mut url = Url::parse(base)?;
        for path in paths {
            url.path_segments_mut().unwrap().extend([path]);
        }
        url.set_query(query);
        debug!("Url built: {:#?}", &url);
        Ok(url)
    }
}

#[derive(Debug)]
pub struct RequestBuilder {}

impl RequestBuilder {
    // builds request using GET method
    pub fn build_request(client: &Client, url: Url) -> Request {
        let request = client.get(url).build().unwrap();
        debug!("Request built: {:#?}", &request);
        request
    }
}
