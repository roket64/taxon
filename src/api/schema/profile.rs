use super::decl_schema_struct;
use serde::{Deserialize, Serialize};

decl_schema_struct!(Profile,
  ["accountId"]
  account_id=String,
  ["displayName"]
  display_name=String,
  ["masteryRank"]
  mastery_rank=u32,
  loadout=ProfileLoadout,
  intrinsics=ProfileIntrinsics,
  ["challengeProgress"]
  challenge_progress=Vec<ProfileChallengeProgress>,
  ["guildId"]
  guild_id=String,
  ["guildName"]
  guild_name=String,
  ["guildTier"]
  guild_tier=u32,
  ["guildXp"]
  guild_xp=u32,
  ["guildClass"]
  guild_class=u32,
  ["guildEmblem"]
  guild_emblem=bool,
  ["allianceId"]
  alliance_id=String,
  ["deathMarks"]
  death_marks=Vec<ProfileDeathMarks>,
  harvestable=bool,
  ["deathSquadable"]
  death_squadable=bool,
  created=String,
  ["migratedToConsole"]
  migrated_to_console=bool,
  missions=Vec<ProfileMissions>,
  syndicate=Vec<ProfileSyndicate>,
  ["dailyStanding"]
  daily_standing=Vec<ProfileDailyStanding>,
  ["dailyFocus"]
  daily_focus=u32,
  ["unlockedOperator"]
  unlocked_operator=bool,
  ["unlockedAlignment"]
  unlocked_unlignment=bool,
  ["operatorLoadouts"]
  operator_loadouts=Vec<ProfileOperatorLoadouts>,
  alignment=Vec<ProfileAlignment>
);

decl_schema_struct!(ProfileAlignment, wisdom = u32, alignment = u32);

decl_schema_struct!(ProfileOperatorLoadouts,
  skins=Option<ProfileOperatorLoadoutsSkins>,
  ["operatorAmp"]
  operator_amp=String,
  upgrades=Vec<ProfileOperatorLoadoutsUpgrades>,
  ["abilityOverride"]
  ability_override=ProfileOperatorLoadoutsAbilityOverride,
  ["primaryColor"]
  primary_color=ProfileOperatorLoadoutsPrimaryColor,
  ["sigilColor"]
  sigil_color=ProfileOperatorLoadoutsSigilColor,
  ["eyeColor"]
  eye_color=ProfileOperatorLoadoutsEyeColor,
  facial=ProfileOperatorLoadoutsFacial,
  cloth=ProfileOperatorLoadoutsCloth
);

decl_schema_struct!(ProfileOperatorLoadoutsCloth);

decl_schema_struct!(ProfileOperatorLoadoutsFacial);

decl_schema_struct!(ProfileOperatorLoadoutsEyeColor);

decl_schema_struct!(ProfileOperatorLoadoutsSigilColor);

decl_schema_struct!(ProfileOperatorLoadoutsPrimaryColor);

decl_schema_struct!(ProfileOperatorLoadoutsAbilityOverride);

decl_schema_struct!(ProfileOperatorLoadoutsUpgrades);

decl_schema_struct!(ProfileOperatorLoadoutsSkins);

decl_schema_struct!(ProfileDailyStanding,
  daily=u32,
  conclave=u32,
  simaris=u32,
  ostrons=u32,
  quillis=u32,
  solaris=u32,
  ["ventKids"]
  vent_kids=u32,
  entrati=u32,
  necraloid=u32,
  holdfasts=u32,
  kahl=u32,
  cavia=u32
);

decl_schema_struct!(ProfileSyndicate, name = String, standing = u32, title = u32);

decl_schema_struct!(ProfileMissions,
  node=String,
  ["nodeKey"]
  node_key=String,
  ["missionType"]
  mission_type=String,
  faction=String,
  completes=String,
  tier=Option<u32>
);

decl_schema_struct!(ProfileDeathMarks);

decl_schema_struct!(ProfileChallengeProgress, name = String, progress = u32);

decl_schema_struct!(
    ProfileIntrinsics,
    railjack = u32,
    engineering = u32,
    gunnery = u32,
    piloting = u32,
    tactical = u32,
    command = u32,
    drifter = u32,
    riding = u32,
    combat = u32,
    opportunity = u32,
    endurance = u32
);

decl_schema_struct!(ProfileLoadout,
  ["weaponSkins"]
  weapon_skins=ProfileLoadoutWeaponSkins,
  suits=ProfileLoadoutSuits,
  secondary=ProfileLoadoutSecondary,
  primary=ProfileLoadoutPrimary,
  melee=ProfileLoadoutMelee,
  ["xpInfo"]
  xp_info=ProfileLoadoutXpInfo
);

/********************************* FUCK ME *********************************/
decl_schema_struct!(ProfileLoadoutXpInfo);

decl_schema_struct!(ProfileLoadoutMelee);

decl_schema_struct!(ProfileLoadoutPrimary);

decl_schema_struct!(ProfileLoadoutSecondary);

decl_schema_struct!(ProfileLoadoutSuits);

decl_schema_struct!(ProfileLoadoutWeaponSkins);
/*************************************************************************/

decl_schema_struct!(Stats,
  ["guildName"]
  guild_name=String,
  ["missionsCompleted"]
  missions_completed=u32,
  ["missionsQuit"]
  missions_quit=u32,
  ["missionsInterrupted"]
  missions_interrupted=u32,
  ["missionsDumped"]
  missions_dumped=u32,
  ["pickupCount"]
  pickup_count=u32,
  weapons=Vec<StatsWeapons>,
  enemies=Vec<StatsEnemies>,
  ["meleeKills"]
  melee_kills=u32,
  abilities=Vec<StatsAbilities>,
  ["cipherSolved"]
  cipher_solved=u32,
  income=u32,
  ["timePlayedSec"]
  time_played_sec=u32,
  ["cipherTime"]
  cipher_time=u32,
  rating=u32,
  rank=u32,
  deaths=u32,
  ["playerLevel"]
  player_level=u32,
  missions=Vec<StatsMissions>,
  healcount=u32,
  ["breedGrounds"]
  breed_grounds=StatsBreedGrounds,
  ["gradivusDilemma"]
  gradivus_dilema=StatsGradivusDillema,
  scans=Vec<StatsScans>,
  ["reviveCount"]
  revive_count=u32,
  ["fomorianEventScore"]
  fomorian_event_score=u32,
  pvp=Vec<StatsPvp>,
  lunaro=StatsLunaro,
  ["dojoObstacleScore"]
  dojo_obstacle_score=u32,
  ["pvpGamesPendingMask"]
  pvp_games_pendig_mask=u32,
  ["pacifismDefect"]
  pacifism_defect=u32,
  ["sentinelGameScore"]
  sentinel_game_score=u32,
  ["amalgamEventMaxScore"]
  amalgam_event_max_score=u32,
  ["scarletSpear"]
  scarlet_spear=StatsScarletSpear,
  ["orphixVenomSpear"]
  orphix_venom_spaer=u32,
  ["kDriveRaces"]
  k_drive_races=Vec<StatsKDriveRaces>
);

decl_schema_struct!(StatsKDriveRaces);

decl_schema_struct!(StatsScarletSpear);

decl_schema_struct!(StatsLunaro);

decl_schema_struct!(StatsPvp);

decl_schema_struct!(StatsScans);

decl_schema_struct!(StatsGradivusDillema);

decl_schema_struct!(StatsBreedGrounds);

decl_schema_struct!(StatsMissions);

decl_schema_struct!(StatsAbilities);

decl_schema_struct!(StatsEnemies);

decl_schema_struct!(StatsWeapons);

#[derive(Debug)]
pub enum ProfileSchema {
    Profile(Profile),
    Stats(Stats),
}

#[derive(Debug)]
pub enum ProfileKind {
    Profile,
    Stats,
}
