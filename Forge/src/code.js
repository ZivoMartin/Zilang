
const mainEntry = document.getElementById("mainInput");
const run_button = document.getElementById("Run");

run_button.addEventListener("click", () => {
    const txt = mainEntry.value;
    window.messsage.save("database/input.vu", txt);
    window.messsage.run("database/input.vu");
})

