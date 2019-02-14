module.exports = function initTasks(
    image,
    width,
    height,
) {


    canvas.height = height
    canvas.width = width

    const n = width * height

    let task = new Array(n).fill(0).map(i => ({
        n: 0,
        rgb: [0, 0, 0]
    }))


    return function reciver(x, y, [r, g, b]) {
        const i = x + y * width
        const p = i * 4

        // 更新 task 数据
        task[i].rgb = [r, g, b].map((v, index) => {
            return task[i].rgb[index] + v
        })
        task[i].n++

        // 更新画布

        image.data[p] = task[i].rgb[0] / task[i].n
        image.data[p + 1] = task[i].rgb[1] / task[i].n
        image.data[p + 2] = task[i].rgb[2] / task[i].n
        image.data[p + 3] = 255

    }

}