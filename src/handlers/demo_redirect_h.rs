use warp::{Rejection, Reply};

/* 响应：/demo/redirect*/
pub async fn index() -> std::result::Result<impl Reply, Rejection> {
    log::debug!("[调试信息]访问了“/demo/redirect”");
    // log::warn!("[警告信息] warn");
    // log::info!("[提示信息] info");

    let html = "重定向示例".to_string();

    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回状态码
}

/* 响应：/demo/redirect/post_repeat */
pub async fn repeat(post: u32) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("[调试信息]访问了“/demo/redirect”");
    // log::warn!("[警告信息] warn");
    // log::info!("[提示信息] info");

    let html = "重定向示例".to_string();

    if post == 0 {

        log::warn!("无post数据，输出表单");
        //无post数据，输出表单
        return Err(warp::reject::not_found()); //错误的返回状态码
    }

    //处理完post数据，跳转到列表页
    let k = warp::redirect::see_other(warp::http::Uri::from_static("/demo/redirect/v"));

    Ok(k)
    // Ok(warp::reply::html(html)) //直接返回html
    // Err(warp::reject::not_found())   //错误的返回
}
