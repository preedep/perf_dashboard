use actix_web::{get, web, HttpResponse, Responder};
use crate::domain::perf::{PerfSummary, PerfTrends, PerfRunFilter};
use crate::usecase::perf_runs_service::PerfRunsService;
use sqlx::PgPool;

#[get("/api/perf-runs")]
pub async fn list_perf_runs(
    pool: web::Data<PgPool>,
    query: web::Query<PerfRunFilter>
) -> impl Responder {
    let service = PerfRunsService { pool: pool.get_ref().clone() };
    let runs = service.list(Some(query.into_inner())).await;
    HttpResponse::Ok().json(runs)
}

#[get("/api/perf-runs/{row_no}")]
pub async fn get_perf_run_by_id(pool: web::Data<PgPool>, row_no: web::Path<i64>) -> impl Responder {
    let service = PerfRunsService { pool: pool.get_ref().clone() };
    match service.get_by_id(row_no.into_inner()).await {
        Some(run) => HttpResponse::Ok().json(run),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/api/perf-runs/summary")]
pub async fn perf_runs_summary(pool: web::Data<PgPool>) -> impl Responder {
    let service = PerfRunsService { pool: pool.get_ref().clone() };
    let summary = service.summary().await;
    HttpResponse::Ok().json(summary)
}

#[get("/api/perf-runs/trends")]
pub async fn perf_runs_trends(pool: web::Data<PgPool>) -> impl Responder {
    let service = PerfRunsService { pool: pool.get_ref().clone() };
    let trends = service.trends().await;
    HttpResponse::Ok().json(trends)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_perf_runs)
        .service(get_perf_run_by_id)
        .service(perf_runs_summary)
        .service(perf_runs_trends);
}
