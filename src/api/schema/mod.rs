pub mod err;
pub mod items;
pub mod profile;
pub mod riven;
pub mod searchable;
pub mod static_processing;
pub mod unstable;
pub mod worldstate;

macro_rules! decl_schema_struct {
    ($strt_name: ident,
        $($([$rename: literal])*
        $key: ident=$val: ty),*) => {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct $strt_name {
            $(
                $(#[serde(rename = $rename)])*
                // wrapping it in Option<> to prevent mapping errors
                $key: Option<$val>
            ),*
        }
    };

    ($strt_name: ident) => {
        decl_schema_struct!($strt_name,);
    };
}

pub(crate) use decl_schema_struct;

// export everything
// pub use items::*;
pub use profile::*;
// pub use riven::*;
// pub use searchable::*;
// pub use static_processing::*;
// pub use unstable::*;
pub use worldstate::*;

#[derive(Debug)]
pub enum SchemaKind {
    // Items(ItemsKind),
    Profile(ProfileKind),
    // Riven(RivenKind),
    // Searchable(SearchableKind),
    // StaticProcessing(StaticProcessingKind),
    // Unstable(UnstableKind),
    WorldState(WorldStateKind),
}
