use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
//use std::ops::{Deref, DerefMut};

use super::task::Task;
use crate::errors::Errors;



#[derive(Serialize, Deserialize, Clone)]
#[derive(Debug)]
pub struct ListTasks{
    pub data: Vec<Task>,
    pub path: PathBuf
}
#[allow(dead_code)]
impl ListTasks{
    pub async fn create_task(&mut self, nm: String, ds: String, st: bool){
        self.data.push(Task::new(nm,ds,st).await);
    }
    pub async fn list_tasks(&self){
        println!("{:?}",self.data);
    }
    pub fn load_from_file(&mut self) -> Result<(), Errors>{
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
    pub async fn save_to_file(&mut self) -> Result<(), Errors>{
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

// impl Deref for ListTasks {
//     type Target = Vec<Task>;

//     fn deref(&self) -> &Self::Target {
//         &self.data
//     }
// }

// impl DerefMut for ListTasks {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.data
//     }
// }