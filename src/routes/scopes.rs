/**
Route for posts.

**Endpoint**: `/posts`
### Methods:
 ```
 * GET - FIXME: Add query params to return 1 post
 * POST
 * PATCH
 * DELETE - TODO: Delete route
 ```
```
return actix_web::Scope
```
 */

pub fn posts_route() -> actix_web::Scope {
    use crate::routes::posts::{alter_post, create_post, delete_post, get_posts};

    actix_web::web::scope("posts")
        .service(get_posts::main)
        .service(create_post::main)
        .service(alter_post::main)
        .service(delete_post::main)
}

pub fn users_route() -> actix_web::Scope {
    use crate::routes::users::{create_user, get_users};

    actix_web::web::scope("users")
        .service(get_users::main)
        .service(create_user::main)
}
