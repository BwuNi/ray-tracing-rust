const ffi = require('ffi')

const rust = ffi.Library('./target/debug/ffi', {
    fib: ['int', ['int', 'int', 'int', 'int']]
});

const [width, height] = process.argv.splice(2).concat(['0', '0']).map(v => parseInt(v))

if (width === 0 && height === 0) return

const result = {
    value: [], // eg:[{ x: 0, y: 0, rgb: [255, 255, 255] }]
    add(x, y, rgb) {
        this.value.push({ x, y, rgb })
    },
    clear() {
        this.value = []
    },

    isSending: false,
    timeOut: null,

    send() {
        process.send(this.value);
        this.clear()
    },
    endSend() {
        this.isSending = false
        if (this.timeOut) clearTimeout(this.timeOut)
    },
    beginSend(time) {
        if (this.isSending) this.endSend()
        this.isSending = true
        const e = () => {
            this.send()
            this.timeOut = setTimeout(e, time)
        }
        e()
    }
}

const task = {
    pixels: [], // eg:[[{x:0,y:0}]]
    initPixels() {
        this.pixels = new Array(height).fill(0).map(
            (a, y) => new Array(width).fill(0).map(
                (b, x) => ({ x, y })
            )
        )
    },
    getPx() {
        if (this.pixels.length == 0) this.initPixels()

        const y = Math.floor(Math.random() * this.pixels.length)

        const pxs = this.pixels[y]

        const x = Math.floor(Math.random() * (pxs.length - 0.0001))

        const px = pxs.splice(x, 1)[0]

        if (pxs.length == 0) this.pixels.splice(y, 1)

        return px
    },

    calcu() {
        const { x, y } = this.getPx()
        const a = rust.fib(width, height, x, y)
        const rgb = [a >> 16, a % 65536 >> 8, a % 256 >> 0]
        return [x, y, rgb]
    },

    timeOut: null,
    stop() {
        if (this.timeOut) clearTimeout(this.timeOut)
    },
    run() {
        this.stop()
        result.beginSend(1000)

        const c = () => {
            new Array(100).fill(0).forEach(() => {
                result.add(...this.calcu())
            })
            this.timeOut = setTimeout(c, 0)
        }
        c()
    }

}

task.run()

// 使父进程输出: 父进程收到消息 { foo: 'bar', baz: null }


// let pixels = []
// let done = []
// let width = 0
// let height = 0
// let renderTimeOut = null
// let isSending = false
// let event = null



// function renderInit(w, h) {
//   width = w
//   height = h
//   pixels = new Array(width).fill(0).map((v, x) => new Array(height).fill(0).map((w, y) => ({
//     x, y
//   })))
// }

// function renderPx() {
//   let i = 80000

//   console.log(pixels.length)
//   while (i-- > 0) {
//     //if (pixels.length == 0) return clearTimeout(sendTimeOut)//renderInit(width, height)

//     if (pixels.length == 0) return isSending = false

//     if (renderTimeOut) clearTimeout(renderTimeOut)

//     const listIndex = Math.floor(Math.random() * pixels.length)

//     const pxs = pixels[listIndex]

//     const itemIndex = Math.floor(Math.random() * pxs.length)

//     const px = pxs.splice(itemIndex, 1)[0]

//     if (pxs.length == 0) pixels.splice(listIndex, 1)

//     const a = rust.fib(width, height, px.x, px.y)

//     done.push([px.x, px.y, [a >> 16, a / 256 >> 8, a / 65536 >> 0]])
//   }

//   renderTimeOut = setTimeout(() => {
//     renderPx()
//   }, 100);
// }


// function send() {
//   if (event) event.sender.send('px-complete', done)
//   if (isSending) setTimeout(i => send(), 1000)
// }

// isSending = true
// send() 