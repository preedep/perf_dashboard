use crate::domain::perf::Error;
use crate::domain::perf::PerfRow;
use crate::domain::perf::PerfRowUpdate;
use crate::domain::perf::PerfSummary;
use crate::domain::perf::PerfTrends;
use log::error;
use sqlx::Arguments;
use sqlx::PgPool;
use sqlx::Row;
use sqlx::query as sqlx_query;

pub struct PerfRunsService {
    pub pool: PgPool,
}

impl PerfRunsService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list(
        &self,
        filter: Option<crate::domain::perf::PerfRunFilter>,
    ) -> Vec<crate::domain::perf::PerfRow> {
        // Example of using the injected connection pool
        let conn = self
            .pool
            .acquire()
            .await
            .expect("Failed to acquire connection");
        let mut query = String::from(
            r#"SELECT release_tag, row_no, test_scenario,
                 p95_latency_ms, avg_tps, peak_tps, failed_txn_pct,
                 failed_txn_count, total_txn_count, baseline_avg_tps, test_result_text, remark_text
                 FROM perf_runs WHERE 1=1"#,
        );
        let mut args = sqlx::postgres::PgArguments::default();
        if let Some(f) = &filter {
            if let Some(ref release_tag) = f.release_tag {
                query.push_str(" AND release_tag = $1");
                if let Err(e) = args.add(release_tag) {
                    error!("Error adding argument: {}", e);
                    // You may want to handle this error more robustly (e.g., return early)
                }
            }
            // Add more filters here as needed
        }
        let rows = sqlx_query(&query)
            .fetch_all(&self.pool)
            .await
            .unwrap_or_default();
        rows.into_iter()
            .map(|row| PerfRow {
                release_tag: row.try_get("release_tag").ok().unwrap_or_default(),
                row_no: row.try_get("row_no").ok(),
                test_scenario: row.try_get("test_scenario").ok(),
                p95_latency_ms: row.try_get("p95_latency_ms").ok(),
                avg_tps: row.try_get("avg_tps").ok(),
                peak_tps: row.try_get("peak_tps").ok(),
                failed_txn_pct: row.try_get("failed_txn_pct").ok(),
                failed_txn_count: row.try_get("failed_txn_count").ok(),
                total_txn_count: row.try_get("total_txn_count").ok(),
                baseline_avg_tps: row.try_get("baseline_avg_tps").ok(),
                test_result_text: row.try_get("test_result_text").ok(),
                remark_text: row.try_get("remark_text").ok(),
            })
            .collect()
    }

    pub async fn get_by_id(&self, row_no: i64) -> Option<crate::domain::perf::PerfRow> {
        let row = sqlx_query(
            r#"SELECT release_tag, row_no, test_scenario, p95_latency_ms,
             avg_tps, peak_tps, failed_txn_pct, failed_txn_count, total_txn_count,
              baseline_avg_tps, test_result_text, remark_text FROM perf_runs WHERE row_no = $1"#
        )
        .bind(row_no)
        .fetch_optional(&self.pool)
        .await
        .ok()??;

        Some(crate::domain::perf::PerfRow {
            release_tag: row.try_get("release_tag").ok().unwrap_or_default(),
            row_no: row.try_get("row_no").ok(),
            test_scenario: row.try_get("test_scenario").ok(),
            p95_latency_ms: row.try_get("p95_latency_ms").ok(),
            avg_tps: row.try_get("avg_tps").ok(),
            peak_tps: row.try_get("peak_tps").ok(),
            failed_txn_pct: row.try_get("failed_txn_pct").ok(),
            failed_txn_count: row.try_get("failed_txn_count").ok(),
            total_txn_count: row.try_get("total_txn_count").ok(),
            baseline_avg_tps: row.try_get("baseline_avg_tps").ok(),
            test_result_text: row.try_get("test_result_text").ok(),
            remark_text: row.try_get("remark_text").ok(),
        })
    }

    pub async fn summary(&self) -> PerfSummary {
        let row = sqlx_query(
            r#"SELECT COUNT(*) as total_runs,
                AVG(avg_tps) as avg_tps,
                AVG(p95_latency_ms) as avg_latency_ms
            FROM perf_runs"#
        )
        .fetch_one(&self.pool)
        .await
        .ok();

        PerfSummary {
            total_runs: row.as_ref().and_then(|r| r.try_get::<i64, _>("total_runs").ok()).unwrap_or(0) as usize,
            avg_tps: row.as_ref().and_then(|r| r.try_get::<f64, _>("avg_tps").ok()),
            avg_latency_ms: row.as_ref().and_then(|r| r.try_get::<f64, _>("avg_latency_ms").ok()),
        }
    }

    pub async fn trends(&self) -> PerfTrends {
        let rows = sqlx_query(
            r#"SELECT release_tag, AVG(avg_tps) as avg_tps
                FROM perf_runs
                GROUP BY release_tag
                ORDER BY release_tag"#
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        let trend_points = rows.into_iter().filter_map(|row| {
            let label = row.try_get::<String, _>("release_tag").ok()?;
            let value = row.try_get::<f64, _>("avg_tps").ok()?;
            Some((label, value))
        }).collect();

        PerfTrends { trend_points }
    }

    pub async fn import(&self, new_run: PerfRow) -> Result<(), Error> {
        // TODO: Implement logic to insert new_run into the database
        Ok(())
    }

    pub async fn update(&self, row_no: i64, update: PerfRowUpdate) -> Result<(), Error> {
        // TODO: Implement logic to update an existing run by row_no
        Ok(())
    }

    pub async fn delete(&self, row_no: i64) -> Result<(), Error> {
        // TODO: Implement logic to delete a run by row_no
        Ok(())
    }
}
