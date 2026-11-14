import { open } from '@tauri-apps/plugin-dialog';
import { convertFileSrc } from '@tauri-apps/api/core';

// Tell TypeScript about our custom global variable
declare global {
  interface Window {
    __CLI_FILE_PATH__: string;
  }
}

// 1. Check if the app was launched with a file path via CMD
if (window.__CLI_FILE_PATH__ && window.__CLI_FILE_PATH__.trim() !== "") {
  const assetUrl = convertFileSrc(window.__CLI_FILE_PATH__);
  window.location.href = assetUrl;
}

// 2. Fallback UI if launched normally without arguments
document.getElementById('open-btn')?.addEventListener('click', async () => {
  const filePath = await open({
    multiple: false,
    filters: [{ name: 'HTML', extensions: ['html', 'htm'] }]
  });
  
  if (filePath) {
    const assetUrl = convertFileSrc(filePath);
    window.location.href = assetUrl;
  }
});