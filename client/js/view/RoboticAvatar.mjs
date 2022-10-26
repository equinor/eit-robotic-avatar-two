import { listDevices, loadCams } from "../modules/cameras.mjs";
import { fromOffers, fromStreams } from "../modules/rtc.mjs";
import { postAnswer, postOffers, postTracking, pullAnswer, pullOffers } from "../modules/server.mjs";
import Viewport from "../view/Viewport.mjs";

export class RoboticAvatar extends React.Component {
    constructor(props) {
        super(props);
        this.sending = false;

        this.setStreams = (streams) => {
            this.setState(streams) 
        };

        this.handleSource = async () => {
            try {
                this.setState({ started: true });
                await source(this.setStreams, this.props.leftCamId, this.props.rightCamId);
            }
            catch (err) {
                console.error(err);
            }
        };

        this.handleReceiver = async () => {
            try {
                this.setState({ started: true });
                this.setState(await receiver());
            }
            catch (err) {
                console.log(err);
            }
        };

        this.handleTracking = async (track) => {
            try {
                if (this.sending)
                    return;
                this.sending = true;
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
                this.sending = false;
            }
            catch (err) {
                console.log(err);
            }
        };

        this.state = {
            started: false,
        };
    }

    render() {
        console.log(this.props);
        return React.createElement(React.Fragment, null,
            React.createElement("div", null,
                React.createElement("p", null,
                    React.createElement("button", { disabled: this.state.started, onClick: this.handleSource }, "Start as source"),
                    React.createElement("button", { disabled: this.state.started, onClick: this.handleReceiver }, "Start as receiver"))),
            React.createElement(Viewport, { left: this.state.left, right: this.state.right, onTrack: this.handleTracking }));
    }
}

async function source(setStreams, leftCamId, rightCamId) {
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

async function receiver() {
    let offers = await pullOffers();
    console.log(offers);
    let con = await fromOffers(offers);
    let answer = await con.createAnswers();
    console.log(answer);
    await postAnswer(answer);
    return con.getStreams();
}