export function clamp(min: number, max: number, value: number): number {
  return Math.max(min, Math.min(max - 1, value));
}

export function formatFileSize(bytes: number): string {
  if (bytes === 0) {
    return '0 B';
  }

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

export function getFileName(path: string): string {
  let parts = path.split('/');
  return parts[parts.length - 1];
}
