use serde::{Deserialize, Serialize};

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

//TODO: implement this
decl_schema_struct!(WorldState);

// Array
decl_schema_struct!(AlertsMissionRewardCountedItems,
    count=u32,
    ["type"]
    tp=String
);

decl_schema_struct!(AlertsMissionReward,
    ["countedItems"]
    counted_items=Vec<AlertsMissionRewardCountedItems>,
    thumbnail=String,
    color=u32,
    credits=u32,
    ["asString"]
    as_string=String,
    items=Vec<String>,
    ["itemString"]
    item_string=String
);

decl_schema_struct!(AlertsMission,
    reward = AlertsMissionReward,
    node=String,
    ["nodeKey"]
    node_key=String,
    faction=String,
    ["factionKey"]
    faction_key=String,
    ["maxEnemyLevel"]
    max_enemy_level=u32,
    ["minEnemyLevel"]
    min_enemy_level=u32,
    ["maxWaveNum"]
    max_wave_num=u32,
    ["type"]
    tp=String,
    ["typeKey"]
    tp_key=String,
    nightmare=bool,
    ["archwingRequired"]
    archwing_required=bool,
    ["isSharkwing"]
    is_sharkwing=bool,
    ["enemySpec"]
    enemy_spec=String,
    ["levelOverride"]
    level_override=String,
    ["advancedSpawners"]
    advanced_spawners=Vec<String>,
    ["requiredItem"]
    required_item=Vec<String>,
    ["consumeRequiredItems"]
    consume_required_items=bool,
    ["readersAlwaysAllowed"]
    readers_always_allowed=bool,
    ["levelAuras"]
    level_auras=Vec<String>,
    description=String
);

// Array
decl_schema_struct!(Alerts,
    id=String,
    activation=String,
    expiry=String,
    ["startString"]
    start_string=String,
    active=bool,
    mission=AlertsMission,
    expired=bool,
    eta=String,
    ["rewardTypes"]
    reward_types=Vec<String>
);

decl_schema_struct!(Arbitration,
    id=String,
    activation=String,
    expiry=String,
    ["startString"]
    start_string=String,
    active=bool,
    node=String,
    enemy=String,
    ["enemyKey"]
    enemy_key=String,
    ["type"]
    tp=String,
    ["typeKey"]
    tp_key=String,
    archwing=bool,
    sharkwing=bool
);

decl_schema_struct!(ArchonHuntMission,
    node=String,
    ["nodeKey"]
    node_key=String,
    ["type"]
    tp=String,
    ["typeKey"]
    tp_key=String,
    nightmare=bool,
    ["archwingRequired"]
    archwing_required=bool,
    ["isSharkwing"]
    is_sharkwing=bool,
    ["advancedSpawners"]
    advanced_spawners=Vec<String>,
    ["requiredItems"]
    required_items=Vec<String>,
    ["levelAuras"]
    level_auras=Vec<String>
);

decl_schema_struct!(ArchonHunt,
    id=String,
    activation=String,
    expiry=String,
    ["startString"]
    start_string=String,
    active=bool,
    ["rewardPool"]
    reward_pool=String,
    missions=Vec<ArchonHuntMission>,
    boss=String,
    faction=String,
    ["factionKey"]
    faction_key=String,
    expired=bool,
    eta=String
);

decl_schema_struct!(CambionDrift,
    id=String,
    expiry=String,
    activation=String,
    state=String,
    ["timeLeft"]
    time_left=String
);

decl_schema_struct!(CetusStatus,
    id=String,
    activation=String,
    expiry=String,
    ["startString"]
    start_string=String,
    active=bool,
    ["isDay"]
    is_day=bool,
    state=String,
    ["timeLeft"]
    time_left=String,
    ["isCetus"]
    is_cetus=bool,
    ["shortString"]
    short_string=String
);

// Array
decl_schema_struct!(ConclaveChallenge,
    mode=String,
    amount=u32,
    eta=String,
    expired=bool,
    ["endString"]
    end_string=String,
    daily=bool,
    description=String,
    id=String,
    expiry=String,
    ["asString"]
    as_string=String,
    category=String,
    ["rootChallenge"]
    root_challenge=bool
);

decl_schema_struct!(ConstructionProgress,
    id=String,
    ["fomorianProgress"]
    fomorian_progress=String,
    ["razorbackProgress"]
    razorback_progress=String,
    ["unknownProgress"]
    unknown_progress=String
);

// Array, should be wrapped in Vec<>
decl_schema_struct!(DailyDeal,
    sold=u32,
    item=String,
    ["uniqueName"]
    unique_name=String,
    total=u32,
    eta=String,
    ["originalPrice"]
    original_price=u32,
    ["salePrice"]
    sale_price=u32,
    discount=u32,
    expiry=String,
    id=String
);

decl_schema_struct!(DarkSectorHistory);

// deprecated
decl_schema_struct!(DarkSector,
    ["defenderMOTD"]
    defender_motd=String,
    ["deployerName"]
    deployer_name=String,
    ["defenderDeploymentActivation"]
    defender_deployment_activation=u32,
    ["defenderName"]
    defender_name=String,
    ["deployerClan"]
    deployer_clan=String,
    ["isAlliance"]
    is_alliance=bool,
    id=String,
    history=Vec<DarkSectorHistory>
);

decl_schema_struct!(DeepArchimedeaMission);

decl_schema_struct!(DeepArchimedeaPersonalModifier);

// I'm getting WorldState response when tried to get this wtf
decl_schema_struct!(DeepArchimedea,
    id=String,
    activation=String,
    expiry=String,
    missions=Vec<DeepArchimedeaMission>,
    ["personalModifiers"]
    personal_modifiers=Vec<DeepArchimedeaPersonalModifier>
);

decl_schema_struct!(EarthRotation,
    id=String,
    activation=String,
    expiry=String,
    ["startString"]
    start_stirng=String,
    active=bool,
    ["isDay"]
    is_day=bool,
    ["timeLeft"]
    time_left=String
);

decl_schema_struct!(Events,);

// Array
decl_schema_struct!(Fissures,
    id=String,
    activation=String,
    expiry=String,
    ["startString"]
    start_string=String,
    active=bool,
    node=String,
    expired=bool,
    eta=String,
    ["missionType"]
    mission_type=String,
    ["missionKey"]
    mission_key=String,
    tier=String,
    ["tierNum"]
    tier_num=u32,
    enemy=String,
    ["enemyKey"]
    enemy_key=String,
    ["isStorm"]
    is_storm=bool,
    ["isHard"]
    is_hard=bool
);

// Array
decl_schema_struct!(FlashSales,
    item=String,
    expired=String,
    eta=String,
    discount=u32,
    ["premiumOverride"]
    premium_override=u32,
    ["isPopular"]
    is_popular=bool,
    ["isFeatured"]
    is_featured=bool
);

// Array
decl_schema_struct!(GlobalUpgrades,
    start=String,
    end=String,
    upgrade=String,
    operation=String,
    ["operationSymbol"]
    operation_symbol=String,
    ["upgradeOperationValue"]
    upgrade_operation_value=u32,
    expired=bool,
    eta=String,
    desc=String
);

// Array
decl_schema_struct!(Invasion,
    id=String,
    activation=String,
    expiry=String,
    ["startString"]
    start_string=String,
    active=bool,
    // TODO: implement this field
    attacker=Vec<String>,
    completed=bool,
    completion=u32,
    count=f32,
    // TODO: implement this field
    defender=Vec<String>,
    desc=String,
    eta=String,
    node=String,
    ["nodeKey"]
    node_key=String,
    ["requiredRuns"]
    required_runs=u32,
    ["rewardTypes"]
    reward_types=Vec<String>,
    vslnfestation=bool
);

// Array
decl_schema_struct!(Kuva,
    id=String,
    activation=String,
    expiry=String,
    ["startString"]
    start_string=String,
    active=bool,
    node=String,
    enemy=String,
    ["enemyKey"]
    enemy_key=String,
    ["type"]
    tp=String,
    ["typeKey"]
    tp_key=String,
    archwing=bool,
    shartwing=bool
);

// TODO: implement this
decl_schema_struct!(Translations);

// Array
decl_schema_struct!(NewsItems,
    date=String,
    ["imageLink"]
    image_link=String,
    eta=String,
    ["primeAccess"]
    prime_access=bool,
    stream=bool,
    translations=Vec<Translations>,
    link=String,
    update=bool,
    id=String,
    ["asString"]
    as_string=String,
    message=String,
    priority=bool
);

// TODO: implement this
decl_schema_struct!(Params);
// TODO: implement this
decl_schema_struct!(PossibleChallanges);
// TODO: implement this
decl_schema_struct!(ActiveChallanges);

decl_schema_struct!(Nightwave,
    id=String,
    activation=String,
    expiry=String,
    ["startString"]
    start_string=String,
    active=bool,
    params=Vec<String>,
    ["rewardTypes"]
    reward_types=Vec<String>,
    season=u32,
    tag=String,
    phase=u32,
    ["possibleChallanges"]
    possible_challanges=Vec<PossibleChallanges>,
    ["activeChallanges"]
    active_challanges=Vec<ActiveChallanges>
);

// Array
decl_schema_struct!(PersistentEnemy,
    ["locationTag"]
    location_tag=String,
    ["agentType"]
    agent_type=String,
    rank=u32,
    ["healthPercent"]
    health_percent=u32,
    ["fleetDamage"]
    fleet_damage=u32,
    region=String,
    ["lastDiscoveredTime"]
    last_discovered_time=String,
    ["lastDiscoveredAt"]
    last_discovered_at=String,
    ["isDiscovered"]
    is_discovered=bool,
    ["isUsingTicketing"]
    is_using_ticketing=bool,
    pid=String
);

decl_schema_struct!(Riven);

//TODO: implement this
decl_schema_struct!(SentientOutpostMission);
//TODO: implement this
decl_schema_struct!(SentientOutpostPrevious);

decl_schema_struct!(
    SentientOutpost,
    mission = SentientOutpostMission,
    active = bool,
    id = String,
    activation = String,
    expiry = String,
    previous = String
);

decl_schema_struct!(SanctuaryStatus,
    target=String,
    ["isTargetActive"]
    is_target_active=bool,
    ["asString"]
    as_string=String
);

//TODO: implement this
decl_schema_struct!(SortieVariants);

decl_schema_struct!(Sortie,
    id=String,
    activation=String,
    expiry=String,
    ["startString"]
    start_string=String,
    active=bool,
    ["rewardPool"]
    reward_pool=String,
    variants=Vec<SortieVariants>,
    boss=String,
    faction=String,
    ["factionKey"]
    faction_key=String,
    expired=bool,
    eta=String
);

decl_schema_struct!(SteelPath);

//TODO: implement this
decl_schema_struct!(SyndicateJob);

decl_schema_struct!(SyndicateMissionNodes,
    nodes=Vec<String>,
    eta=String,
    jobs=Vec<SyndicateJob>,
    syndicate=String,
    id=Vec<String>,
    expiry=String,
    activation=String
);

decl_schema_struct!(Timestamp, timestamp = String);

decl_schema_struct!(OrbVallis,
    id=String,
    expiry=String,
    ["timeLeft"]
    time_left=String,
    ["isWarm"]
    is_warm=bool
);

//TODO: implement this
decl_schema_struct!(VarziaInventory);

decl_schema_struct!(Varzia,
    id=String,
    activation=String,
    expiry=String,
    ["startString"]
    start_string=String,
    active=bool,
    character=String,
    location=String,
    inventory=Vec<VarziaInventory>,
    psld=String,
    ["endString"]
    end_string=String
);

// `VoidTrader` shares same schema with `Varzia`
decl_schema_struct!(VoidTrader,
    id=String,
    activation=String,
    expiry=String,
    ["startString"]
    start_string=String,
    active=bool,
    character=String,
    location=String,
    inventory=Vec<VarziaInventory>,
    psld=String,
    ["endString"]
    end_string=String
);

// Array of `VoidTrader`
decl_schema_struct!(VoidTraders,
    id=String,
    activation=String,
    expiry=String,
    ["startString"]
    start_string=String,
    active=bool,
    character=String,
    location=String,
    inventory=Vec<VarziaInventory>,
    psld=String,
    ["endString"]
    end_string=String
);

#[derive(Debug)]
pub enum WorldStateSchema {
    WorldState(WorldState),
    Alerts(Vec<Alerts>),
    Arbitration(Arbitration),
    ArchonHunt(ArchonHunt),
    CambionDrift(CambionDrift),
    CetusState(CetusStatus),
    ConclaveChallenge(Vec<ConclaveChallenge>),
    ConstructionProgress(ConstructionProgress),
    DailyDeal(Vec<DailyDeal>),
    DeepArchimedea(DeepArchimedea),
    EarthRotation(EarthRotation),
    Events(Events),
    Fissures(Vec<Fissures>),
    FlashSales(Vec<FlashSales>),
    GlobalUpgrades(Vec<GlobalUpgrades>),
    Invasion(Vec<Invasion>),
    Kuva(Vec<Kuva>),
    NewsItems(Vec<NewsItems>),
    Nightwave(Nightwave),
    PersistentEnemy(Vec<PersistentEnemy>),
    Riven(Riven),
    SentientOutpost(SentientOutpost),
    SanctuaryStatus(SanctuaryStatus),
    Sortie(Sortie),
    SteelPath(SteelPath),
    SyndicateMissionNodes(SyndicateMissionNodes),
    Timestamp(Timestamp),
    OrbVallis(OrbVallis),
    Varzia(Varzia),
    VoidTrader(VoidTrader),
    VoidTraders(Vec<VoidTraders>),
}

#[derive(Debug)]
pub enum WorldStateKind {
    WorldState,
    Alerts,
    Arbitration,
    ArchonHunt,
    CambionDrift,
    CetusState,
    ConclaveChallenge,
    ConstructionProgress,
    DailyDeal,
    DeepArchimedea,
    EarthRotation,
    Events,
    Fissures,
    FlashSales,
    GlobalUpgrades,
    Invasion,
    Kuva,
    NewsItems,
    Nightwave,
    PersistentEnemy,
    Riven,
    SentientOutpost,
    SanctuaryStatus,
    Sortie,
    SteelPath,
    SyndicateMissionNodes,
    Timestamp,
    OrbVallis,
    Varzia,
    VoidTrader,
    VoidTraders,
}
