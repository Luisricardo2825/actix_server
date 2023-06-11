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

pub fn posts_route() -> actix_web::Scope {
    use crate::controller::posts::post_controller::PostController;

    actix_web::web::scope("posts")
        .route("/", web::post().to(PostController::create))
        .route("/{id}/", web::get().to(PostController::find))
        .route("/", web::get().to(PostController::find_all))
        .route("/", web::patch().to(PostController::update))
        .route("/", web::delete().to(PostController::delete))
}

pub fn users_route() -> actix_web::Scope {
    use crate::controller::users::user_controller::UserController;

    actix_web::web::scope("users")
        .route("/", web::post().to(UserController::create))
        .route("/{id}/", web::get().to(UserController::find))
        .route("/", web::get().to(UserController::find_all))
        .route("/", web::patch().to(UserController::update))
        .route("/", web::delete().to(UserController::delete))
}

pub fn login_route() -> actix_web::Scope {
    use crate::controller::login::auth_controller::AuthController;

    actix_web::web::scope("login").route("/", web::post().to(AuthController::login))
}
