import { RoboticAvatar } from "./view/RoboticAvatar.mjs";

export function robotic_main(root_elem) {
    const react_root = ReactDOM.createRoot(root_elem);
    react_root.render(React.createElement(RoboticAvatar, null));
}
