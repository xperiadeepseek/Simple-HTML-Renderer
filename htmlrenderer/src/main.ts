import { open } from '@tauri-apps/plugin-dialog';
import { convertFileSrc } from '@tauri-apps/api/core';

document.getElementById('open-btn')?.addEventListener('click', async () => {
  const filePath = await open({
    multiple: false,
    filters: [{ name: 'HTML', extensions: ['html', 'htm'] }]
  });
  
  if (filePath) {
    // Converts the system file path to a Tauri asset:// protocol URL
    const assetUrl = convertFileSrc(filePath);
    // Navigate the webview directly to the local HTML file
    window.location.href = assetUrl;
  }
});