#!/usr/bin/env bash
# Unlicense — cochranblock.org
# Full deploy pipeline: build on lf, push to gd, restart, purge cache, ping search engines.

set -euo pipefail

echo "[1/6] Pull + build on lf..."
ssh lf "cd /home/mcochran/cochranblock && git pull && source ~/.cargo/env && cargo build --release --features approuter 2>&1 | tail -2"

echo "[2/6] Kill old binary on gd..."
ssh gd "kill \$(pgrep -f 'target/release/cochranblock$') 2>/dev/null || true"
sleep 2

echo "[3/6] Copy binary lf → gd..."
scp lf:/home/mcochran/cochranblock/target/release/cochranblock /tmp/cochranblock-x86
scp /tmp/cochranblock-x86 gd:/home/mcochran/cochranblock/target/release/cochranblock
ssh gd "chmod +x /home/mcochran/cochranblock/target/release/cochranblock"

echo "[4/6] Start new binary on gd..."
ssh gd "cd /home/mcochran/cochranblock && nohup target/release/cochranblock > /tmp/cochranblock.log 2>&1 &"
sleep 4

echo "[5/6] Purge Cloudflare cache..."
ssh gd "export \$(cat /home/mcochran/cochranblock-monorepo/approuter/.env | grep -v '^#' | xargs) && /home/mcochran/cochranblock-monorepo/target/release/approuter purge-cache 2>&1"

echo "[6/6] Ping IndexNow..."
curl -s -X POST "https://api.indexnow.org/IndexNow" \
  -H "Content-Type: application/json" \
  -d '{"host":"cochranblock.org","key":"cochranblock-indexnow-key","keyLocation":"https://cochranblock.org/cochranblock-indexnow-key.txt","urlList":["https://cochranblock.org/","https://cochranblock.org/services","https://cochranblock.org/deploy","https://cochranblock.org/book","https://cochranblock.org/mathskillz","https://cochranblock.org/codeskillz","https://cochranblock.org/products","https://cochranblock.org/about","https://cochranblock.org/contact","https://cochranblock.org/sbir","https://cochranblock.org/downloads","https://cochranblock.org/community-grant"]}' \
  -o /dev/null -w "  IndexNow: %{http_code}\n"

# Verify
STATUS=$(ssh gd "curl -s -o /dev/null -w '%{http_code}' http://127.0.0.1:8081/")
echo ""
echo "Done. Status: $STATUS"
echo "Binary: $(ssh lf 'ls -lh /home/mcochran/cochranblock/target/release/cochranblock' | awk '{print $5}')"
