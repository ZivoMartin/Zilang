const { exec } = require('child_process');

const outputManagment = require('./main.js')

const irisPath = "../Iris/target/debug/iris";

const resultJsonPath = "/home/martin/Travail/Vulcain/Forge/database/result.json";

const fs = require('fs');

class Iris {

    newRequest(req) {
        exec(irisPath + " -j "+resultJsonPath+" -d \"" + req + "\" -p", (error, stdout, stderr) => 
        outputManagment(error, stdout, stderr, "Iris direct request execution"));
    }

    execFile(file_path) {
        exec(irisPath + " -f " + file_path, (error, stdout, stderr) => 
        outputManagment(error, stdout, stderr, "Iris file execution"))
    }
}

module.exports = Iris;