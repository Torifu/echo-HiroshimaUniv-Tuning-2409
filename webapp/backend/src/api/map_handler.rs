use crate::{
    domains::{dto::map::UpdateEdgeRequestDto, map_service::MapService},
    errors::AppError,
    repositories::map_repository::MapRepositoryImpl,
};
use actix_web::{web, HttpResponse};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn update_edge_handler(
    service: web::Data<MapService<MapRepositoryImpl>>,
    req: web::Json<UpdateEdgeRequestDto>,
) -> Result<HttpResponse, AppError> {
    
    // 开始计时
    let start = Instant::now();

    let result = match service
        .update_edge(req.node_a_id, req.node_b_id, req.weight)
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(err),
    }

    // 计算执行时间
    let duration = start.elapsed();
    println!("update_edge_handler 时间间隔: {:?}", duration);

    //返回
     result
}
