
const mainEntry = document.getElementById("mainInput");
const run_button = document.getElementById("Run");

run_button.addEventListener("click", () => {
    const txt = mainEntry.value;
    window.versions.save("vulcain_files/input.vu", txt);
    window.versions.run("vulcain_files/input.vu");
})

