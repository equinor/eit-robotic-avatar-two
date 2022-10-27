export class Connection {
    constructor(left, right) {
        this.left = left;
        this.right = right;
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
