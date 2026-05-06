#!/usr/bin/env bash
# build-resume-pdf.sh — render assets/resume.html to a print-clean PDF.
# The PDF is the same source the site serves; print stylesheet hides
# the topbar, backrefs, pdf-nudge, and cosmic backdrop.
#
# Usage:   bash scripts/build-resume-pdf.sh
# Output:  assets/michael-cochran-resume_may_2026.pdf
#
# After running, rebuild the binary so include_packed picks up the new bytes:
#   cargo build --release -p cochranblock --features approuter

set -euo pipefail

REPO=/home/mcochran/cochranblock
SRC="$REPO/assets/resume.html"
OUT="$REPO/assets/michael-cochran-resume_may_2026.pdf"
TMP=/tmp/resume-for-pdf.html

# Rewrite root-relative href / src to absolute https://cochranblock.org/...
# so the rendered PDF has working hyperlinks even when opened standalone.
sed -E '
  s|href="/([^/])|href="https://cochranblock.org/\1|g
  s|href="/"|href="https://cochranblock.org/"|g
  s|src="/([^/])|src="https://cochranblock.org/\1|g
' "$SRC" > "$TMP"

/usr/bin/chromium \
  --headless=new \
  --no-sandbox \
  --disable-gpu \
  --hide-scrollbars \
  --no-pdf-header-footer \
  --print-to-pdf="$OUT" \
  --virtual-time-budget=4500 \
  "file://$TMP"

ls -lh "$OUT"
echo "PDF written. Rebuild binary: cargo build --release -p cochranblock --features approuter"
