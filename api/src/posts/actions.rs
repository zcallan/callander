use super::models::{NewPost, Post};
use diesel::prelude::*;
use log::info;
use uuid::Uuid;

use crate::posts::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all_posts(conn: &mut PgConnection) -> Result<Vec<Post>, DbError> {
    use crate::schema::posts::dsl::*;

    let all_posts = posts
        .load::<models::Post>(conn)
        .expect("Error loading posts");

    Ok(all_posts)
}

pub fn create_post(conn: &mut PgConnection, new_post: &NewPost) -> Result<Post, DbError> {
    use crate::schema::posts::dsl::*;

    let new_post = Post {
        id: Uuid::new_v4().to_string(),
        title: new_post.title.clone(),
        body: new_post.body.clone(),
        for_date: new_post.for_date.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    info!("New post: {:?}", new_post);

    diesel::insert_into(posts).values(&new_post).execute(conn)?;

    Ok(new_post)
}
