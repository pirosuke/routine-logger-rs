extern crate serde;
extern crate serde_json;

use actix_web::{HttpResponse, web, get, post, delete};
use log::{info};
use serde::{Deserialize, Serialize};
use r2d2_postgres::{postgres, r2d2::Pool, PostgresConnectionManager};

use crate::routes::{JSNumberType};

#[derive(Serialize, Deserialize)]
pub struct RoutineLog {
    log_id: i32,
    user_id: i32,
    routine_id: i32,
    routine_name: String,
    quantity: f64,
    unit: String,
    note: String,
    date_of_activity: String,
}

#[derive(Deserialize)]
pub struct RoutineLogAddForm {
    routine_id: i32,
    quantity: JSNumberType,
    date_of_activity: String,
}

#[get("")]
pub async fn log_get_handler(pg_pool: web::Data<Pool<PostgresConnectionManager<postgres::NoTls>>>) -> HttpResponse {
    info!("start log_get_handler");
    let sql = include_str!("../sql/routine_logs/select_logs.sql");

    let mut pg_client = pg_pool.get().unwrap();

    let mut log_list: Vec<RoutineLog> = Vec::new();
    for row in pg_client.query(sql, &[]).unwrap() {
        log_list.push(RoutineLog{
            log_id: row.get("log_id"),
            routine_id: row.get("routine_id"),
            routine_name: row.get("routine_name"),
            user_id: row.get("user_id"),
            quantity: row.get("quantity"),
            unit: row.get("unit"),
            note: row.get("note"),
            date_of_activity: row.get("date_of_activity"),
        });
    }
    HttpResponse::Ok().json(log_list)
}

#[post("")]
pub async fn log_add_handler(pg_pool: web::Data<Pool<PostgresConnectionManager<postgres::NoTls>>>, params: web::Json<RoutineLogAddForm>) -> HttpResponse {
    info!("start log_add_handler");

    let routine_id = params.routine_id.clone();
    let date_of_activity = params.date_of_activity.clone();
    let quantity = match params.quantity.clone() {
        JSNumberType::Float(v) => v,
        JSNumberType::Str(v) => v.unwrap_or("0.0".to_string()).parse::<f64>().unwrap(),
    };

    let sql = include_str!("../sql/routine_logs/insert_log.sql");

    let mut pg_client = pg_pool.get().unwrap();

    let row = pg_client.query_one(sql, &[
        &routine_id,
        &quantity,
        &date_of_activity,
    ]).unwrap();

    let routine = RoutineLog{
        log_id: row.get("log_id"),
        routine_id: row.get("routine_id"),
        routine_name: String::from(""),
        user_id: row.get("user_id"),
        quantity: row.get("quantity"),
        unit: String::from(""),
        note: row.get("note"),
        date_of_activity: row.get("date_of_activity"),
    };

    HttpResponse::Ok().json(routine)
}

#[delete("/{log_id}")]
pub async fn log_delete_handler(pg_pool: web::Data<Pool<PostgresConnectionManager<postgres::NoTls>>>, web::Path(log_id): web::Path<i32>) -> HttpResponse {
    info!("start log_delete_handler");

    let sql = include_str!("../sql/routine_logs/delete_log.sql");

    let mut pg_client = pg_pool.get().unwrap();

    let row = pg_client.query_one(sql, &[
        &log_id,
    ]).unwrap();

    let routine = RoutineLog{
        log_id: row.get("log_id"),
        routine_id: row.get("routine_id"),
        routine_name: String::from(""),
        user_id: row.get("user_id"),
        quantity: row.get("quantity"),
        unit: String::from(""),
        note: row.get("note"),
        date_of_activity: row.get("date_of_activity"),
    };

    HttpResponse::Ok().json(routine)
}
