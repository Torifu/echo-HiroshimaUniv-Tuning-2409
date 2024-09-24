use crate::domains::tow_truck_service::TowTruckService;
use crate::errors::AppError;
use crate::repositories::order_repository::OrderRepositoryImpl;
use crate::repositories::tow_truck_repository::TowTruckRepositoryImpl;
use crate::{
    domains::dto::tow_truck::UpdateLocationRequestDto,
    repositories::map_repository::MapRepositoryImpl,
};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Debug)]
pub struct PaginatedTowTruckQuery {
    page: Option<i32>,
    page_size: Option<i32>,
    status: Option<String>,
    area: Option<i32>,
}

pub async fn get_paginated_tow_trucks_handler(
    service: web::Data<
        TowTruckService<TowTruckRepositoryImpl, OrderRepositoryImpl, MapRepositoryImpl>,
    >,
    query: web::Query<PaginatedTowTruckQuery>,
) -> Result<HttpResponse, AppError> {

    // 开始计时
    let start = Instant::now();

    let tow_trucks = service
        .get_all_tow_trucks(
            query.page.unwrap_or(0),
            query.page_size.unwrap_or(-1),
            query.status.clone(),
            query.area,
        )
        .await?;

    // 计算执行时间
    let duration = start.elapsed();
    println!("get_paginated_tow_trucks_handler 时间间隔: {:?}", duration);    

    Ok(HttpResponse::Ok().json(tow_trucks))
}    

pub async fn get_tow_truck_handler(
    service: web::Data<
        TowTruckService<TowTruckRepositoryImpl, OrderRepositoryImpl, MapRepositoryImpl>,
    >,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {

    // 开始计时
    let start = Instant::now();

    let id = path.into_inner();
    let result = match service.get_tow_truck_by_id(id).await {
        Ok(Some(tow_truck)) => Ok(HttpResponse::Ok().json(tow_truck)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(err) => Err(err),
    };

    // 计算执行时间
    let duration = start.elapsed();
    println!("get_tow_truck_handler 时间间隔: {:?}", duration);

    // 返回处理结果
    result
}

pub async fn update_location_handler(
    service: web::Data<
        TowTruckService<TowTruckRepositoryImpl, OrderRepositoryImpl, MapRepositoryImpl>,
    >,
    req: web::Json<UpdateLocationRequestDto>,
) -> Result<HttpResponse, AppError> {

    // 开始计时
    let start = Instant::now();

    service
        .update_location(req.tow_truck_id, req.node_id)
        .await?;

    // 计算执行时间
    let duration = start.elapsed();
    println!("update_location_handler 时间间隔: {:?}", duration);

    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize, Debug)]
pub struct TowTruckQuery {
    order_id: i32,
}

pub async fn get_nearest_available_tow_trucks_handler(
    service: web::Data<
        TowTruckService<TowTruckRepositoryImpl, OrderRepositoryImpl, MapRepositoryImpl>,
    >,
    query: web::Query<TowTruckQuery>,
) -> Result<HttpResponse, AppError> {

    // 开始计时
    let start = Instant::now();

    let result = match service
        .get_nearest_available_tow_trucks(query.order_id)
        .await
    {
        Ok(Some(tow_truck)) => Ok(HttpResponse::Ok().json(tow_truck)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(err) => Err(err),
    };

    // 计算执行时间
        let duration = start.elapsed();
        println!("get_nearest_available_tow_trucks_handler 时间间隔: {:?}", duration);

    // 返回处理结果
        result        
}
