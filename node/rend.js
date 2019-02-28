
const spawn = require('child_process').spawn;


module.exports = function (call) {
    start(call)
}



function start(call) {
    const ls = spawn('./target/debug/ray-tracing-rust.exe', []);

    ls.stdout.on('data', (data) => {
        let arr = data
            .toString('utf8')
            .split('/')
            .filter(i => i)
            .map(v => v.split("-"))
            .map(v => ({
                x: parseInt(v[0]),
                y: parseInt(v[1]),
                rgb: v[2].split('.').map(v=>parseInt(v))
            }))
        call(arr);
    });

    ls.stderr.on('data', (data) => {
        console.log(`stderr: ${data}`);
    });

    ls.on('close', (code) => {
        console.log(`child process exited with code ${code}`);
    });

}