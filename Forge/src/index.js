const openProjectButton = document.getElementById("openProjectButton");
const newProjectButton = document.getElementById("newProjectButton");

const openProjectDiv = document.getElementById("openProjectDiv");
const newProjectDiv = document.getElementById("newProjectDiv");

const newProjectButtonSubmit = document.getElementById("newProjectButtonSubmit");
const newProjectNameSubmit = document.getElementById("newProjectNameSubmit");

let nbProject = 0;

openProjectButton.addEventListener("click", async () => {
    newProjectDiv.style.display = "none"
    openProjectDiv.style.display = "block"
    const projects = await window.message.getProjects();
    let i = 0;
    projects.forEach(name => {
        if (i >= nbProject) {
            const txt = document.createElement("button")
            txt.innerText = name;
            txt.style.display = "block"
            txt.className = "openProjectButton"
            txt.addEventListener("click", () => {
                window.message.openProject(name);
            })
            nbProject += 1;
            openProjectDiv.appendChild(txt);
        }
        i ++;
    });
})

newProjectButton.addEventListener("click", () => {
    newProjectDiv.style.display = "block"
    openProjectDiv.style.display = "none"
})

newProjectButtonSubmit.addEventListener("click", () => {
    const name = newProjectNameSubmit.value;
    window.message.addProject(name);
})