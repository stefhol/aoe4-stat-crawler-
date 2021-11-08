use anyhow::Error;
use sqlx::{PgPool, query};

async fn get_match_history(pool:PgPool,rl_user_id:i64)->Result<(),Error>{
    let account = sqlx::query!("select * from player where rl_user_id = $1",rl_user_id)
    .fetch_one(&pool)
    .await?;
    todo!()
}