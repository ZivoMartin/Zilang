const openProjectButton = document.getElementById("openProjectButton");
const newProjectButton = document.getElementById("newProjectButton");

const openProjectDiv = document.getElementById("openProjectDiv");
const newProjectDiv = document.getElementById("newProjectDiv");

const newProjectButtonSubmit = document.getElementById("newProjectButtonSubmit");
const newProjectNameSubmit = document.getElementById("newProjectNameSubmit");

openProjectButton.addEventListener("click", () => {
    newProjectDiv.style.display = "none"
    openProjectDiv.style.display = "block"
})

newProjectButton.addEventListener("click", () => {
    newProjectDiv.style.display = "block"
    openProjectDiv.style.display = "none"
})

newProjectButtonSubmit.addEventListener("click", () => {
    const name = newProjectNameSubmit.value;
    window.versions.addProject(name);
})