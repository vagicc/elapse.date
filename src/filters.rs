use crate::routes::article_route;
use crate::routes::home_route;
use warp::Filter;

pub fn all_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let favicon = warp::get()
        .and(warp::path("favicon.ico"))
        .and(warp::path::end())
        .and(warp::fs::file("./static/favicon.ico"));

    let dir = warp::path("static").and(warp::fs::dir("./static"));
    let home = home_route::index();
    let article = article_route::list();

    let routes = home.or(dir).or(favicon).or(article);
    routes
}
