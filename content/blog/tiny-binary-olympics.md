# The Tiny Binary Olympics: 16 Rust Projects from 48 KB to 51 MB — All Public Domain

*Draft for dev.to — review before posting*

---

I built 16 Rust projects in 2 months. Every one compiles to a single binary. The smallest is 48 KB. The largest — an augment engine with local LLM inference — is 51 MB. All are Unlicense (public domain). Here's what I learned about making binaries tiny.

## The Leaderboard

| # | Project | Binary Size | What It Does |
|---|---------|-------------|--------------|
| 1 | call-shield | 48 KB | On-device call screening with Whisper |
| 2 | aptnomo | 312 KB | Autonomous APT threat hunter |
| 3 | exopack | 313 KB | Test framework, zero external deps |
| 4 | provenance-docs | 328 KB | AI development documentation |
| 5 | whyyoulying | 505 KB | DoD fraud detection |
| 6 | pocket-server | 1.01 MB | Your website on your phone |
| 7 | illbethejudgeofthat | 2.5 MB | Legal case builder |
| 8 | ronin-sites | 4.0 MB | Multi-tenant shop platform |
| 9 | cochranblock | 8.4 MB | The site you're about to visit |
| 10 | pixel-forge | 9.2 MB | AI sprite generator with 3 diffusion models |
| 11 | kova | ~51.5 MB | Augment engine + local LLM |

All ARM aarch64. Release profile: `opt-level='s'`, LTO, `codegen-units=1`, `panic='abort'`, `strip=true`.

## The Build Profile That Does It

```toml
[profile.release]
opt-level = 's'    # optimize for size, keep performance
lto = true         # link-time optimization — whole program
codegen-units = 1  # single codegen unit — best optimization
panic = 'abort'    # no unwinding overhead
strip = true       # remove debug symbols
```

Switching from `opt-level = 3` to `'s'` dropped cochranblock from 18 MB to 15 MB (x86) — a 17% reduction from one character change.

## The 312 KB Threat Hunter

aptnomo is a 312 KB autonomous APT threat hunter. It scans for persistence mechanisms, rootkits, suspicious processes, network anomalies, and log tampering. It auto-kills cryptominers and reverse shells. Zero config — drop it on a machine and forget it.

312 KB. Smaller than a JPEG. Does what enterprise EDR charges $50/seat/month for.

## The 48 KB Call Screener

call-shield screens phone calls using on-device Whisper speech-to-text and an intent classifier. Zero audio leaves the device. 48 KB binary. No cloud. No API key. No subscription.

For context: a Docker hello-world image is 13 MB. Our call screener is 270x smaller.

## The AI Model That Ships on a Phone

pixel-forge runs three on-device diffusion models (Cinder 4.2 MB / Quench 22 MB / Anvil 64 MB) that generate, judge, and arrange pixel art sprites. The models were trained from scratch in Rust on consumer GPUs (RTX 3070, RTX 3050 Ti). Not downloaded from HuggingFace. The TinyUNet, training loop, and sampling math are ours.

The entire app runs offline in 10 MB. No API key. No internet required.

## The Speed Comparison Nobody Asked For

cochranblock.org homepage: 9.5 KB, 0 JavaScript.

| Site | Page Size | Scripts |
|------|-----------|---------|
| cochranblock.org | 9.5 KB | 0 |
| Vercel.com | 956 KB (94x) | 116 |
| Stripe.com | 574 KB (56x) | 1 |
| Wix.com | 2,287 KB (240x) | 69 |

The company that sells web hosting ships 116 JavaScript files. We ship zero. Draw your own conclusions.

## The Trifecta

1. **You can't outprice free** — every line of code is Unlicense, public domain
2. **You can't out-transparent public domain** — the source, the bugs, the IR&D audit — all public
3. **Expertise is inherent** — building it in public proves the capability. The code IS the resume.

All 16 repos: [github.com/cochranblock](https://github.com/cochranblock)
The live demo: [cochranblock.org](https://cochranblock.org)
Binary leaderboard: [cochranblock.org/tinybinaries](https://cochranblock.org/tinybinaries)
Speed proof: [cochranblock.org/speed](https://cochranblock.org/speed)

---

*Michael Cochran — Army 17C (Cyber Operations), 13 years defense and enterprise. Building zero-cloud architecture for federal agencies. SDVOSB pending.*
