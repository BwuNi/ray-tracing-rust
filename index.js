const { app, BrowserWindow, ipcMain } = require('electron')
const { fork } = require('child_process');

const render =require('./node/rend')

function createWindow() {
  // 创建浏览器窗口
  win = new BrowserWindow({
    width: 1120, height: 720,
    webPreferences: {
      nodeIntegrationInWorker: true
    }
  })
  // 打开开发者工具
  win.webContents.openDevTools()
  // 然后加载应用的 index.html。
  win.loadFile('./html/index.html')
}

app.on('ready', createWindow)

let render_process = null

ipcMain.on('task-init', (e, [width, height]) => {
  if (render_process) {
    render_process.close()
    render_process = null
  }
  render_process = new RenderProcess(width, height, e)
})


class RenderProcess {
  static new(...arg) {
    return new this(...arg)
  }

  constructor(width = 0, height = 0, event = null) {
    this.width = width
    this.height = height
    this.process = null
    this.event = event
    this.openProcess()
  }

  openProcess() {
    // if (this.process) {
    //   this.process.kill()
    //   this.process = null
    // }
    // this.process = fork("./node/render.js", [this.width, this.height])
    // this.process.on('message', m=>this.onMessage(m));

    render(m=>this.onMessage(m))
  }

  onMessage(m) {
    if(this.event) this.event.sender.send('px-complete', m)
    // console.log(m)
  }

  close() {
    if (this.process) {
      this.process.kill()
      this.process = null
    }
    this.onMessage = () => { }
  }
}
