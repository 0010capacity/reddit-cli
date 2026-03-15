const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const VERSION = '0.1.1';
const GITHUB_REPO = '0010capacity/reddit-cli';

function getPlatform() {
  const platform = process.platform;
  const arch = process.arch;

  const map = {
    'darwin-x64': 'x86_64-apple-darwin',
    'darwin-arm64': 'aarch64-apple-darwin',
    'linux-x64': 'x86_64-unknown-linux-gnu',
    'linux-arm64': 'aarch64-unknown-linux-gnu',
    'win32-x64': 'x86_64-pc-windows-msvc'
  };

  const key = `${platform}-${arch}`;
  return map[key] || null;
}

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        download(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }
      response.pipe(file);
      file.on('finish', () => {
        file.close();
        resolve();
      });
    }).on('error', (err) => {
      fs.unlink(dest, () => {});
      reject(err);
    });
  });
}

async function install() {
  const platform = getPlatform();

  if (!platform) {
    console.error(`Unsupported platform: ${process.platform}-${process.arch}`);
    console.log('Please build from source: cargo install --path .');
    process.exit(1);
  }

  const binDir = path.join(__dirname, 'binaries');
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  const ext = process.platform === 'win32' ? '.exe' : '';
  const binaryName = `reddit-cli-${platform}${ext}`;
  const downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/v${VERSION}/${binaryName}`;
  const destPath = path.join(binDir, `reddit-cli${ext}`);

  console.log(`Downloading reddit-cli v${VERSION} for ${platform}...`);

  try {
    await download(downloadUrl, destPath);

    // Make executable on Unix
    if (process.platform !== 'win32') {
      fs.chmodSync(destPath, 0o755);
    }

    console.log('reddit-cli installed successfully!');
  } catch (err) {
    console.error('Failed to download binary:', err.message);
    console.log('Please build from source: cargo install --path .');
    process.exit(1);
  }
}

install();
