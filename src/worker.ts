import wasm, {read_file, send_file} from "../src-rust/pkg/brain_render_backend";
import { NiftiPoint3D, NiftiProperties, NiftiSliceOrientation } from "./types";

type WorkerMessage =
    | {action: 'read-file', file: File}
    | {action: 'send-file', focalPoint: NiftiPoint3D, orientation: NiftiSliceOrientation}

onmessage = async (event: MessageEvent<WorkerMessage>) => {
    await wasm();
    switch (event.data.action) {
        case 'read-file':
            console.log("Web worker read file.");
            let properties: NiftiProperties = await read_file(event.data.file);
            postMessage({
                action: 'read-file',
                properties,
            });
            break;
        case 'send-file':
            console.log("Web worker send file.");
            let slice = send_file(event.data.focalPoint, event.data.orientation);
            console.log(slice);
            postMessage({
                action: 'send-file',
                slice,
            });
            break;
    }
}

export {};
