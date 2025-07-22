use std::sync::Arc; 
use axum::{extract::State, Json};

use crate::data::{Task,ListTasks};

pub async fn list_tasks_handler(State(state): State<Arc<ListTasks>>) -> Json<Vec<Task>> {
    Json(state.data.clone()) 
}

