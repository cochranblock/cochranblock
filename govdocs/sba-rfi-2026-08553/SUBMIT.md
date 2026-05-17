# Submission Instructions

## Email Routing

| Field | Value |
|---|---|
| **To** | technology@sba.gov |
| **Subject** | RFI Response: Innovation Networks and Supply Chains |
| **Attach** | `COVER_SHEET.pdf` and `RFI_RESPONSE.pdf` (rendered from the markdown in this directory) |
| **Deadline** | May 18, 2026 (no time-of-day specified in the RFI; submit by close of business Eastern to be safe) |

## Email Body (Suggested)

```
To: technology@sba.gov
Subject: RFI Response: Innovation Networks and Supply Chains

Office of Investment and Innovation
U.S. Small Business Administration
Attn: Alison Evans, Program Analyst

Please find attached the response of Cochran Block to the SBA Request
for Information on Supply Chain Gaps and Entrepreneur Assistance,
Federal Register Document 2026-08553 (91 FR 23522).

Two attachments:
  1. COVER_SHEET.pdf, the required cover sheet (does not count toward the 5-page limit).
  2. RFI_RESPONSE.pdf, the body of the response, within the 5-page limit.

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

- [ ] Render `COVER_SHEET.md` to PDF and verify the cover sheet contains the three required fields plus the additional metadata block.
- [ ] Render `RFI_RESPONSE.md` to PDF and verify it does not exceed 5 pages at standard 12pt single-spaced formatting with 1-inch margins.
- [ ] Verify UEI, CAGE, and EIN are still current.
- [ ] Verify the public GitHub organization https://github.com/cochranblock resolves and shows the cited repositories.
- [ ] Verify the live demo at https://cochranblock.org is up.
- [ ] Send from mcochran@cochranblock.org so the reply-to is the cited contact.
- [ ] Save sent copy plus delivery confirmation to `submission_record/` for audit trail.

## Render-to-PDF Commands

If pandoc is on the path:

```
pandoc COVER_SHEET.md   -o COVER_SHEET.pdf   --pdf-engine=xelatex -V geometry:margin=1in -V mainfont="DejaVu Serif" -V fontsize=12pt
pandoc RFI_RESPONSE.md  -o RFI_RESPONSE.pdf  --pdf-engine=xelatex -V geometry:margin=1in -V mainfont="DejaVu Serif" -V fontsize=12pt
```

Confirm `pdfinfo RFI_RESPONSE.pdf | grep Pages` reports 5 or fewer.
