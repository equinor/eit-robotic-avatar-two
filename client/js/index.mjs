import React from "react";
import { createRoot } from 'react-dom/client';
import { RoboticAvatar } from "./view/RoboticAvatar";

const react_root = createRoot(document.getElementById("robotic_avatar"));
react_root.render(React.createElement(RoboticAvatar, null));
