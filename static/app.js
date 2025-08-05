 
// let task_list_enabled = 0;

// // const resposta = await fetch("/create_task.html");
// // const html = await resposta.text();
// // document.querySelector("#container").innerHTML = html;




// document.querySelector(".all_tasks").addEventListener("click", async () => {
//     if(task_list_enabled === 0){
//         task_list_enabled = 1;
//         document.querySelector(".task_list").style.display="block";
//         carregarTarefas();
//     } else if(task_list_enabled === 1){
//         task_list_enabled = 0;
//         document.querySelector(".task_list").style.display="none";
//     }
// })

// document.querySelector(".create_task").addEventListener("click", async () =>{
//     const resposta = await fetch("/static/create_task.html");
//     const html = await resposta.text();
//     document.querySelector("#conteiner").innerHTML = html;
//     console.log("peidou0")
//     createTask();
// })

// async function carregarTarefas() {
//     try {
//         const resposta = await fetch("/task/list");
//         if (!resposta.ok) throw new Error("Erro ao buscar tarefas");

//         const tarefas = await resposta.json();

//         const lista = document.querySelector(".task_list");
//         lista.innerHTML = "";

//         tarefas.forEach(tarefa => {
//             const item = document.createElement("p");
//             item.textContent = `${tarefa.name} \n - ${tarefa.state ? "Completa" : "Incompleta"} - \n ${tarefa.description}`;
//             lista.appendChild(item);
//         });
//     } catch (erro) {
//         console.error("Erro:", erro);
//     }
// }
// async function createTask(){
//     const response = await fetch("/task/create");
//     const html = await response.text();
//     document.querySelector("#conteiner").innerHTML = html;
//     // window.location.href = "/task/create";
//     document.querySelector(".back").addEventListener("click", async () =>{
//         console.log("peidou1");
//         const response = await fetch("/");
//         const html = await response.text();
//         document.querySelector("#conteiner").innerHTML = html;
//     }) 
// }



// organizar funções para reload dos listeners
// organizar listeners

async function initPage() {
    let task_list_enabled = 0;

    const allTasksBtn = document.querySelector(".all_tasks");
    if (allTasksBtn) {
        loadTasks();
        allTasksBtn.addEventListener("click", async () => {
            if (task_list_enabled === 0) {
                task_list_enabled = 1;
                document.querySelector(".task_list").style.display = "block";
            } else {
                task_list_enabled = 0;
                document.querySelector(".task_list").style.display = "none";
            }
        });
    }

    const createTaskBtn = document.querySelector(".create_task");
    if (createTaskBtn) {
        createTaskBtn.addEventListener("click", async () => {
            createTaskPage();
        });
    }
}

async function createTaskPage() {
    const response = await fetch("/task/create");
    const html = await response.text();
    document.querySelector("#conteiner").innerHTML = html;

    const backBtn = document.querySelector(".back");
    if (backBtn) {
        backBtn.addEventListener("click", async () => {
            const response = await fetch("/");
            const html = await response.text();
            document.querySelector("#conteiner").innerHTML = html;
            initPage();
        });
    }
    const printtask = document.querySelector(".accept");
    if (printtask) {
        printtask.addEventListener("click", async () => {
            getResponse();
            const response = await fetch("/");
            const html = await response.text();
            document.querySelector("#conteiner").innerHTML = html;
            initPage();
        });
    }
}

async function loadTasks() {
    try {
        const resposta = await fetch("/task/list");
        if (!resposta.ok) throw new Error("Erro ao buscar tarefas");

        const tarefas = await resposta.json();

        const lista = document.querySelector(".task_list");
        lista.innerHTML = "";

        tarefas.forEach(tarefa => {
            const item = document.createElement("p");
            item.textContent = `${tarefa.name} : ${tarefa.state ? "Completa" : "Incompleta"} - ${tarefa.description}`;
            lista.appendChild(item);
        });
    } catch (erro) {
        console.error("Erro:", erro);
    }
}

async function getResponse() {
    //criar verificação para tarefas vazias
    const name = document.querySelector("#text_name").value;
    const desc = document.querySelector("#text_description").value;

    const answer = await fetch("/task/create/accept", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ "name": name, "description" : desc, "state" : false }),
    });
    const retorno = await answer.text();
    console.log("Servidor respondeu:", retorno);
}
initPage();