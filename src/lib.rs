#![allow(non_snake_case)]
//! Snowflake Virtual Warehouse Router & Credit Calculator for SigmaOS

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WarehouseSize {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
}

impl WarehouseSize {
    pub fn credits_per_hour(&self) -> f64 {
        match self {
            WarehouseSize::XSmall => 1.0,
            WarehouseSize::Small => 2.0,
            WarehouseSize::Medium => 4.0,
            WarehouseSize::Large => 8.0,
            WarehouseSize::XLarge => 16.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SnowflakeWarehouse {
    pub name: String,
    pub size: WarehouseSize,
    pub active_queries: usize,
}

pub struct SnowflakeRouter {
    pub warehouses: Vec<SnowflakeWarehouse>,
}

impl Default for SnowflakeRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl SnowflakeRouter {
    pub fn new() -> Self {
        Self {
            warehouses: Vec::new(),
        }
    }

    pub fn add_warehouse(&mut self, wh: SnowflakeWarehouse) {
        self.warehouses.push(wh);
    }

    pub fn route_query(&mut self) -> Option<&mut SnowflakeWarehouse> {
        self.warehouses
            .iter_mut()
            .min_by_key(|wh| wh.active_queries)
    }

    pub fn estimate_query_cost_credits(size: WarehouseSize, execution_seconds: f64) -> f64 {
        let hours = execution_seconds / 3600.0;
        size.credits_per_hour() * hours
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snowflake_warehouse_routing_and_credit_estimate_aaa() {
        // Arrange
        let mut router = SnowflakeRouter::new();
        router.add_warehouse(SnowflakeWarehouse {
            name: "COMPUTE_WH_1".into(),
            size: WarehouseSize::Small,
            active_queries: 5,
        });
        router.add_warehouse(SnowflakeWarehouse {
            name: "COMPUTE_WH_2".into(),
            size: WarehouseSize::Small,
            active_queries: 1,
        });

        // Act
        let assigned = router.route_query().expect("No warehouse available");
        assigned.active_queries += 1;
        let credits = SnowflakeRouter::estimate_query_cost_credits(WarehouseSize::Small, 1800.0);

        // Assert
        assert_eq!(assigned.name, "COMPUTE_WH_2");
        assert_eq!(assigned.active_queries, 2);
        assert!((credits - 1.0).abs() < 1e-4); // 2 credits/hr * 0.5 hr = 1.0
    }
}
