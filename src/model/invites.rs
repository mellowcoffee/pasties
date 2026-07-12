use chrono::Utc;

pub struct Invite {
    pub code: String,
    pub created_by: u64,
    pub used_by: Option<u64>,
    pub created_at: chrono::DateTime<Utc>,
    pub used_at: Option<chrono::DateTime<Utc>>,
}
