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
            fs.appendFile("./database/result.json", "", (e) => {if (e) throw e});
            this.execFile("./database/init.sql");
            fs.rm('./database/userProjects', { recursive: true }, (err) => { 
                if (err) throw err;
                fs.mkdirSync("./database/userProjects");
                console.log("Iris: Initiation of project system is a success")
            })
        }
    }

    async execSync(cmd, type_op) {
        return new Promise((resolve, reject) => {
            exec(cmd, (error, stdout, stderr) => {
                outputManagment(error, stderr, stdout, type_op)
                resolve(stdout? stdout : stderr);
            });
        });
    }

    newRequest(req) {
        return this.execSync(irisPath + " -j "+resultJsonPath+" -d \"" + req + "\" -p", "Iris direct request execution");
    }

    execFile(file_path) {
        this.execSync(irisPath + " -f " + file_path, "Iris file execution")
    }

    extract_json() {
        const jsonContent = fs.readFileSync(resultJsonPath, {encoding: 'utf8'});
        if (jsonContent == "") {
            return []
        }
        const res = JSON.parse(jsonContent);
        return res;
    }

    async selectRequest(req) {
        await this.newRequest(req);
        return this.extract_json();
    }

    async getProjectList() {
        await this.newRequest("SELECT p_name FROM Projects");
        return this.extract_json().p_name;
    }
}

module.exports = Iris;