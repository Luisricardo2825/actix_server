use actix_web::web;

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

pub struct Scopes;

impl Scopes {
    pub fn posts_scope() -> actix_web::Scope {
        use crate::services::posts::PostsRoute;

        actix_web::web::scope("posts")
            .route("/", web::post().to(PostsRoute::create))
            .route("/{id}/", web::get().to(PostsRoute::find))
            .route("/", web::get().to(PostsRoute::find_all))
            .route("/", web::patch().to(PostsRoute::update))
            .route("/", web::delete().to(PostsRoute::delete))
    }

    pub fn users_scope() -> actix_web::Scope {
        use crate::services::users::UsersRoute;

        actix_web::web::scope("users")
            .route("/", web::post().to(UsersRoute::create))
            .route("/{id}/", web::get().to(UsersRoute::find))
            .route("/", web::get().to(UsersRoute::find_all))
            .route("/", web::patch().to(UsersRoute::update))
            .route("/", web::delete().to(UsersRoute::delete))
    }

    pub fn login_scope() -> actix_web::Scope {
        use crate::services::auth::AuthService;
        actix_web::web::scope("login").route("/", web::post().to(AuthService::login))
    }
}
