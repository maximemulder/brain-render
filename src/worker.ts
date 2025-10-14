import {read_file} from "../dist/brain_render_backend";

onmessage = async (event: MessageEvent<{file: File}>) => {
    await read_file(event.data.file);
}

export {};
