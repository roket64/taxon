use api::schema::*;
use log::debug;
use strum::IntoEnumIterator;
use taxonlib::api::{self, ApiHandler};

#[tokio::main]
async fn main() {
    // load enviroment vars
    dotenv::dotenv().ok();

    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    for kind in WorldStateKind::iter() {
        debug!("fetching {:#?}...", &kind);
        let worldstate = ApiHandler::fetch_worldstate_contents(&kind, Some("langugae=kr")).await;
        debug!("parsed: {:#?}", worldstate);
    }
}
