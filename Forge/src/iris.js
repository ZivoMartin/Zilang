const { exec } = require('child_process');

const irisPath = "../Iris/target/debug/iris";

const resultJsonPath = "/home/martin/Travail/Vulcain/Forge/result.json";

const fs = require('fs');

class Iris {

    constructor(){}

    newRequest(req) {
        exec(irisPath + " -j result.json -d \"" + req + "\" -p", (error, stdout, stderr) => {
            if (error) {
                console.error(`error: ${error.message}`);
                return;
            }
            if (stderr) {
                console.error(`stderr: ${stderr}`);
            }
            console.log(`stdout:\n${stdout}`);
        })
    }
}

module.exports = Iris;