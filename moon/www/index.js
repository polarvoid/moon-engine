import * as wasm from "moon-engine";
const canvas = document.getElementById("canvas");
const gl = canvas.getContext('webgl2');
const counter = document.getElementById("fpsCounter");
const app = new wasm.Application();

const FPS_LIMIT = 1000.0 / 30.0;
let lastDrawTime = -1;

function init() {
    if (!gl) {
        alert('Failed to initialize WebGL2 Context!');
        return;
    }
    app.init();

    canvas.addEventListener("keydown", event => {
        app.input(event.key, true);
    });
    canvas.addEventListener("keyup", event => {
        app.input(event.key, false);
    });
    canvas.addEventListener("mousemove", event => {
        app.mouse_move(event.clientX, event.clientY);
    }, false);

    app.resize(window.innerWidth, window.innerHeight);
    function render() {
        window.requestAnimationFrame(render);
        let currentTime = performance.now();
        let deltaTime = currentTime - lastDrawTime;
        if (deltaTime >= 0) { // Ignoring FPS_LIMIT for now
            lastDrawTime = currentTime;

            if (canvas.height != window.innerHeight || canvas.width != window.innerWidth) {
                canvas.height = window.innerHeight;
                canvas.style.height = window.innerHeight;
                
                canvas.width = window.innerWidth;
                canvas.style.width = window.innerWidth;
                app.resize(window.innerWidth, window.innerHeight);
            }
            app.render(deltaTime);
            counter.innerText = Math.round(1000/deltaTime);
        }
    }

    render();
}

init();