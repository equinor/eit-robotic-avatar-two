import { listDevices, loadCams } from "../modules/cameras.mjs";
import { fromOffers, fromStreams } from "../modules/rtc.mjs";
import { postAnswer, postOffers, postTracking, pullAnswer, pullOffers } from "../modules/server.mjs";
import Viewport from "../view/Viewport.mjs";

const GlobalStyle = styled.createGlobalStyle `
    html, body, #robotic_avatar {
        margin: 0;
        height: 100%;
    }
`;

const Grid = styled.main `
    height: 100%;
    display: grid;
    box-sizing: border-box;
    grid-template-columns: 1fr;
    grid-template-rows: auto 1fr;
    grid-template-areas: 
        "ui"
        "view";
    gap: 16px 16px;
    background-color: rgb(220, 220, 220);
    padding: 8px;
`;

const Ui = styled.div `
    grid-area: ui;
`;

const View = styled(Viewport) `
    grid-area: view;
`;

const LeftCameraId = "LeftCameraId";
const RightCameraId = "RightCameraId";
export class RoboticAvatar extends React.Component {
    constructor(props) {
        var _a, _b;
        super(props);
        this.sending = false;

        this.handleLeftCam = (event) => {
            const value = event.target.value;
            localStorage.setItem(LeftCameraId, value);
            this.setState({ leftCamId: value });
        };

        this.handleRightCam = (event) => {
            const value = event.target.value;
            localStorage.setItem(RightCameraId, value);
            this.setState({ rightCamId: value });
        };

        this.handleSource = async () => {
            try {
                this.setState({ started: true });
                let cams = await loadCams(this.state.leftCamId, this.state.rightCamId);
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

        this.handleSourceNoView = async () => {
            try {
                this.setState({ started: true });
                let cams = await loadCams(this.state.leftCamId, this.state.rightCamId);
                // this.setState(cams);
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

        const left = (_a = localStorage.getItem(LeftCameraId)) !== null && _a !== void 0 ? _a : "";
        const right = (_b = localStorage.getItem(RightCameraId)) !== null && _b !== void 0 ? _b : "";
        this.state = {
            started: false,
            leftCamId: left,
            rightCamId: right,
            devices: []
        };
        listDevices().then(devices => this.setState({ devices: devices }));
    }

    render() {
        const devices = this.state.devices.map(device => React.createElement("li", null,
            device[0],
            ": ",
            device[1]));
        return React.createElement(Grid, null,
            React.createElement(GlobalStyle, null),
            React.createElement(Ui, null,
                React.createElement("h1", null, "Robotic Avatar Demo"),
                React.createElement("p", null,
                    "Left Camera ID: ",
                    React.createElement("input", { size: 64, value: this.state.leftCamId, onChange: this.handleLeftCam }),
                    React.createElement("br", null),
                    "Right Camera ID ",
                    React.createElement("input", { size: 64, value: this.state.rightCamId, onChange: this.handleRightCam }),
                    React.createElement("ul", null, devices)),
                React.createElement("p", null,
                    React.createElement("button", { disabled: this.state.started, onClick: this.handleSource }, "Start as source"),
                    React.createElement("button", { disabled: this.state.started, onClick: this.handleSourceNoView }, "Start as source NO VIEWPORT"),
                    React.createElement("button", { disabled: this.state.started, onClick: this.handleReceiver }, "Start as receiver"))),
            React.createElement(View, { left: this.state.left, right: this.state.right, onTrack: this.handleTracking }));
    }
}