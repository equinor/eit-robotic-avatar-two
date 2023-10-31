import Viewport from "./viewport";

let webSocket = new WebSocket("ws://localhost:3000/ws");
webSocket.binaryType = "blob";
webSocket.onmessage = async (event) => {
    console.timeEnd("frame");
    console.time("frame");
    // Ask for next frame
    webSocket.send("f");
    let splitBlob = event.data.slice(0, 4);
    let eye0Length = new DataView(await splitBlob.arrayBuffer()).getUint32();
    let eye0Blob = event.data.slice(4, eye0Length + 4);
    let eye1Blob = event.data.slice(4 + eye0Length);
    console.timeLog("frame", "Decoded eyes", eye0Length);
    let eye0Bitmap = await createImageBitmap(eye0Blob);
    let eye1Bitmap = await createImageBitmap(eye1Blob);
    let eye0context = document.getElementById("eye-0").getContext("2d");
    let eye1context = document.getElementById("eye-1").getContext("2d");
    eye0context.drawImage(eye0Bitmap, 0, 0)
    eye1context.drawImage(eye1Bitmap, 0, 0)
    console.timeLog("frame", "Painted eyes");
}

webSocket.onopen = function () {
    webSocket.send("f");
    console.time("frame");
}