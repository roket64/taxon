pub mod err;
pub mod items;
pub mod profile;
pub mod riven;
pub mod searchable;
pub mod static_processing;
pub mod unstable;
pub mod worldstate;

// export everything
// pub use items::*;
// pub use profile::*;
// pub use riven::*;
// pub use searchable::*;
// pub use static_processing::*;
// pub use unstable::*;
pub use worldstate::*;

#[derive(Debug)]
pub enum SchemaKind {
    // Items(ItemsKind),
    // Profile(ProfileKind),
    // Riven(RivenKind),
    // Searchable(SearchableKind),
    // StaticProcessing(StaticProcessingKind),
    // Unstable(UnstableKind),
    WorldState(WorldStateKind),
}
