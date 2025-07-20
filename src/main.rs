use std::io::{BufReader, BufWriter};
use std::{fs,fs::File};
use std::path::{Path,PathBuf};
use serde_json::to_writer;
use serde_json::from_reader;
use serde::{Serialize, Deserialize};
use axum::{routing::get, response::Html, extract::State};
use tokio;
use tower_http::services::ServeDir;
use std::sync::Arc; 

mod errors;
use errors::Errors;


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Task{
    name: String,
    description: String,
    state: bool
}
impl Task{
    async fn new(nm: String, desc: String, st: bool) -> Task{
        Task{
            name : nm,
            description : desc,
            state : st,
        }
    }
    async fn check_uncheck_task(&mut self,st: bool){
        self.state = st;
    }
    fn to_string(&self) -> String{
        let state;
        if self.state{
            state = "Completo".to_string();
        }else{
            state = "Incompleto".to_string();
        }
        format!("---\nNome da tarefa: {}\nDescrição da tarefa: {}\nEstado: {}\n---",self.name,self.description,state)
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct ListTasks{
    data: Vec<Task>,
    path: PathBuf
}
impl ListTasks{
    async fn create_task(&mut self, nm: String, ds: String, st: bool){
        self.data.push(Task::new(nm,ds,st).await);
    }
    async fn list_tasks(&self){
        println!("{:?}",self.data);
    }
    fn load_from_file(&mut self) -> Result<(), Errors>{
        let file = File::open(&self.path).map_err(|err| {
            let error = Errors::FileNotFound(err.to_string());
            println!("Error: {:?}", error);
            error
        })?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader).map_err(|err| {
            let error = Errors::FileError(err.to_string());
            println!("Error: {:?}", error);
            error
        })?;
        Ok(self.data = data)
    }
    async fn save_to_file(&mut self) -> Result<(), Errors>{
        let file = match File::open(&self.path){
            Ok(file) => file,
            Err(err) => {
                let error = Errors::FileNotFound(err.to_string());
                println!("Error: {:?}, creating correspondent file.", error);
                File::create(&self.path).map_err(|err| {
                    let error = Errors::FileNotFound(err.to_string());
                    println!("Error: {:?}, on creating file.", error);
                    error
                })?
            }
        };
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.data).map_err(|err|{
            let error = Errors::FileError(err.to_string());
            println!("Error: {:?}", error);
            error
        })?;
        Ok(())
    }
}

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
    vec_tasks.list_tasks().await;

    let shared_state = Arc::new(vec_tasks); 

    let addr = String::from("127.0.0.1:3000");
    let app = axum::Router::new()
        // .route("/task/list", get(|| async {
        //    println!("Criou task")
        // }))
        .route("/task/list", get(list_tasks_handler))
        .with_state(shared_state.clone())
        //.route("task/list", get(|| async {println!("peido")}))
        .route("/",get(serve_index))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("servidor rodando em: {}",&addr);

    axum::serve(listener,app).await.unwrap();
}

async fn serve_index() -> Html<String> {
    let html = fs::read_to_string("static/index.html").unwrap_or_else(|_| {
        "<h1>Erro: index.html não encontrado.</h1>".into()
    });
    Html(html)
}
async fn list_tasks_handler(State(state): State<Arc<ListTasks>>) {
    state.list_tasks().await;
}