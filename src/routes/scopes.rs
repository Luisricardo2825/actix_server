use crate::routes::posts::delete_post;

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
    use crate::routes::posts::{alter_post, create_post, get_posts};

    actix_web::web::scope("posts")
        .service(get_posts::main)
        .service(create_post::main)
        .service(alter_post::main)
        .service(delete_post::main)
}
