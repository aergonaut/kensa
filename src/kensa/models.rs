use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Feature {
    pub id: Uuid,
    pub name: String,
    pub feature_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}
