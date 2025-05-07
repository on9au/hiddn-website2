use sqlx::MySqlPool;

pub struct AnnouncementRepository {
    pool: MySqlPool,
}
