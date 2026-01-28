#!/bin/bash
set -e

echo "Installing Bun..."
npm install -g bun

echo "Bun version:"
bun --version

bun install

echo "Dev container setup complete!"
