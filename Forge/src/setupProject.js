const addFileButton = document.getElementById("addFileButton");
const addFileInput = document.getElementById("addFileInput");

addFileButton.addEventListener("click", () => {
    window.message.addFile(addFileInput.value);
    window.message.backToProject();
})