use std::fs;
use std::path::PathBuf;
use axum::{routing::get, response::Html};
use tokio;
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

    let shared_state = Arc::new(vec_tasks); 

    let addr = String::from("0.0.0.0:3000");
    let app = axum::Router::new()
        .route("/task/list", get(list_tasks_handler))
        .with_state(shared_state.clone())
        .route("/",get(serve_index))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("servidor rodando em: {}",&addr);

    axum::serve(listener,app).await.unwrap();
}

async fn serve_index() -> Html<String> {
    let html = fs::read_to_string("static/index.html").unwrap_or_else(|err| {
        let error = Errors::FileNotFound(err.to_string());
        println!("error: {:?}", error);
        "<h1>Erro: index.html não encontrado.</h1>".to_string()
    });
    Html(html)
}