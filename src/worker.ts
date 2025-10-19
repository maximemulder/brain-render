import {read_file} from "../src-rust/pkg/brain_render_backend";

type WorkerMessage =
    | {action: 'read-file', file: File}
    | {action: 'send-file'}

onmessage = async (event: MessageEvent<WorkerMessage>) => {
    switch (event.data.action) {
        case 'read-file':
            console.log("Web worker read file.");
            return await read_file(event.data.file);
        case 'send-file':
            console.log("Web worker send file.");
            return 0;
    }
}

export {};
