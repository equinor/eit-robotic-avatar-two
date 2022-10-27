import { postTracking } from "../modules/server.mjs";

export async function tracking(track) {
    await postTracking({
        head: {
            rx: track.rx,
            ry: track.ry,
            rz: track.rz,
        },
        drive: {
            speed: track.l.y,
            turn: track.l.x,
        }
    });
}