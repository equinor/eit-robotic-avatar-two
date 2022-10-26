import { RoboticAvatar } from "./view/RoboticAvatar.mjs";

export function minion_root(root_elem) {
    return ReactDOM.createRoot(root_elem);
}

export function render(root, left, right) {
    root.render(React.createElement(RoboticAvatar, {
        left,
        right
    }))
}
