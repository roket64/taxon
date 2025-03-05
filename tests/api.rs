use reqwest::Url;
use taxonlib::api::request::UrlBuilder;
use taxonlib::api::ApiHandler;
use taxonlib::api::{request, schema::*};

macro_rules! impl_response_fn {
    ($(
        $(#[deprecated = $deprecated: literal])?
        {$kind: path}=> fn $fn_name: ident();
    )*) => {
        $(
          $(#[deprecated = $deprecated])?
          #[tokio::test]
          async fn $fn_name() {
            let paths = UrlBuilder::get_worldstate_req_path(&$kind);
            let query = Some("language=en");
            let response = ApiHandler::fetch_contents(paths, query).await;

            assert!(response.status().is_success());
        })*};
}

impl_response_fn!(
{WorldStateKind::Alerts} => fn test_alerts_status();
{WorldStateKind::Arbitration} => fn test_arbitration_status();
{WorldStateKind::ArchonHunt} => fn test_archon_hunt_stauts();
{WorldStateKind::CambionDrift} => fn test_cambion_drift_status();
{WorldStateKind::CetusStatus} => fn test_cetus_status_status();
{WorldStateKind::ConclaveChallenge} => fn test_conclave_challenge_status();
{WorldStateKind::ConstructionProgress} => fn test_construction_progress_status();
{WorldStateKind::DailyDeal} => fn test_daily_deal_status();
{WorldStateKind::DeepArchimedea} => fn test_deep_archimedea_status();
{WorldStateKind::EarthRotation} => fn test_earth_rotation_status();
{WorldStateKind::Events} => fn test_events_status();
{WorldStateKind::Fissures} => fn test_fissures_status();
{WorldStateKind::FlashSales} => fn test_flash_sales_status();
{WorldStateKind::GlobalUpgrades} => fn test_global_upgrades_status();
{WorldStateKind::Invasion} => fn test_invasion_status();
{WorldStateKind::Kuva} => fn test_kuva_status();
{WorldStateKind::NewsItems} => fn test_news_items_status();
{WorldStateKind::Nightwave} => fn test_nightwave_status();
{WorldStateKind::PersistentEnemy} => fn test_persistent_enemy_status();
{WorldStateKind::Riven} => fn test_riven_status();
{WorldStateKind::SentientOutpost} => fn test_sentient_outpost_status();
{WorldStateKind::SanctuaryStatus} => fn test_sanctuary_status_status();
{WorldStateKind::Sortie} => fn test_sortie_status();
{WorldStateKind::SteelPath} => fn test_steel_path_status();
{WorldStateKind::SyndicateMissionNodes} => fn test_syndicate_mission_nodes_status();
{WorldStateKind::Timestamp} => fn test_timestamp_status();
{WorldStateKind::OrbVallis} => fn test_orb_vallis_status();
{WorldStateKind::Varzia} => fn test_varzia_status();
{WorldStateKind::VoidTrader} => fn test_void_trader_status();
{WorldStateKind::VoidTraders} => fn test_void_traders_status();
);

#[tokio::test]
pub async fn test_url_builder() {
    let mut paths: Vec<String> = Vec::new();
    paths.push("pc".to_string());
    let url = request::UrlBuilder::build_request_url(paths, Some("language=en")).unwrap();
    assert_eq!(
        Url::parse("https://api.warframestat.us/pc?language=en").unwrap(),
        url
    );
}