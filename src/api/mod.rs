use bytes::Bytes;
use request::{RequestBuilder, UrlBuilder};
use reqwest::{Client, Response};
use response::ResponseHandler;
use schema::*;

macro_rules! decl_req_path {
    () => {
        &["pc"]
    };

    ($($path:literal),+) => {
        &["pc", $($path),+]
    };
}

macro_rules! decl_req_query {
    ($($key:literal=$val:literal),*) => {
        &[$(($key, $val),)*]
    };
}

// idk this is the best way to implement such thing
macro_rules! decl_fetch_func {
    ($func_name: ident, $wrapper: path, $ret: ty) => {
        fn $func_name(&self, chunk: &Bytes) -> WorldStateSchema {
            let res: $ret = serde_json::from_slice(&chunk).unwrap();
            $wrapper(res)
        }
    };
}

#[derive(Debug)]
pub struct ApiHandler {}

impl ApiHandler {
    pub fn new() -> Self {
        ApiHandler {}
    }

    pub async fn fetch(&self, kind: &WorldStateKind) -> WorldStateSchema {
        let client = Client::new();
        let base = "https://api.warframestat.us";
        //TODO: maybe i should use a predefined HashMap<> or something
        let paths = UrlBuilder::get_req_path(&kind);
        let query = decl_req_query!("language" = "en");

        //TODO: this should try to connect to the server until the connection is established
        let mut response = self
            .connect_server(&client, base, paths, query)
            .await
            .unwrap();

        // you may use just `response.bytes()`
        let chunk = ResponseHandler::get_response_chunk(&mut response)
            .await
            .unwrap();

        let contents = match kind {
            WorldStateKind::WorldState => todo!(),
            WorldStateKind::Alerts => self.fetch_alerts(&chunk),
            WorldStateKind::Arbitration => self.fetch_arbitration(&chunk),
            WorldStateKind::ArchonHunt => self.fetch_archon_hunt(&chunk),
            WorldStateKind::CambionDrift => self.fetch_cambion_drift(&chunk),
            WorldStateKind::CetusState => self.fetch_cetus_state(&chunk),
            WorldStateKind::ConclaveChallenge => self.fetch_conclave_challenges(&chunk),
            WorldStateKind::ConstructionProgress => self.fetch_construction_porgress(&chunk),
            WorldStateKind::DailyDeal => self.fetch_daily_deals(&chunk),
            WorldStateKind::DeepArchimedea => self.fetch_deep_archimedea(&chunk),
            WorldStateKind::EarthRotation => self.fetch_earth_rotation(&chunk),
            WorldStateKind::Events => self.fetch_events(&chunk),
            WorldStateKind::Fissures => self.fetch_fissures(&chunk),
            WorldStateKind::FlashSales => self.fetch_flash_sales(&chunk),
            WorldStateKind::GlobalUpgrades => self.fetch_global_upgrades(&chunk),
            WorldStateKind::Invasion => self.fetch_invasions(&chunk),
            WorldStateKind::Kuva => self.fetch_kuva(&chunk),
            WorldStateKind::NewsItems => self.fetch_news_items(&chunk),
            WorldStateKind::Nightwave => self.fetch_nightwave(&chunk),
            WorldStateKind::PersistentEnemy => self.fetch_persistent_enemy(&chunk),
            WorldStateKind::Riven => self.fetch_riven(&chunk),
            WorldStateKind::SentientOutpost => self.fetch_sentient_outpost(&chunk),
            WorldStateKind::SanctuaryStatus => self.fetch_sanctuary_status(&chunk),
            WorldStateKind::Sortie => self.fetch_sortie(&chunk),
            WorldStateKind::SteelPath => self.fetch_steel_path(&chunk),
            WorldStateKind::SyndicateMissionNodes => self.fetch_syndicate_mission_nodes(&chunk),
            WorldStateKind::Timestamp => self.fetch_timestamp(&chunk),
            WorldStateKind::OrbVallis => self.fetch_orb_vallis(&chunk),
            WorldStateKind::Varzia => self.fetch_varzia(&chunk),
            WorldStateKind::VoidTrader => self.fetch_void_trader(&chunk),
            WorldStateKind::VoidTraders => self.fetch_void_traders(&chunk),
        };
        contents
    }

    async fn connect_server(
        &self,
        client: &Client,
        base: &str,
        paths: &[&str],
        query: &[(&str, &str)],
    ) -> Result<Response, reqwest::Error> {
        let url = RequestBuilder::build_request_url(base, paths).unwrap();
        let req = RequestBuilder::build_request(client, url, query);
        client.execute(req).await
    }

    /********************************** WorldStateKind **********************************/
    decl_fetch_func!(fetch_alerts, WorldStateSchema::Alerts, Vec<Alerts>);

    decl_fetch_func!(
        fetch_arbitration,
        WorldStateSchema::Arbitration,
        Arbitration
    );

    decl_fetch_func!(fetch_archon_hunt, WorldStateSchema::ArchonHunt, ArchonHunt);

    decl_fetch_func!(
        fetch_cambion_drift,
        WorldStateSchema::CambionDrift,
        CambionDrift
    );

    decl_fetch_func!(fetch_cetus_state, WorldStateSchema::CetusState, CetusStatus);

    decl_fetch_func!(
        fetch_conclave_challenges,
        WorldStateSchema::ConclaveChallenge,
        Vec<ConclaveChallenge>
    );

    decl_fetch_func!(
        fetch_construction_porgress,
        WorldStateSchema::ConstructionProgress,
        ConstructionProgress
    );

    decl_fetch_func!(
        fetch_daily_deals,
        WorldStateSchema::DailyDeal,
        Vec<DailyDeal>
    );

    decl_fetch_func!(
        fetch_deep_archimedea,
        WorldStateSchema::DeepArchimedea,
        DeepArchimedea
    );

    decl_fetch_func!(
        fetch_earth_rotation,
        WorldStateSchema::EarthRotation,
        EarthRotation
    );

    decl_fetch_func!(fetch_events, WorldStateSchema::Events, Events);

    decl_fetch_func!(fetch_fissures, WorldStateSchema::Fissures, Vec<Fissures>);

    decl_fetch_func!(
        fetch_flash_sales,
        WorldStateSchema::FlashSales,
        Vec<FlashSales>
    );

    decl_fetch_func!(
        fetch_global_upgrades,
        WorldStateSchema::GlobalUpgrades,
        Vec<GlobalUpgrades>
    );

    decl_fetch_func!(fetch_invasions, WorldStateSchema::Invasion, Vec<Invasion>);

    decl_fetch_func!(fetch_kuva, WorldStateSchema::Kuva, Vec<Kuva>);

    decl_fetch_func!(
        fetch_news_items,
        WorldStateSchema::NewsItems,
        Vec<NewsItems>
    );

    decl_fetch_func!(fetch_nightwave, WorldStateSchema::Nightwave, Nightwave);

    decl_fetch_func!(
        fetch_persistent_enemy,
        WorldStateSchema::PersistentEnemy,
        Vec<PersistentEnemy>
    );

    decl_fetch_func!(fetch_riven, WorldStateSchema::Riven, Riven);

    decl_fetch_func!(
        fetch_sentient_outpost,
        WorldStateSchema::SentientOutpost,
        SentientOutpost
    );

    decl_fetch_func!(
        fetch_sanctuary_status,
        WorldStateSchema::SanctuaryStatus,
        SanctuaryStatus
    );
    decl_fetch_func!(fetch_sortie, WorldStateSchema::Sortie, Sortie);

    decl_fetch_func!(fetch_steel_path, WorldStateSchema::SteelPath, SteelPath);

    decl_fetch_func!(
        fetch_syndicate_mission_nodes,
        WorldStateSchema::SyndicateMissionNodes,
        SyndicateMissionNodes
    );

    decl_fetch_func!(fetch_timestamp, WorldStateSchema::Timestamp, Timestamp);

    decl_fetch_func!(fetch_orb_vallis, WorldStateSchema::OrbVallis, OrbVallis);

    decl_fetch_func!(fetch_varzia, WorldStateSchema::Varzia, Varzia);

    decl_fetch_func!(fetch_void_trader, WorldStateSchema::VoidTrader, VoidTrader);

    decl_fetch_func!(
        fetch_void_traders,
        WorldStateSchema::VoidTraders,
        Vec<VoidTraders>
    );
    /************************************************************************************/
}

pub mod request;
pub mod response;
pub mod schema;
