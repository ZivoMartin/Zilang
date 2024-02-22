const Iris = require("./iris.js");

const { app, BrowserWindow, ipcMain } = require('electron');

const repo_path = require('node:path')

const fs = require('fs');

const { exec } = require('child_process');

const index_path = "view/index.html";
const idepath = "view/code.html";
const viewPath = "./view/"
const userProjectsFolder = "database/userProjects/"

const baseTxt = "import ../std/stdio.vu;\n\n!exit(0);"

var currentProject = null;

const compile_command = (fileName) => "cd ../compiler && cargo run ../Forge/"+ userProjectsFolder+currentProject+"/"+fileName+" -o exe";

const exec_command = "../compiler/exe";


const outputManagment = (error, stdout, stderr, type_op) => {
  if (error) {
    console.error(type_op + ` error: ${error.message}`);
    return false;
  }
  if (stderr) console.error(type_op + ` stderr: ${stderr}`);
  if (stdout) console.log(type_op + ` stdout:\n${stdout}`);      
  return true;  
};



app.whenReady().then(async () => {

  const iris = new Iris()

  const win = new BrowserWindow({
    width: 2000,
    height: 1000,
    fullscreen: true,
    webPreferences: {
      preload: repo_path.join(__dirname, 'preload.js')
    }
  })

  win.setFullScreen(true)
  win.loadFile(index_path)

  const getCurrentFile = async () => {
    await iris.newRequest("SELECT firstFile FROM Projects WHERE p_name == \'"+currentProject+"'");
    return iris.extract_json().firstFile;
  }

  ipcMain.handle('save', (e, fileName, new_txt) => {
    return new Promise(async (resolve, reject) => {
      fs.writeFile(userProjectsFolder+currentProject+"/"+fileName, new_txt, (e)=>{
        if (e) throw e
        resolve(e)
      });
    })
  })
  ipcMain.handle('run', (e, path) => {
    if (path.endsWith(".vu")) {
      return new Promise((resolve, reject) => {
        exec(compile_command(path), (error, stdout, stderr) => {
          if (outputManagment(error, stdout, stderr, "Compilation")){
            exec(exec_command, (error, stdout, stderr) =>  {
              outputManagment(error, stdout, stderr, "Execution")
              resolve(stdout, error);
            });
          }   
        });
      }).then((stdout, error) => {
          if (error) return error;
          return stdout
      })
    }else{
      console.error("Forge error: You can only run .vu files..");
    }
  });
  ipcMain.handle("get_content", (e, path) => fs.readFileSync(path, {encoding: 'utf8'}))
  ipcMain.handle("openide", () => win.loadFile(idepath))
  ipcMain.handle("addProject", (e, name) => {
      fs.mkdirSync(userProjectsFolder + name);
      fs.appendFile(userProjectsFolder + name+"/main.vu", baseTxt, (e) => {if (e) console.error("Failed to create the main.vu file: " + e)});
      iris.newRequest("INSERT INTO Projects (p_name) VALUES ("+name+")");   
      iris.newRequest("INSERT INTO Files (file_path, p_name) VALUES ("+name+","+name+"/main.vu)");
      win.loadFile(idepath);
      currentProject = name;
  })
  ipcMain.handle("init", () => iris.execFile("database/init.sql")),
  ipcMain.handle("getProjects", () => {
      return iris.getProjectList();
  })
  ipcMain.handle("loadFile", (e, path) => win.loadFile(viewPath+path)),
  ipcMain.handle("openProject", (e, name) => {
    currentProject = name;
    win.loadFile(idepath);
  })
  ipcMain.handle("getFirstFileCurrentProject", async (e) => {
    const firstFiles = await getCurrentFile();
    return {
      nameFile: firstFiles[0],
      txt: fs.readFileSync(userProjectsFolder+currentProject+"/"+firstFiles[0], {encoding: 'utf8'})
    }
  })
})