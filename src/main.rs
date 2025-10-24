use std::path::PathBuf;
use axum::{routing::{get, post}};
use tokio::{sync::Mutex};
use tower_http::services::ServeDir;
use std::sync::Arc; 

mod errors;
mod handlers;
mod data;

use crate::errors::Errors;
use crate::data::ListTasks;
use crate::handlers::*;

#[tokio::main]
async fn main(){

    let mut vec_tasks = ListTasks{
        data: Vec::new(),
        path: PathBuf::from("tasks.json")
    };
    match vec_tasks.load_from_file(){
        Ok(_) => println!("File load successefuly!"),
        Err(err) => {
            println!("Error on load file: {:?}",err);
        }
    }

    let shared_state = Arc::new(Mutex::new(vec_tasks)); 

    let addr = String::from("127.0.0.1:3000");
    let app = axum::Router::new()
        .route("/task/list", get(list_tasks_handler))
        .route("/task/create", get(serve_create_task))
        .route("/task/create/accept", post(get_response))
        .route("/task/delete", get(serve_delete_task))
        .with_state(shared_state.clone())
        .route("/",get(serve_index))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("servidor rodando em: {}",&addr);

    axum::serve(listener,app).await.unwrap();
}

