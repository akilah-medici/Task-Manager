use std::fs::File;
use serde_json::to_writer;
use serde_json::from_reader;
use serde::{Serialize, Deserialize};
use std::io;
use std::io::Write;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Task{
    name: String,
    description: String,
    state: bool
}
impl Task{
    fn check_uncheck_task(&mut self,st: bool){
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

fn main() {
    let mut tasks:Vec<Task> = load_from_file();
}

fn create_task() -> Task{
    let nm;
    let des;

    println!("Nome da tarefa:");
    nm = read_input();

    println!("Descrição da tarefa:");
    des = read_input();
    
    Task { name: (nm), description: (des), state: (false) }
}

fn remove_task(vec: &mut Vec<Task>, name: String){
    let nm;
    println!("Nome da tarefa a ser removida:");
    nm = read_input();

    for i in 0..vec.len(){
        if vec[i].name.to_uppercase() == name.to_uppercase(){
            vec.remove(i);
        }
    }
}

fn save_to_file(vec: Vec<Task>){
    let file = File::create("tasks.json").expect("Erro ao criar arquivo");
    to_writer(file, &vec).expect("Erro ao salvar dados em JSON");
}

fn load_from_file() -> Vec<Task> {
    let file = File::open("tasks.json").expect("Erro ao abrir arquivo");
    from_reader(file).expect("Erro ao carregar dados do JSON")
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    input.trim().to_string()
}