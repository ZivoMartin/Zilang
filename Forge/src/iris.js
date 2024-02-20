const { exec } = require('child_process');

const outputManagment = (error, stdout, stderr, type_op) => {
    if (error) {
      console.error(type_op + ` error: ${error.message}`);
      return false;
    }
    if (stderr) console.error(type_op + ` stderr: ${stderr}`);
    if (stdout) console.log(type_op + ` stdout:\n${stdout}`);      
    return true;  
  };
  

const irisPath = "../Iris/target/debug/iris";

const resultJsonPath = "./database/result.json";

const fs = require('fs');


class Iris {

    constructor() {
        if (!fs.existsSync("./database/result.json")) {
            this.execFile("./database/init.sql");
            fs.appendFile("./database/result.json", "", (e) => {if (e) console.error("Failed to create the result.json file: " + e)});
            console.log("Iris: Initiation of project system is a success")
        }
    }

    newRequest(req) {
        exec(irisPath + " -j "+resultJsonPath+" -d \"" + req + "\" -p", (error, stdout, stderr) => 
        outputManagment(error, stdout, stderr, "Iris direct request execution"));
    }

    async execFile(file_path) {
        exec(irisPath + " -f " + file_path, (error, stdout, stderr) => 
        outputManagment(error, stdout, stderr, "Iris file execution"))
    }

    extract_json() {
        return JSON.parse(fs.readFileSync(resultJsonPath, {encoding: 'utf8'}));
    }
}

module.exports = Iris;