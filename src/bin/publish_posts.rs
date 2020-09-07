extern crate diesel_demo;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use self::models::Post;


fn main() {
    use diesel_demo::schema::posts::dsl::{posts, published};

    let id: i32 = 220003; 
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published post {}", post.title);
}