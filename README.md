# Smart HTML Renderer

A lightweight, distraction-free desktop application built with Tauri v2 that renders local HTML files in a clean window. It strips away standard browser gimmicks (like address bars and tabs) to give you an isolated, focused reading or presentation environment.

## Features

* **Distraction-Free UI:** A pure, borderless viewing experience for local HTML content.
* **Smart Link Routing:** Any external internet links (`http://` or `https://`) clicked within the local HTML page are automatically intercepted and opened in your system's default web browser, keeping the app focused solely on local content.
* **CLI Support:** Instantly open specific HTML files straight from your command line or terminal.
* **Local File Picker:** A minimal fallback interface to browse and select `.html` files if the app is launched normally without arguments.
* **Secure Asset Loading:** Safely maps local system paths to Tauri's internal `asset://` protocol.

## Installation & Usage

### Option 1: Download the Pre-built App (Windows)
1. Go to the [Releases](../../releases) page of this repository.
2. Download the latest `htmlrenderer_x64-setup.exe` from the Assets section.
3. Install the application.

### Option 2: Command Line Usage
You can launch the application directly from the Command Prompt (cmd) or PowerShell by passing the path of the HTML file you want to open.

```cmd
"C:\Path\To\Your\htmlrenderer.exe" "C:\Path\To\Your\document.html"
```

The app will bypass the file picker and immediately render `document.html`.

## Development Setup

To build and run this project locally, you need a few prerequisites.

### Prerequisites

1. **Node.js** (v20+)
2. **Rust & Cargo** (Install via [rustup](https://rustup.rs))
3. **OS-Specific Build Tools:**
   * **Windows:** Visual Studio C++ Build Tools (Desktop development with C++ workload).
   * **Linux (or GitHub Codespaces):**

     ```bash
     sudo apt update
     sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
     ```

### Running Locally

Clone the repository and install the frontend dependencies:

```bash
git clone https://github.com/xperiadeepseek/tauri.git
cd tauri/htmlrenderer
npm install
```

Start the development server:

```bash
npm run tauri dev
```

> **Note:** If you are running this inside a headless GitHub Codespace without a display manager, the app will compile but GTK will fail to open the window. Use the GitHub Actions build pipeline to generate testable executables instead.

## Building for Production

### Local Build

To compile the final, optimized application for your current operating system, run:

```bash
npm run tauri build
```

The finished installers will be located in `src-tauri/target/release/bundle/`.

### Cloud Build (GitHub Actions)

This repository includes a CI/CD workflow that automatically builds the Windows `.exe` installer in the cloud.

1. Navigate to the **Actions** tab in GitHub.
2. Select **Build Windows App** on the left sidebar.
3. Click the **Run workflow** dropdown on the right and trigger the build.
4. Once completed, the new installer will be automatically published as a **Draft Release** on the Releases page.

## How it Works

* **Backend (`src-tauri/src/lib.rs`):** Uses a custom `on_navigation` hook to monitor webview URL changes. It allows internal `tauri.localhost` and `asset.localhost` traffic, but intercepts `http`/`https` traffic and forwards it to the OS using `tauri-plugin-shell`. It also reads `std::env::args()` on startup and injects the requested file path into the frontend via an initialization script.
* **Frontend (`src/main.ts`):** Checks for the injected `window.__CLI_FILE_PATH__` variable. If present, it uses `convertFileSrc` to safely translate the raw system path into a secure Tauri asset URL and redirects the page. Otherwise, it loads the standard file dialog via `tauri-plugin-dialog`.