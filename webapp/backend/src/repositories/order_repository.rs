use crate::domains::order_service::OrderRepository;
use crate::errors::AppError;
use crate::models::order::Order;
use chrono::{DateTime, Utc};
use sqlx::mysql::MySqlPool;

#[derive(Debug)]
pub struct OrderRepositoryImpl {
    pool: MySqlPool,
}

impl OrderRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        OrderRepositoryImpl { pool }
    }
}

impl OrderRepository for OrderRepositoryImpl {
    async fn find_order_by_id(&self, id: i32) -> Result<Order, AppError> {
        let order = sqlx::query_as::<_, Order>(
            "SELECT 
                *
            FROM
                orders 
            WHERE
                id = ?",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(order)
    }

    async fn update_order_status(&self, order_id: i32, status: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE orders SET status = ? WHERE id = ?")
            .bind(status)
            .bind(order_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_paginated_orders(
        &self,
        page: i32,
        page_size: i32,
        sort_by: Option<String>,
        sort_order: Option<String>,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<Order>, AppError> {
        // 计算 offset
        let offset = page * page_size;

        // 构建排序子句，默认排序依据是 order_time，默认升序
        let order_clause = format!(
            "ORDER BY {} {}",
            match sort_by.as_deref() {
                Some("car_value") => "o.car_value",
                Some("status") => "o.status",
                _ => "o.order_time",
            },
            match sort_order.as_deref() {
                Some("DESC" | "desc") => "DESC",
                _ => "ASC",
            }
        );

        // 构建 WHERE 子句
        let mut where_conditions = vec![];
        if let Some(status) = &status {
            where_conditions.push("o.status = ?");
        }
        if let Some(_) = area {
            where_conditions.push("n.area_id = ?");
        }
        let where_clause = if !where_conditions.is_empty() {
            format!("WHERE {}", where_conditions.join(" AND "))
        } else {
            "".to_string()
        };

        // 完整的 SQL 查询语句
        let sql = format!(
            "SELECT 
                o.id, 
                o.client_id, 
                o.dispatcher_id, 
                o.tow_truck_id, 
                o.status, 
                o.node_id, 
                o.car_value, 
                o.order_time, 
                o.completed_time
            FROM
                orders o
            JOIN
                nodes n
            ON 
                o.node_id = n.id
            {} 
            {} 
            LIMIT ? 
            OFFSET ?",
            where_clause, order_clause
        );

        // 构建参数绑定
        let mut query = sqlx::query_as::<_, Order>(&sql);
        if let Some(status) = status {
            query = query.bind(status);
        }
        if let Some(area) = area {
            query = query.bind(area);
        }

        // 绑定分页参数
        query = query.bind(page_size).bind(offset);

        // 执行查询
        let orders = query.fetch_all(&self.pool).await?;

        Ok(orders)
    }

    async fn create_order(
        &self,
        client_id: i32,
        node_id: i32,
        car_value: f64,
    ) -> Result<(), AppError> {
        sqlx::query("INSERT INTO orders (client_id, node_id, status, car_value) VALUES (?, ?, 'pending', ?)")
            .bind(client_id)
            .bind(node_id)
            .bind(car_value)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_order_dispatched(
        &self,
        id: i32,
        dispatcher_id: i32,
        tow_truck_id: i32,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE orders SET dispatcher_id = ?, tow_truck_id = ?, status = 'dispatched' WHERE id = ?",
        )
        .bind(dispatcher_id)
        .bind(tow_truck_id)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn create_completed_order(
        &self,
        order_id: i32,
        tow_truck_id: i32,
        completed_time: DateTime<Utc>,
    ) -> Result<(), AppError> {
        sqlx::query("INSERT INTO completed_orders (order_id, tow_truck_id, completed_time) VALUES (?, ?, ?)")
            .bind(order_id)
            .bind(tow_truck_id)
            .bind(completed_time)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // async fn find_orderDto_by_id(&self, id: i32) -> Result<OrderDto, AppError> {
    //     let order = sqlx::query_as::<_, OrderDto>(
    //         "SELECT 
    //             o.id as id,
    //             o.client_id as client_id,
    //             u.username as client_username,


    //         FROM
    //             orders o
    //         JOIN  
    //             users u ON o.client_id = u.id
    //         JOIN 
    //             dispatchers d ON o.dispatcher_id = d.id  
    //         JOIN 
    //             t ON tow_truck_id    
    //         WHERE
    //             id = ?",
    //     )
    //     .bind(id)
    //     .fetch_one(&self.pool)
    //     .await?;

    //     Ok(order)
    // }
}
