export class Connection {
    constructor(left, right) {
        this.left = left;
        this.right = right;
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
function getStream(peer) {
    let stream = new MediaStream();
    for (const track of peer.getReceivers()) {
        stream.addTrack(track.track);
    }
    return stream;
}
