# LinkedIn Post: The One-Character Bug

*Draft for LinkedIn — review before posting*

---

We spent 40 hours of GPU time finding a one-character bug.

`rand` vs `randn`. One generates uniform noise. The other generates Gaussian noise. Our diffusion model trained for days producing static because we called the wrong function.

Here's what we learned building AI from scratch — not downloading it:

**The model is ours.** We built a TinyUNet diffusion model in Rust. Not Python. Not PyTorch. Not downloaded from HuggingFace. 1M parameters. Trains on consumer GPUs (RTX 3070, RTX 3050 Ti in laptops running Debian). Total training cost: electricity.

**The bugs are real.** We found three fundamental math errors by reading raw tensor values and tracing gradients:
- Gaussian noise scaling (rand vs randn)
- Epsilon prediction vs x0 prediction mismatch
- DDIM sampling step size calculation

No amount of compute fixes a math bug. You have to understand the math.

**It ships on a phone.** The model runs offline in a 10 MB app. No API key. No cloud. No subscription. No internet required. Try that with a HuggingFace download.

**Everything is public domain.** 16 Rust repos, all Unlicense. Every binary compiles from source. Every claim is verifiable. The website itself (cochranblock.org) runs as a single 8.4 MB binary on a laptop for $10/month.

Most AI companies download a model and wrap an API around it. We train our own models on our own hardware, debug the math ourselves, and ship binaries that work without internet.

The difference is ownership — we own every layer of the stack, and we release it all into the public domain.

cochranblock.org | github.com/cochranblock

#Rust #AI #MachineLearning #Veterans #GovCon #Cybersecurity #SDVOSB #OpenSource #ZeroCloud

---

*Michael Cochran — Fractional CTO, Zero-Cloud Architect. Army 17C (Cyber Operations). 13 years defense and enterprise. Building things that work without the cloud.*
