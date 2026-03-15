#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');

const binaryName = process.platform === 'win32' ? 'reddit-cli.exe' : 'reddit-cli';
const binaryPath = path.join(__dirname, '..', 'binaries', binaryName);

const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  env: process.env
});

child.on('exit', (code) => {
  process.exit(code || 0);
});

child.on('error', (err) => {
  console.error('Failed to run reddit-cli:', err.message);
  process.exit(1);
});
