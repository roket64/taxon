#[cfg(test)]
mod schema_test {
    use taxonlib::api::request::*;
    use taxonlib::api::{schema::*, ApiHandler};

    macro_rules! impl_test_fn {
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
            let chunk = response.bytes().await.unwrap();

            match $kind {
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
        })*};
    }

    impl_test_fn!({WorldStateKind::Alerts} => fn test_alerts();
    {WorldStateKind::Arbitration} => fn test_arbitration();
    {WorldStateKind::ArchonHunt} => fn test_archon_hunt();
    {WorldStateKind::CambionDrift} => fn test_cambion_drift();
    {WorldStateKind::CetusStatus} => fn test_cetus_status();
    {WorldStateKind::ConclaveChallenge} => fn test_conclave_challenge();
    {WorldStateKind::ConstructionProgress} => fn test_construction_progress();
    {WorldStateKind::DailyDeal} => fn test_daily_deal();
    {WorldStateKind::DeepArchimedea} => fn test_deep_archimedea();
    {WorldStateKind::EarthRotation} => fn test_earth_rotation();
    {WorldStateKind::Events} => fn test_events();
    {WorldStateKind::Fissures} => fn test_fissures();
    {WorldStateKind::FlashSales} => fn test_flash_sales();
    {WorldStateKind::GlobalUpgrades} => fn test_global_upgrades();
    {WorldStateKind::Invasion} => fn test_invasion();
    {WorldStateKind::Kuva} => fn test_kuva();
    {WorldStateKind::NewsItems} => fn test_news_items();
    {WorldStateKind::Nightwave} => fn test_nightwave();
    {WorldStateKind::PersistentEnemy} => fn test_persistent_enemy();
    {WorldStateKind::Riven} => fn test_riven();
    {WorldStateKind::SentientOutpost} => fn test_sentient_outpost();
    {WorldStateKind::SanctuaryStatus} => fn test_sanctuary_status();
    {WorldStateKind::Sortie} => fn test_sortie();
    {WorldStateKind::SteelPath} => fn test_steel_path();
    {WorldStateKind::SyndicateMissionNodes} => fn test_syndicate_mission_nodes();
    {WorldStateKind::Timestamp} => fn test_timestamp();
    {WorldStateKind::OrbVallis} => fn test_orb_vallis();
    {WorldStateKind::Varzia} => fn test_varzia();
    {WorldStateKind::VoidTrader} => fn test_void_trader();
    {WorldStateKind::VoidTraders} => fn test_void_traders(););
}
