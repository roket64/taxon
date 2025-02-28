use bytes::Bytes;
use log::debug;
use request::UrlBuilder;
use reqwest::Response;
use response::ResponseHandler;
use schema::*;

macro_rules! decl_req_path {
    () => {
        vec!["pc".to_string()]
    };

    ($($path:literal),+) => {
        vec!["pc".to_string(), $($path.to_string()),+]
    };
}

macro_rules! decl_fetch_fn {
    ($(
        $(#[deprecated = $deprecated: literal])?
        {$wrapper: path, $parse: ty} => fn $fn_name: ident(chunk: &Bytes) -> $ret: ty;
    )*) => {
        $(
            $(#[deprecated = $deprecated])?
            pub fn $fn_name(chunk: &Bytes) -> $ret {
            let res: $parse = serde_json::from_slice(&chunk).unwrap();
            $wrapper(res)
        })*
    };
}

#[derive(Debug)]
pub struct ApiHandler;

impl ApiHandler {
    pub async fn fetch_contents(paths: Vec<String>, query: Option<&str>) -> Response {
        let url = UrlBuilder::build_request_url(paths, query).unwrap();
        reqwest::get(url).await.unwrap()
    }

    pub async fn fetch_profile_contents(
        kind: &ProfileKind,
        query: Option<&str>,
        username: &str,
    ) -> ProfileSchema {
        let paths = UrlBuilder::get_profile_req_path(kind, username);
        let mut response = ApiHandler::fetch_contents(paths, query).await;
        debug!("`Response` received: {:#?}", response);

        // you may use just `response.bytes()`
        let chunk = ResponseHandler::get_response_chunk(&mut response)
            .await
            .unwrap();
        debug!("`Bytes` parsed: {:#?}", &chunk);

        let schema = match kind {
            ProfileKind::Profile => ApiHandler::fetch_profile(&chunk),
            ProfileKind::Stats => ApiHandler::fetch_stats(&chunk),
        };
        schema
    }

    pub async fn fetch_worldstate_contents(
        kind: &WorldStateKind,
        query: Option<&str>,
    ) -> WorldStateSchema {
        let paths = UrlBuilder::get_worldstate_req_path(kind);
        let mut response = ApiHandler::fetch_contents(paths, query).await;
        debug!("response received: {:#?}", &response);

        // you may use just `response.bytes()`
        let chunk = ResponseHandler::get_response_chunk(&mut response)
            .await
            .unwrap();
        debug!("bytes parsed: {:#?}", &chunk);

        let schema = match kind {
            // WorldStateKind::WorldState => todo!(),
            WorldStateKind::Alerts => ApiHandler::fetch_alerts(&chunk),
            WorldStateKind::Arbitration => ApiHandler::fetch_arbitration(&chunk),
            WorldStateKind::ArchonHunt => ApiHandler::fetch_archon_hunt(&chunk),
            WorldStateKind::CambionDrift => ApiHandler::fetch_cambion_drift(&chunk),
            WorldStateKind::CetusStatus => ApiHandler::fetch_cetus_status(&chunk),
            WorldStateKind::ConclaveChallenge => ApiHandler::fetch_conclave_challenges(&chunk),
            WorldStateKind::ConstructionProgress => ApiHandler::fetch_construction_progress(&chunk),
            WorldStateKind::DailyDeal => ApiHandler::fetch_daily_deals(&chunk),
            WorldStateKind::DeepArchimedea => ApiHandler::fetch_deep_archimedea(&chunk),
            WorldStateKind::EarthRotation => ApiHandler::fetch_earth_rotation(&chunk),
            WorldStateKind::Events => ApiHandler::fetch_events(&chunk),
            WorldStateKind::Fissures => ApiHandler::fetch_fissures(&chunk),
            WorldStateKind::FlashSales => ApiHandler::fetch_flash_sales(&chunk),
            WorldStateKind::GlobalUpgrades => ApiHandler::fetch_global_upgrades(&chunk),
            WorldStateKind::Invasion => ApiHandler::fetch_invasions(&chunk),
            WorldStateKind::Kuva => ApiHandler::fetch_kuva(&chunk),
            WorldStateKind::NewsItems => ApiHandler::fetch_news_items(&chunk),
            WorldStateKind::Nightwave => ApiHandler::fetch_nightwave(&chunk),
            WorldStateKind::PersistentEnemy => ApiHandler::fetch_persistent_enemy(&chunk),
            WorldStateKind::Riven => ApiHandler::fetch_riven(&chunk),
            WorldStateKind::SentientOutpost => ApiHandler::fetch_sentient_outpost(&chunk),
            WorldStateKind::SanctuaryStatus => ApiHandler::fetch_sanctuary_status(&chunk),
            WorldStateKind::Sortie => ApiHandler::fetch_sortie(&chunk),
            WorldStateKind::SteelPath => ApiHandler::fetch_steel_path(&chunk),
            WorldStateKind::SyndicateMissionNodes => {
                ApiHandler::fetch_syndicate_mission_nodes(&chunk)
            }
            WorldStateKind::Timestamp => ApiHandler::fetch_timestamp(&chunk),
            WorldStateKind::OrbVallis => ApiHandler::fetch_orb_vallis(&chunk),
            WorldStateKind::Varzia => ApiHandler::fetch_varzia(&chunk),
            WorldStateKind::VoidTrader => ApiHandler::fetch_void_trader(&chunk),
            WorldStateKind::VoidTraders => ApiHandler::fetch_void_traders(&chunk),
        };
        schema
    }

    decl_fetch_fn!(
        { WorldStateSchema::Alerts, Vec<Alerts> } => fn fetch_alerts(chunk: &Bytes) -> WorldStateSchema;
        { WorldStateSchema::Arbitration, Arbitration } => fn fetch_arbitration(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::ArchonHunt, ArchonHunt} => fn fetch_archon_hunt(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::CambionDrift, CambionDrift } => fn fetch_cambion_drift(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::CetusStatus, CetusStatus } => fn fetch_cetus_status(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::ConclaveChallenge, Vec<ConclaveChallenge> } => fn fetch_conclave_challenges(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::ConstructionProgress, ConstructionProgress } => fn fetch_construction_progress(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::DailyDeal, Vec<DailyDeal> } => fn fetch_daily_deals(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::DeepArchimedea, DeepArchimedea } => fn fetch_deep_archimedea(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::EarthRotation, EarthRotation } => fn fetch_earth_rotation(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::Events, Events } => fn fetch_events(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::Fissures, Vec<Fissures> } => fn fetch_fissures(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::FlashSales, Vec<FlashSales> } => fn fetch_flash_sales(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::GlobalUpgrades, Vec<GlobalUpgrades> } => fn fetch_global_upgrades(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::Invasion, Vec<Invasion> } => fn fetch_invasions(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::Kuva, Vec<Kuva> } => fn fetch_kuva(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::NewsItems, Vec<NewsItems> } => fn fetch_news_items(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::Nightwave, Nightwave } => fn fetch_nightwave(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::PersistentEnemy, Vec<PersistentEnemy> } => fn fetch_persistent_enemy(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::Riven, Riven } => fn fetch_riven(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::SentientOutpost, SentientOutpost } => fn fetch_sentient_outpost(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::SanctuaryStatus, SanctuaryStatus } => fn fetch_sanctuary_status(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::Sortie, Sortie } => fn fetch_sortie(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::SteelPath, SteelPath } => fn fetch_steel_path(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::SyndicateMissionNodes, SyndicateMissionNodes } => fn fetch_syndicate_mission_nodes(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::Timestamp, Timestamp } => fn fetch_timestamp(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::OrbVallis, OrbVallis } => fn fetch_orb_vallis(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::Varzia, Varzia } => fn fetch_varzia(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::VoidTrader, VoidTrader } => fn fetch_void_trader(chunk: &Bytes)-> WorldStateSchema;
        { WorldStateSchema::VoidTraders, Vec<VoidTraders> } => fn fetch_void_traders(chunk: &Bytes)-> WorldStateSchema;
        { ProfileSchema::Profile,  Profile} => fn fetch_profile(chunk: &Bytes)-> ProfileSchema;
        { ProfileSchema::Stats, Stats} => fn fetch_stats(chunk: &Bytes)-> ProfileSchema;
    );
}

pub mod request;
pub mod response;
pub mod schema;
