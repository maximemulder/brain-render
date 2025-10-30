import wasm, {init_renderer, read_file, render_slice} from "../src-rust/pkg/brain_render_backend";
import { AnatomicalAxis, NiftiProperties, DisplayWindow } from "./types";

type WorkerMessage =
  | {action: 'init-renderer', canvas: OffscreenCanvas}
  | {action: 'read-file', file: File}
  | {action: 'render-slice', window: DisplayWindow, axis: AnatomicalAxis, coordinate: number}

onmessage = async (event: MessageEvent<WorkerMessage>) => {
  await wasm();
  switch (event.data.action) {
    case 'init-renderer':
      console.debug("[web-worker] initialize renderer");
      await init_renderer(event.data.canvas);
      postMessage({
        action: 'init-renderer',
      });
      break;
    case 'read-file':
      console.debug("[web-worker] read nifti file");
      let properties: NiftiProperties = await read_file(event.data.file);
      postMessage({
        action: 'read-file',
        properties,
      });
      break;
    case 'render-slice':
      console.debug("[web-worker] render slice");
      render_slice(event.data.axis, event.data.coordinate, event.data.window);
  }
}

export {};
