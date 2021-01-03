extern crate clap;
extern crate serde;
extern crate serde_json;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader};

use actix_files as fs;
use actix_web::{App, HttpServer, HttpResponse, web, get, post};
use actix_web::middleware::Logger;
use env_logger::Env;
use log::{info};
use serde::{Deserialize, Serialize};
use r2d2_postgres::{postgres, r2d2::Pool, PostgresConnectionManager};

#[derive(Deserialize)]
struct DbConfig {
    host: String,
    port: String,
    db_name: String,
    user_name: String,
    password: String,
}

#[derive(Deserialize)]
struct AppConfig {
    public_dir_path: String,
    server_host: String,
    db: DbConfig,
}

#[derive(Serialize, Deserialize)]
struct Routine {
    routine_id: i32,
    user_id: i32,
    name: String,
    unit_period: String,
    target_quantity: f64,
    insert_datetime: String,
}

#[derive(Deserialize)]
struct RoutineAddForm {
    name: Option<String>,
    unit_period: Option<String>,
    target_quantity: Option<String>,
}

#[get("/routines")]
async fn routine_get_handler(pg_pool: web::Data<Pool<PostgresConnectionManager<postgres::NoTls>>>) -> HttpResponse {
    info!("start routine_get_handler");
    let sql = include_str!("../sql/select_routines.sql");

    let mut pg_client = pg_pool.get().unwrap();

    let mut routine_list: Vec<Routine> = Vec::new();
    for row in pg_client.query(sql, &[]).unwrap() {
        routine_list.push(Routine{
            routine_id: row.get("routine_id"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            unit_period: row.get("unit_period"),
            target_quantity: row.get("target_quantity"),
            insert_datetime: row.get("insert_datetime"),
        });
    }
    HttpResponse::Ok().json(routine_list)
}

#[post("/routines")]
async fn routine_add_handler(pg_pool: web::Data<Pool<PostgresConnectionManager<postgres::NoTls>>>, params: web::Json<RoutineAddForm>) -> HttpResponse {
    info!("start routine_add_handler");

    let name = params.name.clone().unwrap_or("".to_string());
    let unit_period = params.unit_period.clone().unwrap_or("0".to_string());
    let target_quantity = params.target_quantity.clone().unwrap_or("0.0".to_string());

    let sql = include_str!("../sql/insert_routine.sql");

    let mut pg_client = pg_pool.get().unwrap();

    let row = pg_client.query_one(sql, &[
        &name,
        &unit_period,
        &target_quantity.parse::<f64>().unwrap(),
    ]).unwrap();

    let routine = Routine{
        routine_id: row.get("routine_id"),
        user_id: row.get("user_id"),
        name: row.get("name"),
        unit_period: row.get("unit_period"),
        target_quantity: row.get("target_quantity"),
        insert_datetime: row.get("insert_datetime"),
    };

    HttpResponse::Ok().json(routine)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli_options = clap::App::new("routine-logger-web")
        .about("Routine Logger")
        .arg(clap::Arg::with_name("config")
            .help("Config File Path")
            .long("config")
            .short("c")
            .takes_value(true)
        )
        .get_matches();
    
    let p_config_file_path = cli_options.value_of("config").unwrap();
    let config_file_path = Path::new(p_config_file_path);
    if !config_file_path.exists() {
        panic!("Config file not found");
    }

    let config_file = match File::open(config_file_path) {
        Ok(n) => n,
        Err(err) => panic!("Config file opening error: {:?}", err),
    };

    let app_config: AppConfig = serde_json::from_reader(BufReader::new(config_file)).unwrap();

    let manager = PostgresConnectionManager::new(
        format!(
            "postgresql://{user}:{password}@{host}:{port}/{db_name}",
            user=app_config.db.user_name,
            password=app_config.db.password,
            host=app_config.db.host,
            port=app_config.db.port,
            db_name=app_config.db.db_name,
        ).parse().unwrap(),
        postgres::NoTls,
    );
    let pg_pool = Pool::new(manager).unwrap();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let public_dir_path = app_config.public_dir_path.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pg_pool.clone())
            .service(routine_get_handler)
            .service(routine_add_handler)
            .service(fs::Files::new("/", public_dir_path.clone()).show_files_listing())
    })
    .bind(app_config.server_host)?
    .run()
    .await
}