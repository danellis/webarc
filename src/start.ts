import { Cpu } from './cpu';


declare let window:any;
window.cpu = null;

let request = new XMLHttpRequest();
request.open('GET', 'riscos311.rom', true);
request.responseType = 'arraybuffer';
request.onload = function (event) {
    window.cpu = new Cpu(request.response);
};
request.send();
