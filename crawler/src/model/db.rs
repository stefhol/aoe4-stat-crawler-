use time::{PrimitiveDateTime};
use uuid::Uuid;


#[derive(sqlx::FromRow)]
pub struct Player{
    pub id:Uuid,
    pub rl_user_id:i64,
    pub username:String,
    pub region:String,
    #[sqlx(default)]
    pub avatar_url:Option<String>
}
#[derive(sqlx::FromRow)]
pub struct MatchHistory{
    pub id:Uuid,
    pub time:PrimitiveDateTime,
    pub elo:i32,
    pub elo_rating: i32,
    pub rank:i32,
    pub wins:i32,
    pub losses:i32,
    pub win_streak:i32,
    pub match_type:String
}