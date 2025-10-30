import { ChangeEvent, useEffect, useState } from "react";
import { formatFileSize, getFileName } from "./util";

type DemoFile = {
  name: string,
  size: number,
  url: string,
}

async function getDemoFiles() {
  const files: Record<string, {default: string}> = import.meta.glob('/public/assets/*.nii', {
    eager: true,
    query: '?url',
  });

  const demoFiles: DemoFile[] = [];

  for (const module of Object.values(files)) {
    let path = module.default;
    const name = getFileName(path);
    const url = path;

    const response = await fetch(url, { method: 'HEAD' });

    const contentLength = response.headers.get('content-length');
    const size = contentLength ? parseInt(contentLength, 10) : 0;

    demoFiles.push({
      name,
      size,
      url,
    });
  }

  return demoFiles;
}

export default function FileLoader({onFileLoaded}: {onFileLoaded: (file: File) => void}) {
  const [demoFiles, setDemoFiles] = useState<DemoFile[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [downloadProgress, setDownloadProgress] = useState(0);

  useEffect(() => {
    async function scanDemoFiles() {
      setDemoFiles(await getDemoFiles());
    }

    scanDemoFiles();
  }, []);

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
      const response = await fetch(file.url);

      if (!response.body || !response.ok) {
        console.error("[file-downloader] failed to fetch file");
        return;
      }

      const contentLength = response.headers.get('content-length');
      if (contentLength === null) {
        console.error("[file-downloader] failed to get file size");
        return;
      }

      const total = file.size
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

        setDownloadProgress(Math.round((loaded / total) * 100));
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
          {demoFiles.map((file) => (
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
