
const mainEntry = document.getElementById("mainInput");
const runButton = document.getElementById("Run");
const backButton = document.getElementById("back");
const clearButton = document.getElementById("clear");
const saveButton = document.getElementById("save");
const mainOutput = document.getElementById("mainOutput");
const addFileButton = document.getElementById("addFileButton");
const tabDiv = document.getElementById("tabDiv");
let currentFile = "loading.."


window.message.getCurrentProjectData().then((res) => {
    mainEntry.value = res.txt
    currentFile = res.nameFile;
    res.tabs.forEach(tabName => {
        const tab = document.createElement("button")
        tab.innerText = tabName;
        tab.className = "tabButton";
        tab.style.display = "block"
        tab.addEventListener("click", async () => {
            mainEntry.value = await window.message.getTabText(tabName);
            currentFile = tabName;
            for(let i = 0; i<tabDiv.children.length; i++) {
                if (tabDiv.children[i].innerHTML != "Tabs:") {
                    tabDiv.children[i].style.backgroundColor = "#333";
                }                
            }
            tab.style.backgroundColor = "#550";
        })
        tabDiv.appendChild(tab);
    });
})  

document.addEventListener("keypress", (e) => {
    if (e.ctrlKey && e.key == 's') {
        window.message.save(currentFile, mainEntry.value);
    }
})

addFileButton.addEventListener("click", () => {
    window.message.loadFile("setupProject.html");
})

saveButton.addEventListener("click", () =>  {
    window.message.save(currentFile, mainEntry.value);
})

clearButton.addEventListener("click", () => {
    mainOutput.value = "";
})


backButton.addEventListener("click", () => {
    window.message.loadFile("index.html");
})

runButton.addEventListener("click", () => {
    const txt = mainEntry.value;
    window.message.save(currentFile, txt).then(async () => {
        const output = await window.message.run(currentFile);
        mainOutput.value += output+"\n";
    })
    
})

