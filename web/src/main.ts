import init, { Game, memory_access } from "./wasm/engine.js"
import { WASM_BUILD_ID } from "./wasm_version.js";

console.log("WASM_BUILD_ID: ", WASM_BUILD_ID);

const TILE_PX = 16;


function rgbToCss(rgb: number): string {
    return "#" + rgb.toString(16).padStart(6, "0");
}

async function main() {
    await init(new URL("./wasm/engine_bg.wasm", import.meta.url));

    const memory = memory_access() as WebAssembly.Memory;
    
    const viewW = 80;
    const viewH = 45;

    const canvas = document.getElementById("game") as HTMLCanvasElement;
    canvas.width = viewW * TILE_PX;
    canvas.height = viewH * TILE_PX;

    const ctx = canvas.getContext("2d")!;
    ctx.font = `${TILE_PX}px ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace`;
    ctx.textBaseline = "top";

    const game = new Game(12345, viewW, viewH);

    function draw() {
        game.prepare_render_buff();

        const len = game.buff_len();
        const bgPtr = game.bg_rgb_buff_ptr();
        const fgPtr = game.fg_rgb_buff_ptr();
        const glyphPtr = game.glyph_buff_ptr();

        console.log({ len, bgPtr, fgPtr, glyphPtr });

        const bg = new Uint32Array(memory.buffer, game.bg_rgb_buff_ptr(), len);
        const fg = new Uint32Array(memory.buffer, game.fg_rgb_buff_ptr(), len);
        const glyph = new Uint32Array(memory.buffer, game.glyph_buff_ptr(), len);

        const center = Math.floor(viewW / 2) + Math.floor(viewH / 2) * viewW;
        console.log("center glyph code:", glyph[center], "fg:", fg[center].toString(16));
        console.log("center glyph code:", glyph[center], "bg:", bg[center].toString(16));

        for(let y = 0; y < viewH; y++) {
            for(let x = 0; x < viewW; x++) {
                const i = y * viewW + x;

                ctx.fillStyle = rgbToCss(bg[i]);
                ctx.fillRect(x * TILE_PX, y * TILE_PX, TILE_PX, TILE_PX);

                const code = glyph[i];
                if(code !== 0) {
                    ctx.fillStyle = rgbToCss(fg[i]);
                    ctx.fillText(String.fromCodePoint(code), x * TILE_PX, y * TILE_PX);
                }
            }
        }

        requestAnimationFrame(draw);
    }

    window.addEventListener("keydown", (e) => {
        if (e.key === "ArrowUp") game.move_by(0, -1);
        if (e.key === "ArrowDown") game.move_by(0, 1);
        if (e.key === "ArrowLeft") game.move_by(-1, 0);
        if (e.key === "ArrowRight") game.move_by(1, 0);
    });

    draw();
}

main();
