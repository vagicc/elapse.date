use crate::common::get_env;

mod common;
mod db;
mod filters;
mod format_logger;
mod handlers;
mod models;
mod pager;
mod routes;
mod schema;
mod template;

#[macro_use]
extern crate diesel;

#[tokio::main]
async fn main() {
    println!("https://elapse.date!");
    // // 最基础warp
    // let hello = warp::path!("hello" / String).map(|name| format!("你好，{}！欢迎来到warp", name));
    // warp::serve(hello).run(([127, 0, 0, 1], 1358)).await;

    // env_logger::init();
    let log_level = crate::format_logger::get_log_level();
    // 自定义日志输出格式
    env_logger::Builder::new()
        .format(crate::format_logger::formatlog)
        .filter(None, log_level)
        .target(env_logger::Target::Stdout) //添加这行可以重定向日志
        .init();
    // log::info!("info日志");
    // log::warn!("警告日志");
    // log::error!("错误日志");
    // log::debug!("高度日志");
    log::info!("warp框架web站点：{}", get_env("BASE_URL"));

    let routes = filters::all_routes();

    //取得https证书等
    let cert_path = get_env("cert_path");
    let key_path = get_env("key_path");
    let ip_addr = get_env("ip_address");
    let socket_addr: std::net::SocketAddr = ip_addr.as_str().parse().unwrap();

    warp::serve(routes)
        .tls()
        .cert_path(cert_path)
        .key_path(key_path)
        .run(socket_addr)
        .await;
}
