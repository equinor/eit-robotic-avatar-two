import { fromOffers } from "../modules/rtc.mjs";
import { postAnswer, postTracking, pullOffers } from "../modules/server.mjs";

export async function receiver() {
    let offers = await pullOffers();
    console.log(offers);
    let con = await fromOffers(offers);
    let answer = await con.createAnswers();
    console.log(answer);
    await postAnswer(answer);
    return con.getStreams();
}

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