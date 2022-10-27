export class Connection {
    constructor(left, right) {
        this.left = left;
        this.right = right;
        this.left = left;
        this.right = right;
        registerEvents(left, "left");
        registerEvents(right, "right");
    }
    async createOffers() {
        // no await want to happen in parallel.
        let left = createOffer(this.left);
        let right = createOffer(this.right);
        return {
            left: await left,
            right: await right,
        };
    }
    async createAnswers() {
        let left = createAnswer(this.left);
        let right = createAnswer(this.right);
        return {
            left: await left,
            right: await right,
        };
    }
    getStreams() {
        return {
            left: getStream(this.left),
            right: getStream(this.right),
        };
    }
    async setAnswers(answer) {
        let left = this.left.setRemoteDescription(answer.left);
        let right = this.right.setRemoteDescription(answer.right);
        await Promise.all([left, right]);
    }
    isConnected() {
        let left = this.left.connectionState === "connected";
        let right = this.left.connectionState === "connected";
        return left && right;
    }
}
export async function fromStreams(leftStream, rightStream) {
    // no await want to happen in parallel.
    let left = fromStream(leftStream);
    let right = fromStream(rightStream);
    return new Connection(await left, await right);
}
export async function fromOffers(offers) {
    // no await want to happen in parallel.
    let left = fromOffer(offers.left);
    let right = fromOffer(offers.right);
    return new Connection(await left, await right);
}
/* ---- Private stuff --- */
/**
 * @param {RTCPeerConnection} peer
 */
async function createOffer(peer) {
    let offer = await peer.createOffer();
    peer.setLocalDescription(offer);
    while (peer.iceGatheringState != "complete") {
        await new Promise(resolve => setTimeout(resolve, 100));
    }
    return peer.localDescription;
}
/**
 * @param {RTCPeerConnection} peer
 */
async function createAnswer(peer) {
    let offer = await peer.createAnswer();
    peer.setLocalDescription(offer);
    while (peer.iceGatheringState != "complete") {
        await new Promise(resolve => setTimeout(resolve, 100));
    }
    return peer.localDescription;
}
/**
 * @param {RTCPeerConnection} peer
 */
function getStream(peer) {
    let stream = new MediaStream();
    for (const track of peer.getReceivers()) {
        stream.addTrack(track.track);
    }
    return stream;
}
/**
 * @param {MediaStream} stream
 */
async function fromStream(stream) {
    let peer = newPeer();
    for (const track of stream.getTracks()) {
        peer.addTrack(track, stream);
    }
    return peer;
}
async function fromOffer(offer) {
    let peer = newPeer();
    await peer.setRemoteDescription(offer);
    return peer;
}

function newPeer() {
    return new RTCPeerConnection({
        //iceServers: [
        //    {urls: `stun:stun.l.google.com:19302`},
        //    {urls: `stun:stun1.l.google.com:19302`},
        //    {urls: `stun:stun2.l.google.com:19302`},
        //    {urls: `stun:stun3.l.google.com:19302`},
        //    {urls: `stun:stun4.l.google.com:19302`}
        //]
    });
}

/**
 * @param {RTCPeerConnection} peer
 */
function registerEvents(peer, side) {
    peer.onconnectionstatechange = e => {
        console.log(side, "onconnectionstatechange", peer.connectionState);
    };
    peer.ondatachannel = e => {
        console.log(side, "ondatachannel");
    };
    peer.onicecandidate = e => {
        console.log(side, "onicecandidate", e.candidate);
    };
    peer.onicecandidateerror = e => {
        console.log(side, "onicecandidateerror");
    };
    peer.oniceconnectionstatechange = e => {
        console.log(side, "oniceconnectionstatechange", peer.iceConnectionState);
    };
    peer.onicegatheringstatechange = e => {
        console.log(side, "onicegatheringstatechange", peer.iceGatheringState);
    };
    peer.onnegotiationneeded = e => {
        console.log(side, "onnegotiationneeded");
    };
    peer.onsignalingstatechange = e => {
        console.log(side, "onsignalingstatechange", peer.signalingState);
    };
    peer.ontrack = e => {
        console.log(side, "ontrack");
    };
}