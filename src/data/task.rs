use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Clone)]
#[derive(Debug)]
pub struct Task{
    pub name: String,
    pub description: String,
    pub state: bool
}
#[allow(dead_code)]
impl Task{
    pub async fn new(nm: String, desc: String, st: bool) -> Task{
        Task{
            name : nm,
            description : desc,
            state : st,
        }
    }
    pub async fn check_uncheck_task(&mut self,st: bool){
        self.state = st;
    }
    pub fn to_string(&self) -> String{
        let state;
        if self.state{
            state = "Completo".to_string();
        }else{
            state = "Incompleto".to_string();
        }
        format!("---\nNome da tarefa: {}\nDescrição da tarefa: {}\nEstado: {}\n---",self.name,self.description,state)
    }
}