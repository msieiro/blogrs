#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection_manager = ConnectionManager::<PgConnection>::new(&database_url);

    Pool::builder()
        .build(connection_manager)
        .expect("COULD NOT CREATE THE POOL")
}

use self::models::{NewPost, Post};

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn find_all_posts<'a>(conn: &PgConnection) -> Vec<Post> {
    use crate::schema::posts::dsl::posts;

    let posts_result = posts.load::<Post>(conn).expect("Error loading posts");
    println!("Size of posts: {}", posts_result.len());

    return posts_result;
}

/* pub fn update_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
} */

/*
pub fn delete_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use crate::schema::posts::dsl::posts;

    diesel::delete(posts.filter(title.like(title)))
        .execute(conn)
        .expect("Error saving new post")
} */
