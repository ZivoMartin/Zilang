
const mainEntry = document.getElementById("mainInput");
const runButton = document.getElementById("Run");
const backButton = document.getElementById("back");
const clearButton = document.getElementById("clear");
const saveButton = document.getElementById("save");
const mainOutput = document.getElementById("mainOutput");

let currentFile = "loading.."



document.addEventListener("keypress", (e) => {
    if (e.ctrlKey && e.key == 's') {
        window.message.save(currentFile, mainEntry.value);
    }

})

saveButton.addEventListener("click", () =>  {
    window.message.save(currentFile, mainEntry.value);
})

clearButton.addEventListener("click", () => {
    mainOutput.value = "";
})

window.message.getFirstFileCurrentProject().then((res) => {
    mainEntry.value = res.txt
    currentFile = res.nameFile;
})  

backButton.addEventListener("click", () => {
    window.message.loadFile("index.html");
})

runButton.addEventListener("click", () => {
    const txt = mainEntry.value;
    window.message.save(currentFile, txt).then(async () => {
        const output = await window.message.run(currentFile);
        mainOutput.value += output;
    })
    
})

