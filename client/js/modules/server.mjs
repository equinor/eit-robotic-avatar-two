export async function postOffers(offers) {
    return await postRtc("/api/minion/post_offer", offers);
}

// pull and server until we get something looks like an offer.
export async function pullOffers() {
    return await pullRtc("/api/minion/get_offer");
}

export async function postAnswer(answer) {
    return await postRtc("/api/minion/post_answer", answer);
}

// pull and server until we get something looks like an answer.
export async function pullAnswer() {
    return await pullRtc("/api/minion/get_answer");
}

export async function postTracking(tracking) {
    return await postRtc("/api/minion/tracking", tracking);
}

/* ---- Private stuff --- */
async function postRtc(path, payload) {
    await fetch(path, {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
    });
}

// pull and server until we get something looks valid.
async function pullRtc(path) {
    while (true) {
        let req = await fetch(path);
        let offer = await req.json();
        if (offer != undefined && offer.left != undefined) {
            return offer;
        }
        // Wait 5 sec
        await new Promise(resolve => setTimeout(resolve, 5000));
    }
}
