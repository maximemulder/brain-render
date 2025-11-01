import wasm, {initRenderer, readFile, renderSlice} from "../src-rust/pkg/brain_renderer";
import { AnatomicalAxis, NiftiProperties, DisplayWindow } from "./types";

type WorkerMessage =
  | {action: 'init-renderer', canvas: OffscreenCanvas}
  | {action: 'read-file', file: File}
  | {action: 'render-slice', window: DisplayWindow, axis: AnatomicalAxis, coordinate: number, timepoint: number}

onmessage = async (event: MessageEvent<WorkerMessage>) => {
  await wasm();
  switch (event.data.action) {
    case 'init-renderer':
      console.debug("[web-worker] initialize renderer");
      let result = await initRenderer(event.data.canvas);
      postMessage({
        action: 'init-renderer',
        result,
      });
      break;
    case 'read-file':
      console.debug("[web-worker] read nifti file");
      let properties: NiftiProperties = await readFile(event.data.file);
      postMessage({
        action: 'read-file',
        properties,
      });
      break;
    case 'render-slice':
      console.debug("[web-worker] render slice");
      renderSlice(event.data.axis, event.data.coordinate, event.data.timepoint, event.data.window);
  }
}

export {};
