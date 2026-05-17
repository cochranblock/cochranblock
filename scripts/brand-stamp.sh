#!/usr/bin/env bash
# Unlicense - public domain - cochranblock.org
# Stamp Cochran Block brand banner + header + footer onto repo READMEs.
# Idempotent: re-running replaces existing brand blocks via marker comments.
# Usage:
#   scripts/brand-stamp.sh                   # all repos in repos.tsv
#   scripts/brand-stamp.sh kova              # only one repo
#   scripts/brand-stamp.sh --no-inner kova   # README + banner only, skip inner .md footers
set -euo pipefail

WORKSPACE="${WORKSPACE:-/home/mcochran}"
BRAND_DIR="${WORKSPACE}/cochranblock/assets/brand"
TEMPLATE="${BRAND_DIR}/banner-template.svg"
HEADER_SNIPPET="${BRAND_DIR}/header-snippet.md"
FOOTER_SNIPPET="${BRAND_DIR}/footer-snippet.md"
FOOTER_LIGHT="${BRAND_DIR}/footer-snippet-light.md"
MANIFEST="${BRAND_DIR}/repos.tsv"

NO_INNER=0
ONLY=""
for arg in "$@"; do
    case "$arg" in
        --no-inner) NO_INNER=1 ;;
        *) ONLY="$arg" ;;
    esac
done

ensure_trailing_newline() {
    local f="$1"
    [ -s "$f" ] || { echo "" > "$f"; return; }
    if [ "$(tail -c 1 "$f" | od -An -c | tr -d ' ')" != "\\n" ]; then
        printf '\n' >> "$f"
    fi
}

stamp_footer_light() {
    local md="$1"
    [ -f "$md" ] || return 0
    sed -i '/COCHRANBLOCK-BRAND-FOOTER:START/,/COCHRANBLOCK-BRAND-FOOTER:END/d' "$md"
    ensure_trailing_newline "$md"
    cat "$FOOTER_LIGHT" >> "$md"
}

while IFS=$'\t' read -r repo title subtitle tag1 tag2 tag3 title_size; do
    [ "${repo:-}" = "repo" ] && continue
    [ -z "${repo:-}" ] && continue
    [ -n "$ONLY" ] && [ "$ONLY" != "$repo" ] && continue
    repo_dir="${WORKSPACE}/${repo}"
    if [ ! -d "$repo_dir" ]; then
        echo "[skip] ${repo} - not found at ${repo_dir}"
        continue
    fi
    echo "[stamp] ${repo}"

    mkdir -p "${repo_dir}/assets/brand"
    sed \
        -e "s|__TITLE__|${title}|g" \
        -e "s|__SUBTITLE__|${subtitle}|g" \
        -e "s|__TAG1__|${tag1}|g" \
        -e "s|__TAG2__|${tag2}|g" \
        -e "s|__TAG3__|${tag3}|g" \
        -e "s|__TITLE_SIZE__|${title_size}|g" \
        "$TEMPLATE" > "${repo_dir}/assets/brand/banner.svg"

    # Air-gap safe: ship the 4 brand badges in-repo so the README renders
    # without an outbound HTTP call to img.shields.io.
    mkdir -p "${repo_dir}/assets/brand/badges"
    cp -n "${BRAND_DIR}/badges/"*.svg "${repo_dir}/assets/brand/badges/" 2>/dev/null || true

    readme="${repo_dir}/README.md"
    if [ ! -f "$readme" ]; then
        printf '# %s\n\n%s\n' "$title" "$subtitle" > "$readme"
    fi

    sed -i '/COCHRANBLOCK-BRAND-HEADER:START/,/COCHRANBLOCK-BRAND-HEADER:END/d' "$readme"
    sed -i '/COCHRANBLOCK-BRAND-FOOTER:START/,/COCHRANBLOCK-BRAND-FOOTER:END/d' "$readme"

    header_tmp=$(mktemp)
    sed \
        -e "s|__BANNER_PATH__|assets/brand/banner.svg|g" \
        -e "s|__TITLE__|${title}|g" \
        -e "s|__TAG1__|${tag1}|g" \
        -e "s|__TAG2__|${tag2}|g" \
        -e "s|__TAG3__|${tag3}|g" \
        "$HEADER_SNIPPET" > "$header_tmp"

    combined=$(mktemp)
    cat "$header_tmp" > "$combined"
    printf '\n' >> "$combined"
    cat "$readme" >> "$combined"
    ensure_trailing_newline "$combined"
    cat "$FOOTER_SNIPPET" >> "$combined"
    mv "$combined" "$readme"
    rm -f "$header_tmp"

    if [ "$NO_INNER" -eq 0 ]; then
        for md in "${repo_dir}"/*.md; do
            [ -f "$md" ] || continue
            bname=$(basename "$md")
            case "$bname" in
                README.md|CLAUDE.md|AGENTS.md|LICENSE.md|UNLICENSE.md|COPYING.md) continue ;;
            esac
            stamp_footer_light "$md"
        done
        if [ -d "${repo_dir}/docs" ]; then
            while IFS= read -r md; do
                stamp_footer_light "$md"
            done < <(find "${repo_dir}/docs" -maxdepth 3 -name "*.md" -type f \
                -not -path "*/target/*" -not -path "*/node_modules/*")
        fi
    fi
done < "$MANIFEST"

echo "[done] brand-stamp.sh"
