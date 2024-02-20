
const { contextBridge, ipcRenderer } = require('electron')

contextBridge.exposeInMainWorld('message', {
  save: (path, new_txt) => ipcRenderer.invoke('save', path, new_txt),
  get_content: (path) => ipcRenderer.invoke('get_content', path),
  run: (path) => ipcRenderer.invoke('run', path),
  openide: () => ipcRenderer.invoke('openide'),
  loadFile: (file) => ipcRenderer.invoke("loadFile", file),
  addProject: (name) => ipcRenderer.invoke('addProject', name),
  init: () => ipcRenderer.invoke("init"),
  getProjects: async () => {
    const res = await ipcRenderer.invoke("getProjects");
    return res;
  },
  openProject: (name) => ipcRenderer.invoke("openProject", name) 
})
