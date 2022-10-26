import { loadCams } from "../modules/cameras.mjs";
import { fromOffers, fromStreams } from "../modules/rtc.mjs";
import { postAnswer, postOffers, postTracking, pullAnswer, pullOffers } from "../modules/server.mjs";


export async function source(setStreams, leftCamId, rightCamId) {
    let cams = await loadCams(leftCamId, rightCamId);
    setStreams(cams);
    let con = await fromStreams(cams);
    let offers = await con.createOffers();
    console.log(offers);
    await postOffers(offers);
    let answer = await pullAnswer();
    console.log(answer);
    await con.setAnswers(answer);
}

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