use crate::api::ApiHandler;
use api::schema::WorldStateKind;
use log::debug;

#[tokio::main]
async fn main() {
    // load enviroment vars
    dotenv::dotenv().ok();

    // initiate logging crate using file
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let kind = WorldStateKind::ArchonHunt;
    let handler = ApiHandler::new();
    let what = handler.fetch(&kind).await;
    debug!("what: {:#?}", what);
}

pub mod api;
pub mod utils;
