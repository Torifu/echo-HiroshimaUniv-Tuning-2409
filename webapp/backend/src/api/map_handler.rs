use crate::{
    domains::{dto::map::UpdateEdgeRequestDto, map_service::MapService},
    errors::AppError,
    repositories::map_repository::MapRepositoryImpl,
};
use actix_web::{web, HttpResponse};

pub async fn update_edge_handler(
    service: web::Data<MapService<MapRepositoryImpl>>,
    req: web::Json<UpdateEdgeRequestDto>,
) -> Result<HttpResponse, AppError> {
    
    
    use std::time::{SystemTime, UNIX_EPOCH};

    match service
        .update_edge(req.node_a_id, req.node_b_id, req.weight)
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(err),
    }

    println!("update_edge_handler 时间间隔: {:?}", SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis());
}
