#!/usr/bin/env bash
# screenshots.sh — capture multi-viewport screenshots of the running cochranblock
# binary at phone-portrait, phone-landscape, tablet-portrait, tablet-landscape,
# and desktop sizes. Output: screenshots/<page>-<viewport>.png
#
# Usage:   bash scripts/screenshots.sh [base_url]
#          base_url defaults to http://192.168.1.52:8081
#
# Requires the binary to be running on the target host first.

set -euo pipefail

BASE="${1:-http://192.168.1.52:8081}"
REPO=/home/mcochran/cochranblock
OUT="$REPO/screenshots"
mkdir -p "$OUT"

# (label, css_width x css_height) — devicePixelRatio kept at 1 for now.
VIEWPORTS=(
  "phone-portrait    390x844"
  "phone-landscape   844x390"
  "tablet-portrait   768x1024"
  "tablet-landscape  1024x768"
  "desktop           1440x900"
)

# Pages to capture
PAGES=(
  "lets-team   /"
  "resume      /resume"
  "manual      /manual"
  "govdocs     /govdocs"
  "services    /services"
)

for vp in "${VIEWPORTS[@]}"; do
  read -r vp_name dim <<< "$vp"
  W="${dim%x*}"
  H="${dim#*x}"
  for pg in "${PAGES[@]}"; do
    read -r pg_name path <<< "$pg"
    out="$OUT/${pg_name}-${vp_name}.png"
    /usr/bin/chromium \
      --headless=new \
      --no-sandbox \
      --disable-gpu \
      --hide-scrollbars \
      --window-size="${W},${H}" \
      --virtual-time-budget=3000 \
      --screenshot="$out" \
      "${BASE}${path}" >/dev/null 2>&1
    if [[ -f "$out" ]]; then
      sz=$(stat -c%s "$out")
      printf "  %-30s %dx%d  %s bytes\n" "${pg_name}-${vp_name}" "$W" "$H" "$sz"
    else
      printf "  %-30s FAILED\n" "${pg_name}-${vp_name}"
    fi
  done
done

echo
echo "Screenshots in $OUT/"
ls -1 "$OUT" | head -25
