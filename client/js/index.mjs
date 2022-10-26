import Viewport from "./view/Viewport.mjs";

export function minion_root(root_elem) {
    return ReactDOM.createRoot(root_elem);
}

export function render(root, left, right, onTrack) {
    root.render(React.createElement(Viewport, {
        left,
        right,
        onTrack
    }))
}
