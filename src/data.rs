use sqlx::{Pool, Postgres};

use crate::config::AdminSettings;

#[derive(Clone, Debug)]
pub struct ApplicationData {
    pub admin: AdminSettings,
    pub pool: Pool<Postgres>,
}
