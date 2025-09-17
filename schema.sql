DROP TABLE IF EXISTS perf_runs_normalized;
CREATE TABLE perf_runs_normalized (
                                      id SERIAL PRIMARY KEY,
                                      release_tag VARCHAR(64) NOT NULL,
                                      row_no BIGINT,
                                      test_scenario TEXT,
                                      p95_latency_ms DOUBLE PRECISION,
                                      avg_tps DOUBLE PRECISION,
                                      peak_tps DOUBLE PRECISION,
                                      failed_txn_pct DOUBLE PRECISION,
                                      failed_txn_count BIGINT,
                                      total_txn_count DOUBLE PRECISION,
                                      baseline_avg_tps DOUBLE PRECISION,
                                      test_result_text TEXT,
                                      remark_text TEXT,
                                      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);