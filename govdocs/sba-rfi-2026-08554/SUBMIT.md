# Submission Instructions

## Email Routing

| Field | Value |
|---|---|
| **To** | technology@sba.gov |
| **Subject** | RFI Response: Strengthening Domestic Supply Chains and Critical Supplier Competition |
| **Attach** | `COVER_SHEET.pdf` and `RFI_RESPONSE.pdf` (rendered from this directory) |
| **Deadline** | May 18, 2026 (no time-of-day specified; submit by close of business Eastern) |

## Email Body (Suggested)

```
To: technology@sba.gov
Subject: RFI Response: Strengthening Domestic Supply Chains and Critical Supplier Competition

Office of Investment and Innovation
U.S. Small Business Administration
Attn: Rikki Jones, Program Analyst

Please find attached the response of Cochran Block to the SBA Request
for Information on Scaling Critical Suppliers in Domestic Supply
Chains, Federal Register Document 2026-08554 (91 FR 23523).

Two attachments:
  1. COVER_SHEET.pdf, the required cover sheet (does not count toward
     the 5-page limit).
  2. RFI_RESPONSE.pdf, the body of the response, within the 5-page
     limit.

A separate response to the companion RFI 2026-08553 (Innovation
Networks and Supply Chains, Alison Evans, OII) is being submitted
under separate cover with the appropriate subject line.

Cochran Block, UEI W7X3HAQL9CF9, CAGE 1CQ66, is a Maryland small
business (MD CSB approved, SDVOSB pending) building public-domain
sovereign software for federal acquisition. The response cites more
than twenty production Unlicense Rust repositories as the evidence
base, all of which are publicly auditable at
https://github.com/cochranblock.

Respectfully,

Michael Cochran
Founder, Cochran Block
mcochran@cochranblock.org
```

## Pre-Send Checklist

- [ ] Render `COVER_SHEET.md` and `RFI_RESPONSE.md` to PDF in light mode.
- [ ] Verify `RFI_RESPONSE.pdf` is at most five pages.
- [ ] Verify the subject line on the email matches the published RFI exactly.
- [ ] Verify UEI, CAGE, and EIN are still current.
- [ ] Save sent copy plus delivery confirmation to `submission_record/`.

## Render-to-PDF (uses the shared renderer)

```
PATH=/home/mcochran/.cargo/bin:$PATH
RENDER=/home/mcochran/cochranblock/govdocs/sba-rfi-2026-08553/render/target/release/rfi-md2html

$RENDER COVER_SHEET.md  /tmp/cover.html
$RENDER RFI_RESPONSE.md /tmp/body.html
chromium --headless --disable-gpu --no-sandbox --no-pdf-header-footer \
  --print-to-pdf=COVER_SHEET.pdf  file:///tmp/cover.html
chromium --headless --disable-gpu --no-sandbox --no-pdf-header-footer \
  --print-to-pdf=RFI_RESPONSE.pdf file:///tmp/body.html
```

For a Cochran-Block-branded dark-mode archival copy:

```
$RENDER --dark COVER_SHEET.md  /tmp/cover-dark.html
$RENDER --dark RFI_RESPONSE.md /tmp/body-dark.html
chromium --headless --disable-gpu --no-sandbox --no-pdf-header-footer \
  --print-to-pdf=COVER_SHEET-dark.pdf  file:///tmp/cover-dark.html
chromium --headless --disable-gpu --no-sandbox --no-pdf-header-footer \
  --print-to-pdf=RFI_RESPONSE-dark.pdf file:///tmp/body-dark.html
```
