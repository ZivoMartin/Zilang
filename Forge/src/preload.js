
const { contextBridge, ipcRenderer } = require('electron')

contextBridge.exposeInMainWorld('messsage', {
  save: (path, new_txt) => ipcRenderer.invoke('save', path, new_txt),
  get_content: (path) => ipcRenderer.invoke('get_content', path),
  run: (path) => ipcRenderer.invoke('run', path),
  openide: () => ipcRenderer.invoke('openide'),
  addProject: (name) => ipcRenderer.invoke('addProject', name),
  init: () => ipcRenderer.invoke("init"),
  getProjects: () => ipcRenderer.invoke("getProjects")
})