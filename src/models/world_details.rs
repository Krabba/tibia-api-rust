use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

use super::{GameWorldType, Location, Player, PvpType, TransferType};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorldDetails {
    #[schema(example = "Antica")]
    pub name: String,
    pub is_online: bool,
    #[schema(example = "152")]
    pub players_online_count: u32,
    #[schema(example = "1211")]
    pub players_online_record: u32,
    #[schema(value_type = String, format = DateTime)]
    pub players_online_record_date: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub creation_date: DateTime<Utc>,
    pub location: Location,
    pub pvp_type: PvpType,
    #[schema(example = json!(["Rise of Devovorga", "The Lightbearer"]))]
    pub world_quest_titles: Vec<String>,
    pub battl_eye: bool,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub battl_eye_date: Option<DateTime<Utc>>,
    pub game_world_type: GameWorldType,
    pub transfer_type: Option<TransferType>,
    pub premium_required: bool,
    pub players_online: Vec<Player>,
}
