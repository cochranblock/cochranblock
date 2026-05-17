# Provenance

cochranblock uses the [Provenance Docs Framework](https://github.com/cochranblock/provenance-docs) — a two-document system for AI-piloted software development.

## Documents

- **[TIMELINE_OF_INVENTION.md](../TIMELINE_OF_INVENTION.md)** — dated, commit-linked record of what was built, when, and why. Every entry has a mandatory `AI Role` field documenting the human/AI boundary.
- **[PROOF_OF_ARTIFACTS.md](../PROOF_OF_ARTIFACTS.md)** — build metrics, architecture diagrams, screenshots, and verification commands. A reproducibility contract anyone can run.

## Why It Exists

Federal acquisition (DFARS 252.227-7014) requires identification of privately-developed vs government-funded software. USPTO requires human inventors. NIST SP 800-218 requires provenance tracking. None of these frameworks address AI-generated code.

The TOI + POA system creates a chain of custody: human decision → AI execution → human verification → commit.

## Verify

```bash
# The evidence is in the repository
cat TIMELINE_OF_INVENTION.md   # human/AI attribution per entry
cat PROOF_OF_ARTIFACTS.md      # build evidence + verification commands
git log --oneline              # cross-reference TOI commit hashes
```

## Framework Spec

See [provenance-docs](https://github.com/cochranblock/provenance-docs) for the full whitepaper, CDRL mapping, and SBIR phase plan.
