#!/usr/bin/env python3
"""
Drive Gemini image generation via CDP.
Navigates to gemini.google.com, submits the prompt, waits for images,
and downloads them to ~/cochranblock/assets/panther-defense/
"""

import json, time, sys, os, re, urllib.request
import requests, websocket

CDP_BASE = "http://localhost:9222"
OUT = os.path.expanduser("~/cochranblock/assets/panther-defense/")

PROMPT = """\
8-bit pixel art game background sprite sheet, 4 separate parallax layers \
stacked vertically on a single tall canvas, transparent checkerboard background \
behind all non-solid pixels.

Scene: tropical jungle clearing at night, C5ISR military defense theme.

LAYER 1 — Sky: deep navy night sky, pixel stars, crescent moon top-left, \
silhouettes of satellite dish arrays and a comms tower on the horizon, \
faint green radar sweep glow.

LAYER 2 — Far jungle: dark jungle canopy, tall ISR surveillance tower above \
treeline on the left with a blinking red pixel at the top, distant ridge.

LAYER 3 — Mid jungle: closer jungle treeline, hanging vines, ruined concrete \
wall with mounted antenna, dense undergrowth, dark teal and olive palette.

LAYER 4 — Foreground/ground: flat dirt clearing (the ground the panther runs \
across), on the RIGHT SIDE a large mossy boulder formation approximately \
2x character-height tall with a flat top — suitable for a panther to jump and \
land on. Sparse grass tufts. Dark foreground plants framing left edge.

Style: crisp 8-bit pixel art, no anti-aliasing, limited palette per layer \
(8-16 colors max), black outlines, game-ready sprite sheet format, \
each layer strip 1408px wide x 200px tall, transparent background.\
"""


def get_target():
    tabs = requests.get(f"{CDP_BASE}/json").json()
    for t in tabs:
        if t.get("type") == "page":
            return t["webSocketDebuggerUrl"]
    raise RuntimeError("No page target found")


class CDP:
    def __init__(self, url):
        self.ws = websocket.create_connection(url, timeout=30)
        self._id = 0

    def send(self, method, params=None):
        self._id += 1
        msg = {"id": self._id, "method": method, "params": params or {}}
        self.ws.send(json.dumps(msg))
        return self._id

    def recv_until(self, target_id, timeout=60):
        deadline = time.time() + timeout
        while time.time() < deadline:
            raw = self.ws.recv()
            msg = json.loads(raw)
            if msg.get("id") == target_id:
                return msg
        raise TimeoutError(f"Timed out waiting for id={target_id}")

    def eval(self, expr, timeout=30):
        mid = self.send("Runtime.evaluate", {
            "expression": expr,
            "awaitPromise": True,
            "returnByValue": True,
        })
        return self.recv_until(mid, timeout)

    def navigate(self, url):
        mid = self.send("Page.navigate", {"url": url})
        self.recv_until(mid, timeout=15)
        time.sleep(3)

    def close(self):
        self.ws.close()


def wait_for_selector(cdp, selector, timeout=30):
    deadline = time.time() + timeout
    while time.time() < deadline:
        r = cdp.eval(f"!!document.querySelector({json.dumps(selector)})")
        if r.get("result", {}).get("result", {}).get("value"):
            return True
        time.sleep(1)
    return False


def main():
    os.makedirs(OUT, exist_ok=True)
    ws_url = get_target()
    print(f"Connecting to: {ws_url}")
    cdp = CDP(ws_url)

    # Navigate to Gemini
    print("Navigating to gemini.google.com ...")
    cdp.navigate("https://gemini.google.com/app")
    time.sleep(4)

    # Find the input box and type the prompt
    print("Looking for input field ...")
    # Gemini uses a rich text input
    if not wait_for_selector(cdp, "rich-textarea, [contenteditable='true'], textarea", timeout=20):
        print("ERROR: Could not find input field — may need to log in first")
        cdp.close()
        sys.exit(1)

    # Click and type
    cdp.eval("""
        (function() {
            const el = document.querySelector('rich-textarea [contenteditable]') ||
                       document.querySelector('[contenteditable="true"]') ||
                       document.querySelector('textarea');
            if (!el) return 'not found';
            el.focus();
            return 'focused';
        })()
    """)
    time.sleep(0.5)

    # Type via clipboard (avoids issues with special chars)
    prompt_escaped = PROMPT.replace("\\", "\\\\").replace("`", "\\`").replace("$", "\\$")
    cdp.eval(f"""
        (async function() {{
            const text = `{prompt_escaped}`;
            await navigator.clipboard.writeText(text);
        }})()
    """)
    time.sleep(0.3)
    cdp.eval("document.execCommand('paste')")
    time.sleep(1)

    # Submit
    print("Submitting prompt ...")
    cdp.eval("""
        (function() {
            const btn = document.querySelector('button[aria-label*="Send"], button[data-test-id="send-button"], button[jsname]');
            if (btn) { btn.click(); return 'clicked'; }
            // Fallback: Enter key
            const el = document.querySelector('[contenteditable="true"]');
            if (el) {
                el.dispatchEvent(new KeyboardEvent('keydown', {key: 'Enter', bubbles: true}));
                return 'enter';
            }
            return 'no button found';
        })()
    """)

    # Wait for images to appear (Gemini generates 4 images)
    print("Waiting for images to generate (up to 90s) ...")
    deadline = time.time() + 90
    img_srcs = []
    while time.time() < deadline and len(img_srcs) < 1:
        time.sleep(3)
        r = cdp.eval("""
            (function() {
                const imgs = Array.from(document.querySelectorAll('img')).filter(i =>
                    i.src && i.src.includes('gemini') ||
                    i.src && i.src.includes('blob:') ||
                    (i.naturalWidth > 200 && i.naturalHeight > 200 &&
                     !i.src.includes('icon') && !i.src.includes('avatar'))
                );
                return imgs.map(i => ({src: i.src, w: i.naturalWidth, h: i.naturalHeight}));
            })()
        """)
        imgs = r.get("result", {}).get("result", {}).get("value", [])
        img_srcs = [i["src"] for i in imgs if i.get("w", 0) > 200]
        if img_srcs:
            print(f"Found {len(img_srcs)} images")

    if not img_srcs:
        print("No images found — check if Gemini is logged in and generated successfully")
        cdp.close()
        sys.exit(1)

    # Download images
    for i, src in enumerate(img_srcs[:4]):
        fname = os.path.join(OUT, f"Gemini_bg_layer_{i}.png")
        print(f"Downloading image {i+1} → {fname}")
        try:
            if src.startswith("blob:"):
                # Fetch blob via CDP
                r = cdp.eval(f"""
                    (async function() {{
                        const res = await fetch({json.dumps(src)});
                        const buf = await res.arrayBuffer();
                        const b64 = btoa(String.fromCharCode(...new Uint8Array(buf)));
                        return b64;
                    }})()
                """, timeout=15)
                b64 = r.get("result", {}).get("result", {}).get("value", "")
                if b64:
                    import base64
                    with open(fname, "wb") as f:
                        f.write(base64.b64decode(b64))
                    print(f"  saved ({os.path.getsize(fname)} bytes)")
            else:
                urllib.request.urlretrieve(src, fname)
                print(f"  saved ({os.path.getsize(fname)} bytes)")
        except Exception as e:
            print(f"  failed: {e}")

    cdp.close()
    print("Done.")


if __name__ == "__main__":
    main()
