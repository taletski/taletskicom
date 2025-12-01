#!/bin/bash
set -e
echo "🚀 Setting up the dev environment..."

cargo install --locked prek # Install prek for pre-commit hooks
prek install --hook-type pre-commit --hook-type commit-msg # Install git hooks

echo "✅ All set up, happy coding!"
