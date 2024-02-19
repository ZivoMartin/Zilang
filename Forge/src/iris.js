const { exec } = require('child_process');

const outputManagment = (error, stdout, stderr, type_op) => {
    if (error) {
      console.error(type_op + ` error: ${error.message}`);
      return false;
    }
    if (stderr) {
      console.error(type_op + ` stderr: ${stderr}`);
    }
    console.log(type_op + ` stdout:\n${stdout}`);      
    return true;  
  };
  
const vulcainPath = "/home/martin/Travail/Vulcain"

const irisPath = vulcainPath+"/Iris/target/debug/iris";

const resultJsonPath = vulcainPath+"/Forge/database/result.json";

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