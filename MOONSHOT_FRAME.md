# P26: Moonshot Frame

> **"If this code were the foundation of a civilization-scale outcome, would it still hold up?"**

One question. Ask it every time before you merge, push, or hit post. If the answer is "yeah kinda" or "I'll fix it later," you already failed, go fix it now before you forget.

This doc is the per-repo template. Copy it to your repo root, tick some boxes, ship. That's it, that's the whole protocol. Unlicensed, forkable, strip my name off it, rebrand it, sell it, I don't care, that's the point.

## Pick your moonshot

What's the worst-case scenario this code has to survive? Pick one, write it at the top of your commit, and judge every line against it:

- [ ] My code is running on a spacecraft two AU from earth and the service tech's name is "nobody"
- [ ] A DCAA auditor is reading this in a beige room at 4:50 PM before their Friday happy hour
- [ ] A hostile evaluation board is grading my proposal at the same time they're watching college football
- [ ] A forensic investigator is reading my commit messages in 2036 trying to figure out what I meant
- [ ] The Internet Archive is hosting this LinkedIn post for the next 80 years and historians read it
- [ ] A thousand strangers forked my repo and one of them is not a nice person
- [ ] A regulator is dragging me in front of a judge to prove I did what I said I did
- [ ] (your moonshot here) __________________

That scenario IS the frame. If your code would make you look like an idiot in it, fix the code.

## The Five Forces (this is where you actually work)

Walk every non-trivial change through these five. If you can't tick all five cleanly, the change isn't done yet.

### 1. Typed where possible (because I will find you)

No string-splitting structured data. If the data has a shape, give it a struct. If JSON, use `serde`. If binary, use a typed parser. If you're about to `body.split(":\"")` to extract something, stop, breathe, give it a struct. 🫠

- [ ] No split-on-delimiter parsing on structured inputs (JSON, TOML, CCSDS, safetensors)
- [ ] `serde::Deserialize` on everything that crosses a wire or a disk
- [ ] No magic byte offsets without a struct and a comment explaining the byte offsets
- [ ] No stringly-typed state machines where an enum would do

### 2. Bounded where unbounded (loops don't love you back)

Every `loop`, every `while`, every `for`, every pagination call, every recursion, every retry — they all need a cliff. Name the cliff. Write it down. I promise future you will love past you for this, and future auditor will love both of you.

- [ ] Every loop has an exit condition you can actually hit
- [ ] Pagination has a ceiling (document the ceiling, don't just set it and hope)
- [ ] Retries use exponential backoff with a total-attempt limit
- [ ] Resource allocations (bytes, file handles, connections, sockets) have a ceiling

### 3. Observable where silent (errors are not vampires, you don't need to hide them)

`let _ = ...` on a critical path is a crime. `Vec::new()` on HTTP failure is a crime with extra steps. If something can fail, make the failure LOUD. Send it to stderr, to tracing, to a file, to the hash chain, to a cronjob that pages you at 3 AM if you want, just DON'T eat it silently and return zero like nothing happened.

- [ ] No `let _ = some_that_returns_result()` on anything that matters
- [ ] No `unwrap_or_default()` or `Vec::new()` hiding HTTP/parse failures
- [ ] Rate limits surface as a distinct error (not a generic 403)
- [ ] Logs go somewhere. If you can't find them, they didn't happen.

### 4. Explainable where obvious (comment the WHY, never the WHAT)

Don't comment `// increment i` above `i += 1` like some junior-year CS assignment. Comment the WHY above non-obvious decisions. If future-you reading the code in 8 months would ask "why is this here," past-you should've left a `// why:` line.

- [ ] Non-obvious decisions have a `// why:` comment
- [ ] Public APIs have a docstring with at least one example
- [ ] Named constants replace magic numbers (`const CEILING: u32 = 20;` not `20`)
- [ ] Functions too long to keep in your head have a 2-line summary at the top

### 5. Reviewer-friendly where shortcut-friendly (write for the auditor not the IDE)

Somebody who has never seen your repo should be able to clone it, read it top to bottom, and be productive inside of 10 minutes. That's not aspirational, that's the bar. File layout tells a story. Tests prove the story.

- [ ] File layout reads like a story: entry → model → ops → errors → tests
- [ ] Every exported surface has at least one test
- [ ] `cargo doc --open` produces something readable, not a wall
- [ ] Stranger-clones-and-runs-in-10-minutes is the acceptance test

## When to SKIP this whole thing

This protocol is not your Sunday-morning crossword. Skip it freely when:

- You're writing a 3-line bash script to rename some files
- You're scaffolding code you guarantee will be replaced next commit
- You're in pre-MVP exploration trying to figure out if the idea even works
- The change is literally a typo fix in a README

If you're using P26 to justify perfectionism paralysis, you're using it wrong. Ship scrappy first, apply P26 before anything goes into the repo's "this is the real version" branch. Scrappy → Moonshot → Done.

## Commit message convention

Tag commits with `[P26]` when you formally applied the Five Forces. Not every commit. Just the ones where you did the work.

Example:
```
[P26] Replace string-split JSON parser with serde structs

The previous parser split the response on `"date":"` and pulled
timestamps via `find('"')`. That worked until it didn't, and when it
didn't, it silently returned `Vec::new()` and nobody knew.

Five Forces applied:
  1. Typed:      GhCommitEnvelope / GhCommit / GhSignature
  2. Bounded:    20-page pagination cap (safety cliff)
  3. Observable: FetchError enum, eprintln on failure
  4. Explainable: why-comment on why we use committer.date not author.date
  5. Reviewer-friendly: fetch_commit_timestamps() as its own function
```

## The Shield

If your repo passes all five forces on the code that matters, put the badge in your README right next to your license:

```markdown
[![Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](https://unlicense.org/)
[![P26 Moonshot Frame](https://img.shields.io/badge/P26-Moonshot%20Frame-black.svg)](https://cochranblock.org/protocols#p26)
```

Side by side those two shields say: *"This code is public domain AND it survives adult scrutiny."* That's your whole brand in two PNGs.

## Last thing

This file is itself P26-framed. It will be forked, stripped, rebranded, re-published on someone else's blog with "written by" replaced, slapped onto a Gumroad, and sold as a $49 ebook by somebody who stumbles into it on Hacker News in 2028. That's fine. That's the whole point. If somebody's reading a copy of this doc with my name scrubbed off, it's still doing the job. Spread it. Red crayons forever.

---

**Canonical source:** cochranblock.org/protocols#p26 (redirects to /arch#p26)
**Template source:** cochranblock.org/MOONSHOT_FRAME.md
**Originated:** The Cochran Block, LLC · Dundalk, MD · April 2026
**License:** Unlicense. If you're reading this with someone else's attribution on it, that's not theft, that's the protocol working. 💪

#UnlicenseBaby #P26 #MoonshotFrame
