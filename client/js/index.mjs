//import React from "react";
//import { createRoot } from 'react-dom/client';
//import { RoboticAvatar } from "./view/RoboticAvatar";

export function robotic_main(root_elem) {
    const react_root = createRoot(root_elem);
    react_root.render(React.createElement(RoboticAvatar, null));
}

window.robotic_main = robotic_main;
