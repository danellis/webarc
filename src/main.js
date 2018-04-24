const wasm = require('./boot.rs');

console.log(wasm);

let futureWasm = wasm.initialize({noExitRuntime: true});

let request = new XMLHttpRequest();

request.open('GET', 'riscos311.rom', true);
request.responseType = 'arraybuffer';
request.onload = function (event) {
    let rom = new Uint8Array(request.response);

    futureWasm.then(module => {
        console.log(module.cwrap);
        // const boot = module.cwrap('boot', null, ['array']);
        console.log("Calling boot from JavaScript");
        // boot(rom);
    });
};

request.send();
