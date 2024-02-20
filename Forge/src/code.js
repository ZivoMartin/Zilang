
const mainEntry = document.getElementById("mainInput");
const runButton = document.getElementById("Run");
const backButton = document.getElementById("back");

currentFile = window.message.getFirstFileCurrentProject();  

backButton.addEventListener("click", () => {
    window.message.loadFile("index.html");
})

runButton.addEventListener("click", () => {
    const txt = mainEntry.value;
    window.message.save("database/input.vu", txt);
    window.message.run("database/input.vu");
})

