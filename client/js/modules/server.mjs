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

