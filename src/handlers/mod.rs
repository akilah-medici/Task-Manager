use std::sync::Arc; 
use axum::{extract::State, Json, response::Html, response::IntoResponse};
use std::fs;
use tokio::sync::Mutex;

use crate::data::{Task,ListTasks};
use crate::Errors;


pub async fn list_tasks_handler(State(state): State<Arc<Mutex<ListTasks>>>) -> Json<Vec<Task>> {
    Json(state.lock().await.data.clone()) 
}

pub async fn serve_index() -> Html<String> {
    let html = fs::read_to_string("static/index.html").unwrap_or_else(|err| {
        let error = Errors::FileNotFound(err.to_string());
        println!("error: {:?}", error);
        "<h1>Erro: index.html não encontrado.</h1>".to_string()
    });
    Html(html)
}
pub async fn serve_create_task() -> Html<String> {
    let html = fs::read_to_string("static/create_task.html").unwrap_or_else(|err| {
        let error = Errors::FileNotFound(err.to_string());
        println!("error: {:?}", error);
        "<h1>Erro: create_task.html não encontrado.</h1>".to_string()
    });
    Html(html)
}
#[axum::debug_handler]
pub async fn get_response(State(state): State<Arc<Mutex<ListTasks>>>, Json(tsk): Json<Task>) -> impl IntoResponse {
    let mut lt = state.lock().await;
    lt.data.push(tsk.clone());
    lt.save_to_file().await;
    format!("Recebido: {}", tsk.name)
}

