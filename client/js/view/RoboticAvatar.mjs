import { listDevices, loadCams } from "../modules/cameras.mjs";
import { fromOffers, fromStreams } from "../modules/rtc.mjs";
import { postAnswer, postOffers, postTracking, pullAnswer, pullOffers } from "../modules/server.mjs";
import Viewport from "../view/Viewport.mjs";

const LeftCameraId = "LeftCameraId";
const RightCameraId = "RightCameraId";
export class RoboticAvatar extends React.Component {
    constructor(props) {
        super(props);
        this.sending = false;

        this.handleSource = async () => {
            try {
                this.setState({ started: true });
                let cams = await loadCams(this.props.leftCamId, this.props.rightCamId);
                this.setState(cams);
                this.setState({ devices: await listDevices() });
                let con = await fromStreams(cams);
                let offers = await con.createOffers();
                console.log(offers);
                await postOffers(offers);
                let answer = await pullAnswer();
                console.log(answer);
                await con.setAnswers(answer);
            }
            catch (err) {
                console.error(err);
            }
        };

        this.handleReceiver = async () => {
            try {
                this.setState({ started: true });
                let offers = await pullOffers();
                console.log(offers);
                let con = await fromOffers(offers);
                let answer = await con.createAnswers();
                console.log(answer);
                await postAnswer(answer);
                let streams = con.getStreams();
                this.setState(streams);
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
            devices: []
        };
        listDevices().then(devices => this.setState({ devices: devices }));
    }

    render() {
        console.log(this.props);
        const devices = this.state.devices.map(device => React.createElement("li", null,
            device[0],
            ": ",
            device[1]));
        return React.createElement(React.Fragment, null,
            React.createElement("div", null,
                React.createElement("p", null,
                    React.createElement("ul", null, devices)),
                React.createElement("p", null,
                    React.createElement("button", { disabled: this.state.started, onClick: this.handleSource }, "Start as source"),
                    React.createElement("button", { disabled: this.state.started, onClick: this.handleReceiver }, "Start as receiver"))),
            React.createElement(Viewport, { left: this.state.left, right: this.state.right, onTrack: this.handleTracking }));
    }
}