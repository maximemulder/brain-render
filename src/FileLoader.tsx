import { ChangeEvent, useState } from "react";
import { formatFileSize } from "./util";

type DemoFile = {
  name: string,
  size: number,
}

declare const DEMO_FILES: DemoFile[];

export default function FileLoader({onFileLoaded}: {onFileLoaded: (file: File) => void}) {
  const [isLoading, setIsLoading] = useState(false);
  const [downloadProgress, setDownloadProgress] = useState(0);

  async function handleFileChange(e: ChangeEvent<HTMLInputElement>) {
    if (e.target.files === null) {
      return;
    }

    onFileLoaded(e.target.files[0]);
  }

  async function handleLoadDemoFile(file: DemoFile) {
    setIsLoading(true);
    setDownloadProgress(0);

    try {
      const response = await fetch(`${import.meta.env.BASE_URL}assets/${file.name}`);

      if (!response.body || !response.ok) {
        console.error("[file-downloader] failed to fetch file");
        return;
      }

      let loaded = 0;

      const reader = response.body.getReader();
      const chunks: Uint8Array<ArrayBuffer>[] = [];

      while (true) {
        const { done, value } = await reader.read();

        if (done) {
          break;
        }

        chunks.push(value);
        loaded += value.length;

        setDownloadProgress(Math.round((loaded / file.size) * 100));
      }

      const blob = new Blob(chunks, { type: 'application/octet-stream' });
      const demoFile = new File([blob], file.name, { type: 'application/octet-stream' });

      onFileLoaded(demoFile);
    } catch (error) {
      console.error('Failed to load demo file:', error);
    } finally {
      setIsLoading(false);
      setDownloadProgress(0);
    }
  }

  return (
    <div className="file-loader">
      <div className="custom-file-loader">
        <h3>Use custom file</h3>
        <div className="custom-file">
          <input
            type="file"
            disabled={isLoading}
            onChange={handleFileChange}
            accept=".nii,.nii.gz"
          />
        </div>
      </div>
      <div className="demo-file-loader">
        <h3>Use demonstration files</h3>
        <div className="demo-files">
          {DEMO_FILES.map((file) => (
            <button
              key={file.name}
              type="button"
              className="demo-file-button"
              disabled={isLoading}
              onClick={() => handleLoadDemoFile(file)}
            >
              {file.name} ({formatFileSize(file.size)})
            </button>
          ))}
        </div>
      </div>
      {isLoading && (
        <progress value={downloadProgress} max={100} />
      )}
    </div>
  );
}
