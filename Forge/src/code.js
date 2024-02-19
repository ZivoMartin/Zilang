
const mainEntry = document.getElementById("mainInput");
const run_button = document.getElementById("Run");

run_button.addEventListener("click", () => {
    const txt = mainEntry.value;
    window.versions.save("database/input.vu", txt);
    window.versions.run("database/input.vu");
})

