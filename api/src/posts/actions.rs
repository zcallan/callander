use super::models::{NewPost, Post};
use diesel::prelude::*;
use log::info;
use uuid::Uuid;

use crate::{posts::models, schema::posts};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all_posts(conn: &mut PgConnection) -> Result<Vec<Post>, DbError> {
    let all_posts = posts::table
        .load::<models::Post>(conn)
        .expect("Error loading posts");

    Ok(all_posts)
}

pub fn find_post_by_id(conn: &mut PgConnection, id: String) -> Result<Post, DbError> {
    info!("id: {id}");

    let post = posts::table
        .filter(posts::id.eq(id))
        .first(conn)
        // .load::<Post>(conn)
        .expect("Error loading posts");

    Ok(post)
}

pub fn create_post(conn: &mut PgConnection, new_post: &NewPost) -> Result<Post, DbError> {
    let new_post = Post {
        id: Uuid::new_v4().to_string(),
        title: new_post.title.clone(),
        body: new_post.body.clone(),
        for_date: new_post.for_date.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)?;

    Ok(new_post)
}
