document.querySelector(".all_tasks").addEventListener("click", async () => {
    const resposta = await fetch("/task/list");
    // const texto = await resposta.json();
    // document.getElementById("resultado").textContent = texto;
});