const Iris = require("./iris.js");

const { app, BrowserWindow, ipcMain } = require('electron');

const repo_path = require('node:path')

const fs = require('fs');

const { exec } = require('child_process');

const index_path = "view/index.html";
const idepath = "view/code.html";
const compile_command = (path) => "cd ../compiler && cargo run ../Forge/" + path + " -o exe";

const exec_command = "../compiler/exe";

const iris = new Iris();


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
      exec(compile_command(path), () => {})
      exec(exec_command, (error, stdout, stderr) => {
        if (error) {
          console.error(`error: ${error.message}`);
          return;
        }
        if (stderr) {
          console.error(`stderr: ${stderr}`);
        }
        console.log(`stdout:\n${stdout}`);
      }); 
    }else{
      console.log("You can only run .vu files..");
    }
  });
  ipcMain.handle("get_content", (e, path) => fs.readFileSync(path, {encoding: 'utf8'}))
  ipcMain.handle("openide", () => win.loadFile(idepath))
  ipcMain.handle("addProject", () => {
    iris.newRequest("SELECT the_name FROM Humain WHERE age>18");
  })
})

