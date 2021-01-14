extern crate serde;
extern crate serde_json;

use actix_web::{HttpResponse, web, get, post, put, delete};
use log::{info};
use serde::{Deserialize, Serialize};
use r2d2_postgres::{postgres, r2d2::Pool, PostgresConnectionManager};

use crate::routes::{JSNumberType};

#[derive(Serialize, Deserialize)]
pub struct Routine {
    routine_id: i32,
    user_id: i32,
    name: String,
    unit_period: String,
    target_quantity: f64,
    unit: String,
    insert_datetime: String,
}

#[derive(Deserialize)]
pub struct RoutineAddForm {
    name: Option<String>,
    unit_period: Option<String>,
    target_quantity: JSNumberType,
    unit: Option<String>,
}

#[derive(Deserialize)]
pub struct RoutineUpdateForm {
    routine_id: i32,
    name: Option<String>,
    unit_period: Option<String>,
    target_quantity: JSNumberType,
    unit: Option<String>,
}

#[get("")]
pub async fn routine_get_handler(pg_pool: web::Data<Pool<PostgresConnectionManager<postgres::NoTls>>>) -> HttpResponse {
    info!("start routine_get_handler");
    let sql = include_str!("../sql/routines/select_routines.sql");

    let mut pg_client = pg_pool.get().unwrap();

    let mut routine_list: Vec<Routine> = Vec::new();
    for row in pg_client.query(sql, &[]).unwrap() {
        routine_list.push(Routine{
            routine_id: row.get("routine_id"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            unit_period: row.get("unit_period"),
            target_quantity: row.get("target_quantity"),
            unit: row.get("unit"),
            insert_datetime: row.get("insert_datetime"),
        });
    }
    HttpResponse::Ok().json(routine_list)
}

#[post("")]
pub async fn routine_add_handler(pg_pool: web::Data<Pool<PostgresConnectionManager<postgres::NoTls>>>, params: web::Json<RoutineAddForm>) -> HttpResponse {
    info!("start routine_add_handler");

    let name = params.name.clone().unwrap_or("".to_string());
    let unit_period = params.unit_period.clone().unwrap_or("0".to_string());
    let target_quantity = match params.target_quantity.clone() {
        JSNumberType::Float(v) => v,
        JSNumberType::Str(v) => v.unwrap_or("0.0".to_string()).parse::<f64>().unwrap(),
    };
    let unit = params.unit.clone().unwrap_or("".to_string());

    let sql = include_str!("../sql/routines/insert_routine.sql");

    let mut pg_client = pg_pool.get().unwrap();

    let row = pg_client.query_one(sql, &[
        &name,
        &unit_period,
        &target_quantity,
        &unit,
    ]).unwrap();

    let routine = Routine{
        routine_id: row.get("routine_id"),
        user_id: row.get("user_id"),
        name: row.get("name"),
        unit_period: row.get("unit_period"),
        target_quantity: row.get("target_quantity"),
        unit: row.get("unit"),
        insert_datetime: row.get("insert_datetime"),
    };

    HttpResponse::Ok().json(routine)
}

#[put("")]
pub async fn routine_update_handler(pg_pool: web::Data<Pool<PostgresConnectionManager<postgres::NoTls>>>, params: web::Json<RoutineUpdateForm>) -> HttpResponse {
    info!("start routine_update_handler");

    let routine_id = params.routine_id.clone();
    let name = params.name.clone().unwrap_or("".to_string());
    let unit_period = params.unit_period.clone().unwrap_or("0".to_string());
    let target_quantity = match params.target_quantity.clone() {
        JSNumberType::Float(v) => v,
        JSNumberType::Str(v) => v.unwrap_or("0.0".to_string()).parse::<f64>().unwrap(),
    };
    let unit = params.unit.clone().unwrap_or("".to_string());

    let sql = include_str!("../sql/routines/update_routine.sql");

    let mut pg_client = pg_pool.get().unwrap();

    let row = pg_client.query_one(sql, &[
        &routine_id,
        &name,
        &unit_period,
        &target_quantity,
        &unit,
    ]).unwrap();

    let routine = Routine{
        routine_id: row.get("routine_id"),
        user_id: row.get("user_id"),
        name: row.get("name"),
        unit_period: row.get("unit_period"),
        target_quantity: row.get("target_quantity"),
        unit: row.get("unit"),
        insert_datetime: row.get("insert_datetime"),
    };

    HttpResponse::Ok().json(routine)
}

#[delete("/{routine_id}")]
pub async fn routine_delete_handler(pg_pool: web::Data<Pool<PostgresConnectionManager<postgres::NoTls>>>, web::Path(routine_id): web::Path<i32>) -> HttpResponse {
    info!("start routine_delete_handler");

    let sql = include_str!("../sql/routines/delete_routine.sql");

    let mut pg_client = pg_pool.get().unwrap();

    let row = pg_client.query_one(sql, &[
        &routine_id,
    ]).unwrap();

    let routine = Routine{
        routine_id: row.get("routine_id"),
        user_id: row.get("user_id"),
        name: row.get("name"),
        unit_period: row.get("unit_period"),
        target_quantity: row.get("target_quantity"),
        unit: row.get("unit"),
        insert_datetime: row.get("insert_datetime"),
    };

    HttpResponse::Ok().json(routine)
}
