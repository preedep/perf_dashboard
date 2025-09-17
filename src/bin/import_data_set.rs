use log::info;
use dotenv;
use pretty_env_logger;
use csv::ReaderBuilder;
use sqlx::PgPool;
use serde::{Deserialize, Deserializer};
use anyhow;

fn parse_row_no<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        let s = s.trim();
        if s.is_empty() {
            Ok(None)
        } else if let Ok(i) = s.parse::<i64>() {
            Ok(Some(i))
        } else if let Ok(f) = s.parse::<f64>() {
            Ok(Some(f as i64))
        } else {
            Err(serde::de::Error::custom("Invalid row_no"))
        }
    } else {
        Ok(None)
    }
}

#[derive(Debug, Deserialize)]
struct PerfRow {
    release_tag: String,
    #[serde(deserialize_with = "parse_row_no")]
    row_no: Option<i64>,
    test_scenario: Option<String>,
    p95_latency_ms: Option<f64>,
    avg_tps: Option<f64>,
    peak_tps: Option<f64>,
    failed_txn_pct: Option<f64>,
    failed_txn_count: Option<i64>,
    total_txn_count: Option<f64>,
    baseline_avg_tps: Option<f64>,
    test_result_text: Option<String>,
    remark_text: Option<String>,
}

pub async fn import_csv(pool: &PgPool, path: &str) -> anyhow::Result<()> {
    let mut rdr = ReaderBuilder::new().from_path(path)?;
    for (i, result) in rdr.deserialize::<PerfRow>().enumerate() {
        match result {
            Ok(row) => {
                sqlx::query!(
                    r#"INSERT INTO perf_runs_normalized
                    (release_tag,row_no,test_scenario,p95_latency_ms,avg_tps,peak_tps,
                     failed_txn_pct,failed_txn_count,total_txn_count,baseline_avg_tps,
                     test_result_text,remark_text)
                    VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)"#,
                    row.release_tag,
                    row.row_no,
                    row.test_scenario,
                    row.p95_latency_ms,
                    row.avg_tps,
                    row.peak_tps,
                    row.failed_txn_pct,
                    row.failed_txn_count,
                    row.total_txn_count,
                    row.baseline_avg_tps,
                    row.test_result_text,
                    row.remark_text
                )
                .execute(pool)
                .await?;
            }
            Err(e) => {
                eprintln!("Error at record {}: {:?}", i + 1, e);
                info!("Error at record {}: {:?}", i + 1, e);
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();
    info!("Starting import...");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to DB");
    let csv_path = std::env::args().nth(1).expect("Please provide CSV path as argument");
    info!("Importing {}...", csv_path);
    import_csv(&pool, &csv_path).await.expect("Import failed");
    info!("Import finished.");
}