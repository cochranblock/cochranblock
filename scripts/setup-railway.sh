#!/bin/bash
# One-time setup for Railway deployment.
# Run from approuter root.
set -e
cd "$(cd "$(dirname "$0")/.." && pwd)"

echo "=== approuter Railway setup ==="

# 1. Create GitHub repo (requires gh CLI or manual)
if command -v gh &>/dev/null; then
  echo "Creating cochranblock/approuter..."
  gh repo create cochranblock/approuter --public --source=. --remote=origin --push 2>/dev/null || \
    echo "Repo may exist. Adding remote..."
  git remote add origin git@github.com:cochranblock/approuter.git 2>/dev/null || true
else
  echo "Install gh (brew install gh) to create repo, or create cochranblock/approuter manually on GitHub."
  echo "Then: git remote add origin git@github.com:cochranblock/approuter.git"
fi

# 2. Create cochranblock/rogue-repo (for standalone rogue-repo pushes)
if command -v gh &>/dev/null; then
  echo "Creating cochranblock/rogue-repo..."
  gh repo create cochranblock/rogue-repo --public 2>/dev/null || echo "rogue-repo may exist."
fi

echo ""
echo "Next steps:"
echo "1. git add -A && git commit -m 'Initial monorepo' && git push -u origin master"
echo "2. Railway: New Project -> Deploy from GitHub -> cochranblock/approuter"
echo "3. Add services with Root Directory: approuter, cochranblock, oakilydokily, rogue-repo"
echo "4. Add Postgres, link to rogue-repo. Set env vars per approuter/docs/RAILWAY.md"
