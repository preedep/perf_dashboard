use serde::{Deserialize, Deserializer};

pub fn parse_row_no<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
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
pub struct PerfRow {
    pub release_tag: String,
    #[serde(deserialize_with = "crate::domain::perf::parse_row_no")]
    pub row_no: Option<i64>,
    pub test_scenario: Option<String>,
    pub p95_latency_ms: Option<f64>,
    pub avg_tps: Option<f64>,
    pub peak_tps: Option<f64>,
    pub failed_txn_pct: Option<f64>,
    pub failed_txn_count: Option<i64>,
    pub total_txn_count: Option<f64>,
    pub baseline_avg_tps: Option<f64>,
    pub test_result_text: Option<String>,
    pub remark_text: Option<String>,
}