# Intake & Community Grant UX Analysis

**Source:** triple_sims screenshot capture (cochranblock-test)  
**Date:** 2026-03

## Scope

Intake (`/intake`) and Community Grant (`/community-grant`) forms. Waiver-style UI, cosmic theme.

## Findings (from screenshot review)

### Strengths
- Clear progress indicator (1 Review, 2 Submit, 3 Complete)
- Consistent dark theme with teal accents
- Card-based layout with gradient and shadow
- Full-width submit button
- Checkbox consent flow

### Gaps
1. **Intro text** — Community grant intro blends into body; needs callout treatment (like contact testimonial)
2. **Form density** — Fields feel stacked without visual grouping; org/contact vs mission/technical could be separated
3. **Scroll hint** — Intake "Scroll through the entire document" is easy to miss
4. **Section labels** — "Submit your request" / "Submit your application" could use a subtle accent (left border) for hierarchy
5. **Required vs optional** — Optional fields marked but required not explicitly called out

## Recommendations Applied

- `.intake-intro` callout styling for community grant (left border, light background)
- `.intake-form-group` for field grouping with margin
- `.intake-hint` stronger treatment (small callout above terms)
- `.intake-sign` left border accent for section hierarchy
- Optional labels already present; keep as-is
