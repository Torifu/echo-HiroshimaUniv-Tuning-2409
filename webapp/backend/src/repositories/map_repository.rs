use sqlx::MySqlPool;

use crate::{
    domains::map_service::MapRepository,
    models::graph::{Edge, Node},
};

#[derive(Debug)]
pub struct MapRepositoryImpl {
    pool: MySqlPool,
}

impl MapRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        MapRepositoryImpl { pool }
    }
}

impl MapRepository for MapRepositoryImpl {
    async fn get_all_nodes(&self, area_id: Option<i32>) -> Result<Vec<Node>, sqlx::Error> {
        let where_clause = match area_id {
            Some(_) => "WHERE area_id = ?",
            None => "",
        };

        let sql = format!(
            "SELECT
                * 
            FROM
                nodes
            {}
            ORDER BY
                id",
            where_clause
        );

        let nodes = match area_id {
            Some(area_id) => {
                sqlx::query_as::<_, Node>(&sql)
                    .bind(area_id)
                    .fetch_all(&self.pool)
                    .await?
            }
            None => {
                sqlx::query_as::<_, Node>(&sql)
                    .fetch_all(&self.pool)
                    .await?
            }
        };

        Ok(nodes)
    }

    use sqlx::query_as;

    pub async fn get_all_edges(&self, area_id: Option<i32>) -> Result<Vec<Edge>, sqlx::Error> {
        let base_query = "
            SELECT
                e.node_a_id,
                e.node_b_id,
                e.weight
            FROM
                edges e";

        // 如果有 area_id，添加 JOIN 和 WHERE 子句，否则只使用基础查询
        let sql = if area_id.is_some() {
            format!("{} JOIN nodes n ON e.node_a_id = n.id WHERE n.area_id = ?", base_query)
        } else {
            base_query.to_string()
        };

        // 使用 query_as 直接执行查询
        let query = sqlx::query_as::<_, Edge>(&sql);

        // 绑定参数（如果有）并执行查询
        let edges = match area_id {
            Some(area_id) => query.bind(area_id).fetch_all(&self.pool).await?,
            None => query.fetch_all(&self.pool).await?,
        };

        Ok(edges)
    }

    async fn get_area_id_by_node_id(&self, node_id: i32) -> Result<i32, sqlx::Error> {
        let area_id = sqlx::query_scalar("SELECT area_id FROM nodes WHERE id = ?")
            .bind(node_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(area_id)
    }

    async fn update_edge(
        &self,
        node_a_id: i32,
        node_b_id: i32,
        weight: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE edges SET weight = ? WHERE (node_a_id = ? AND node_b_id = ?) OR (node_a_id = ? AND node_b_id = ?)")
            .bind(weight)
            .bind(node_a_id)
            .bind(node_b_id)
            .bind(node_b_id)
            .bind(node_a_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
