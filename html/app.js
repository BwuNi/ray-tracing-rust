const { ipcRenderer } = require('electron')
const initTasks = require('./task')

const canvas = document.getElementsByTagName('canvas')[0]
const ctx = canvas.getContext('2d')
const height = 400
const width = 800
const image = ctx.createImageData(width, height)

const reciver = initTasks(image, width, height)

function updataCtx() {
    ctx.putImageData(image, 0, 0)
    setTimeout(i => updataCtx(), 2000)
}
updataCtx()

let i = 0

ipcRenderer.send('task-init', [width, height])
ipcRenderer.on('px-complete', (event, arg) => {
    i += arg.length
    console.log(i / (height * width))
    arg.forEach(e => {
        reciver(e.x, e.y, e.rgb)
    });
})
