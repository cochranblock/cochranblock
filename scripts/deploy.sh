#!/usr/bin/env bash
# Unlicense — cochranblock.org
# Full deploy pipeline: build on lf, hot-reload to gd, purge cache, ping search engines.

set -euo pipefail

echo "[1/5] Pull + build on lf..."
ssh lf "cd /home/mcochran/cochranblock && git pull && source ~/.cargo/env && cargo build --release --features approuter 2>&1 | tail -2"

echo "[2/5] Copy binary lf → gd (staging)..."
scp lf:/home/mcochran/cochranblock/target/release/cochranblock /tmp/cochranblock-x86
scp /tmp/cochranblock-x86 gd:/tmp/cochranblock-new
ssh gd "chmod +x /tmp/cochranblock-new && mv -f /tmp/cochranblock-new /home/mcochran/cochranblock/target/release/cochranblock"

echo "[3/5] Hot reload on gd (new binary kills old via PID lockfile)..."
ssh gd "export \$(cat /home/mcochran/cochranblock-monorepo/approuter/.env | grep -v '^#' | xargs) && cd /home/mcochran/cochranblock && nohup target/release/cochranblock > /tmp/cochranblock.log 2>&1 &"
sleep 4

echo "[4/5] Purge Cloudflare cache..."
ssh gd "export \$(cat /home/mcochran/cochranblock-monorepo/approuter/.env | grep -v '^#' | xargs) && /home/mcochran/cochranblock-monorepo/target/release/approuter purge-cache 2>&1"

echo "[5/5] Ping IndexNow..."
curl -s -X POST "https://api.indexnow.org/IndexNow" \
  -H "Content-Type: application/json" \
  -d '{"host":"cochranblock.org","key":"cochranblock-indexnow-key","keyLocation":"https://cochranblock.org/cochranblock-indexnow-key.txt","urlList":["https://cochranblock.org/","https://cochranblock.org/services","https://cochranblock.org/deploy","https://cochranblock.org/book","https://cochranblock.org/stats","https://cochranblock.org/codeskillz","https://cochranblock.org/products","https://cochranblock.org/about","https://cochranblock.org/contact","https://cochranblock.org/sbir","https://cochranblock.org/downloads","https://cochranblock.org/community-grant","https://cochranblock.org/openbooks","https://cochranblock.org/analytics","https://cochranblock.org/tinybinaries","https://cochranblock.org/govdocs","https://cochranblock.org/vre","https://cochranblock.org/source","https://cochranblock.org/search"]}' \
  -o /dev/null -w "  IndexNow: %{http_code}\n"

# Verify
STATUS=$(ssh gd "curl -s -o /dev/null -w '%{http_code}' http://127.0.0.1:8081/")
echo ""
echo "Done. Status: $STATUS"
echo "Binary: $(ssh lf 'ls -lh /home/mcochran/cochranblock/target/release/cochranblock' | awk '{print $5}')"
