import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";
import fs from "fs";
import path from 'path';

type DemoFile = {
  name: string,
  size: number,
}

function getDemoFiles(): DemoFile[] {
  const assetsPath = "./public/assets";
  const demoFiles: DemoFile[] = [];

  const fileNames = fs.readdirSync(assetsPath).filter(file => file.endsWith('.nii'));

  for (const fileName of fileNames) {
    const filePath = path.join(assetsPath, fileName);
    const stats = fs.statSync(filePath);
    demoFiles.push({
      name: fileName,
      size: stats.size,
    });
  }

  return demoFiles;
}

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(({mode}) => ({
  plugins: [react(), topLevelAwait(), wasm()],
  base: mode === 'production' ? '/brain-render/' : '/',
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
  },
  css: {
    modules: {
      localsConvention: 'camelCase'
    }
  },
  define: {
    DEMO_FILES: getDemoFiles(),
  },
}));
