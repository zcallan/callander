use super::models::{NewPost, Post, PostsFindAllQuery};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    posts::models,
    schema::posts,
    utils::pagination::{Paginate, Paginated},
};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all_posts(
    conn: &mut PgConnection,
    user_id: String,
    options: &PostsFindAllQuery,
) -> Result<Paginated<Vec<Post>>, DbError> {
    let page = options.page.max(1);
    let per_page = options.per_page.unwrap_or(10).min(20);

    let query = posts::table
        .order(posts::created_at.desc())
        .filter(posts::user_id.eq(user_id))
        .paginate(page)
        .per_page(per_page);

    let (posts, total_items, total_pages) = query.load_and_count_pages::<models::Post>(conn)?;

    let paginated = Paginated::<Vec<Post>> {
        items: posts,
        per_page: per_page,
        total_items: total_items,
        total_pages: total_pages,
        current_page: page,
        offset: (page - 1) * per_page,
        has_next_page: page < total_pages,
        has_prev_page: page > 1,
    };

    Ok(paginated)
}

pub fn find_post_by_id(
    conn: &mut PgConnection,
    user_id: String,
    id: String,
) -> Result<Post, DbError> {
    let post = posts::table
        .filter(posts::id.eq(id))
        .filter(posts::user_id.eq(user_id))
        .first(conn)
        .expect("Error loading posts");

    Ok(post)
}

pub fn create_post(
    conn: &mut PgConnection,
    user_id: String,
    new_post: &NewPost,
) -> Result<Post, DbError> {
    let new_post = Post {
        id: Uuid::new_v4().to_string(),
        title: new_post.title.clone(),
        body: new_post.body.clone(),
        for_date: new_post.for_date.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        user_id: user_id.clone(),
    };

    let result = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)?;

    Ok(result)
}
