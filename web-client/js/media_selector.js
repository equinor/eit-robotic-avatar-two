export async function user_video_id(id) {
    return await navigator.mediaDevices.getUserMedia({video: true});
}