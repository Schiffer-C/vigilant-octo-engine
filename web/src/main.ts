import init, { Game } from "./wasm/engine.js"
import { WASM_BUILD_ID } from "./wasm_version.js";

console.log("WASM_BUILD_ID: ", WASM_BUILD_ID);

async function run() {
    await init();

    const game = new Game();
    game.move_by(1, 6);

    console.log("Player: ", game.pos_x(), game.pos_y());
}

run();