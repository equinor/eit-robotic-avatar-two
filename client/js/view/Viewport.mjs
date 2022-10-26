import * as THREE from 'three';
import { VRButton } from 'https://unpkg.com/three@0.142.0/examples/jsm/webxr/VRButton.js';

const Canvas = styled.canvas `
    background-color: #000;
    height: 100%;
    width: 100%;
`;

const HiddenVideo = styled.video `
    display: none;
`;

export default class Viewport extends React.Component {
    constructor() {
        super(...arguments);
        this.canvas = React.createRef();
        this.left = React.createRef();
        this.right = React.createRef();
    }

    render() {
        return React.createElement("div", { className: this.props.className },
            React.createElement(Canvas, { ref: this.canvas }),
            React.createElement(HiddenVideo, { autoPlay: true, ref: this.left }),
            React.createElement(HiddenVideo, { autoPlay: true, ref: this.right }));
    }

    componentDidMount() {
        this.componentDidUpdate();
        setup_3d(this.canvas.current, this.left.current, this.right.current, this.props.onTrack);
    }

    componentDidUpdate() {
        var _a, _b;
        this.left.current.srcObject = (_a = this.props.left) !== null && _a !== void 0 ? _a : null;
        this.right.current.srcObject = (_b = this.props.right) !== null && _b !== void 0 ? _b : null;
    }
}

function toController(game) {
    var _a, _b, _c, _d, _e, _f, _g, _h, _j, _k;
    //I am just guessing
    return {
        x: (_a = game.axes[2]) !== null && _a !== void 0 ? _a : 0,
        y: (_b = game.axes[3]) !== null && _b !== void 0 ? _b : 0,
        a: (_d = (_c = game.buttons[4]) === null || _c === void 0 ? void 0 : _c.pressed) !== null && _d !== void 0 ? _d : false,
        b: (_f = (_e = game.buttons[5]) === null || _e === void 0 ? void 0 : _e.pressed) !== null && _f !== void 0 ? _f : false,
        c: (_h = (_g = game.buttons[0]) === null || _g === void 0 ? void 0 : _g.value) !== null && _h !== void 0 ? _h : 0,
        d: (_k = (_j = game.buttons[1]) === null || _j === void 0 ? void 0 : _j.value) !== null && _k !== void 0 ? _k : 0,
    };
}

export function setup_3d(canvas, left_video, right_video, onTrack) {
    // Based on the assumption both Oculus Quest 2 and Webcam have a diagonal field of view of 90. But a different aspect ratio.
    let fov = 0.830097;

    const scene = new THREE.Scene();
    const camera = new THREE.OrthographicCamera(-1, 1, 1, -1, -1, 1);
    const renderer = new THREE.WebGLRenderer({
        canvas: canvas
    });

    renderer.setSize(3664, 1920, false);
    renderer.xr.enabled = true;
    renderer.xr.cameraAutoUpdate = false;
    // @ts-ignore
    renderer.xr.getCamera = function () {
        return camera;
    };

    const textureLeft = new THREE.VideoTexture(left_video);
    textureLeft.center = new THREE.Vector2(0.5, 0.5);
    textureLeft.rotation = 90 * 3 * (Math.PI / 180);
    const left = new THREE.Mesh(new THREE.PlaneGeometry(0.58952 * fov, 2 * fov), new THREE.MeshBasicMaterial({ map: textureLeft }));
    left.position.x = -0.5;
    scene.add(left);
    const textureRight = new THREE.VideoTexture(right_video);
    textureRight.center = new THREE.Vector2(0.5, 0.5);
    textureRight.rotation = 90 * 1 * (Math.PI / 180);
    const right = new THREE.Mesh(new THREE.PlaneGeometry(0.58952 * fov, 2 * fov), new THREE.MeshBasicMaterial({ map: textureRight }));
    right.position.x = 0.5;
    scene.add(right);

    renderer.setAnimationLoop((_, xrframe) => {
        if (renderer.xr.isPresenting && onTrack) {
            const r = xrframe.getViewerPose(renderer.xr.getReferenceSpace()).transform.orientation;
            let track = {
                rx: r.x,
                ry: r.y,
                rz: r.z,
                l: { x: 0, y: 0, a: false, b: false, c: 0, d: 0 },
                r: { x: 0, y: 0, a: false, b: false, c: 0, d: 0 },
            };
            xrframe.session.inputSources.forEach(function (input) {
                if (input.handedness === "left" && input.gamepad) {
                    track.l = toController(input.gamepad);
                }
                else if (input.handedness === "right" && input.gamepad) {
                    track.r = toController(input.gamepad);
                }
            });
            onTrack(track);
        }
        renderer.render(scene, camera);
    });
    document.body.appendChild(VRButton.createButton(renderer));
}