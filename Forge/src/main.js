const Iris = require("./iris.js");

const { app, BrowserWindow, ipcMain } = require('electron');

const repo_path = require('node:path')

const fs = require('fs');

const { exec } = require('child_process');

const index_path = "view/index.html";
const idepath = "view/code.html";
const viewPath = "./view/"
const compile_command = (path) => "cd ../compiler && cargo run ../Forge/" + path + " -o exe";

const exec_command = "../compiler/exe";

const iris = new Iris();

const projectList = iris.extract_json().p_name;

const outputManagment = (error, stdout, stderr, type_op) => {
  if (error) {
    console.error(type_op + ` error: ${error.message}`);
    return false;
  }
  if (stderr) console.error(type_op + ` stderr: ${stderr}`);
  if (stdout) console.log(type_op + ` stdout:\n${stdout}`);      
  return true;  
};



app.whenReady().then(() => {
  
  const win = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      preload: repo_path.join(__dirname, 'preload.js')
    }
  })
  
  win.loadFile(index_path)

  ipcMain.handle('save', (e, path, new_txt) => fs.writeFile(path, new_txt, (e)=>{if (e) throw e}))
  ipcMain.handle('run', (e, path) => {
    if (path.endsWith(".vu")) {
      exec(compile_command(path), (error, stdout, stderr) => {
        if (outputManagment(error, stdout, stderr, "Compilation")){
          exec(exec_command, (error, stdout, stderr) =>  outputManagment(error, stdout, stderr, "Execution"));
        }
      });
    }else{
      console.error("Forge error: You can only run .vu files..");
    }
  });
  ipcMain.handle("get_content", (e, path) => fs.readFileSync(path, {encoding: 'utf8'}))
  ipcMain.handle("openide", () => win.loadFile(idepath))
  ipcMain.handle("addProject", (e, name) => {
      iris.newRequest("INSERT INTO Projects (p_name) VALUES ("+name+")");
      projectList.push(name);
      win.loadFile(idepath);
  })
  ipcMain.handle("init", () => iris.execFile("database/init.sql")),
  ipcMain.handle("getProjects", () => {
      return projectList;
  })
  ipcMain.handle("loadFile", (e, path) => win.loadFile(viewPath+path)),
  ipcMain.handle("openProject", (e, name) => {console.log(name);win.loadFile(idepath)})
})


