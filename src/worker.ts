import wasm, {read_file, send_file} from "../src-rust/pkg/brain_render_backend";
import { AnatomicalAxis, NiftiProperties } from "./types";

type WorkerMessage =
    | {action: 'read-file', file: File}
    | {action: 'send-file', axis: AnatomicalAxis, coordinate: number}

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
            let slice = send_file(event.data.axis, event.data.coordinate);
            console.log(slice);
            postMessage({
                action: 'send-file',
                slice,
            });
            break;
    }
}

export {};
