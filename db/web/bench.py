#!/usr/bin/env python3
"""How long the explorer blocks the browser: long tasks (main-thread blocks over 50 ms) per interaction, in
headless Chromium over its debugging protocol. Serves crates/app/dist itself, so build first (build.sh).

  db/.venv/bin/python db/web/bench.py           (needs the `websockets` package, which db/.venv has)

Measures the map's date slider, the disputes filter, and the notebook with a 2,000-claim result: run, "Show all",
JSON and back. game/design/rewrites.md keeps the numbers before and after each rewrite."""
import asyncio
import json
import subprocess
import sys
import tempfile
import time
import urllib.request
from pathlib import Path

import websockets

DIST = Path(__file__).resolve().parent / "crates" / "app" / "dist"
PORT, CDP = 8772, 9334
BASE = f"http://127.0.0.1:{PORT}/"

OBSERVE = """window.__lt = []; new PerformanceObserver(l => { for (const e of l.getEntries()) window.__lt.push(e.duration); })
  .observe({type: 'longtask', buffered: true}); true"""
SETTLE = "new Promise(r => setTimeout(r, 1500))"
QUERY = "SELECT id, key, text, kind, time, topics FROM claim LIMIT 2000;"


def dispatch(selector: str, value: str, event: str) -> str:
    """Set a form control's value and fire its event, as a person would."""
    return f"""(() => {{ const el = document.querySelector({json.dumps(selector)}); el.value = {json.dumps(value)};
        el.dispatchEvent(new Event({json.dumps(event)}, {{bubbles: true}})); return true; }})()"""


def click(selector: str, text: str | None = None) -> str:
    """Click the first element matching the selector (with that text, if given)."""
    pick = f"[...document.querySelectorAll({json.dumps(selector)})].find(b => b.innerText == {json.dumps(text)})" if text \
        else f"document.querySelector({json.dumps(selector)})"
    return f"(() => {{ const b = {pick}; if (b) b.click(); return !!b; }})()"


def start() -> tuple[subprocess.Popen, subprocess.Popen, str]:
    """The static server and the browser; returns them with the page's debugging socket."""
    server = subprocess.Popen([sys.executable, "-m", "http.server", str(PORT), "-d", str(DIST), "--bind", "127.0.0.1"],
                              stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    browser = subprocess.Popen(["chromium", "--headless=new", "--no-sandbox", "--disable-gpu", f"--remote-debugging-port={CDP}",
                                f"--user-data-dir={tempfile.mkdtemp(prefix='kbbench_')}", "about:blank"],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    for _ in range(50):
        try:
            tabs = json.load(urllib.request.urlopen(f"http://127.0.0.1:{CDP}/json"))
            return server, browser, next(t["webSocketDebuggerUrl"] for t in tabs if t["type"] == "page")
        except OSError:
            time.sleep(0.2)
    raise SystemExit("chromium did not open its debugging port")


async def session(ws_url: str):
    async with websockets.connect(ws_url, max_size=None) as ws:
        n = 0

        async def call(method, **params):
            nonlocal n
            n += 1
            await ws.send(json.dumps({"id": n, "method": method, "params": params}))
            while True:
                m = json.loads(await ws.recv())
                if m.get("id") == n:
                    return m.get("result", {})

        async def ev(expr):
            r = await call("Runtime.evaluate", expression=expr, awaitPromise=True, returnByValue=True)
            return r.get("result", {}).get("value")

        async def go(route):
            await ev(f"location.hash = {json.dumps(route)}; true")

        async def measure(label, action):
            await ev("window.__lt = []; true")
            t0 = await ev("performance.now()")
            await ev(action)
            await ev(SETTLE)
            t1 = await ev("performance.now()")
            lt = await ev("window.__lt")
            print(f"{label:42} long tasks {sum(lt):7.0f} ms (max {max(lt or [0]):5.0f}, n={len(lt)}), wall {t1 - t0 - 1500:6.0f} ms", flush=True)

        await call("Page.navigate", url=BASE + "#/about")
        await asyncio.sleep(2)
        await ev(OBSERVE)
        for _ in range(120):
            if "ready in" in (await ev("document.body.innerText") or ""):
                break
            await asyncio.sleep(1)
        print("ready:", await ev("document.querySelector('.status').innerText"), flush=True)

        await go("#/map")
        await asyncio.sleep(8)
        for v in (10, 20, 30, 40, 50):
            await measure(f"map: move 'from' slider to {v}", dispatch("input[type=range]", str(v), "input"))
        await go("#/disputes")
        await asyncio.sleep(6)
        for i in range(1, 6):
            await measure(f"disputes: filter '{'drone'[:i]}'", dispatch("input[type=search]", "drone"[:i], "input"))
        await measure("notebook: run 2,000-claim query", f"(() => {{ location.hash = '#/notebook?q=' + encodeURIComponent({json.dumps(QUERY)}); return true; }})()")
        await asyncio.sleep(5)
        await measure("notebook: 'Show all' rows", click("button.more"))
        await measure("notebook: switch to JSON", click(".modes button", "JSON"))
        await measure("notebook: back to Table", click(".modes button", "Table"))


def main():
    server, browser, ws_url = start()
    try:
        asyncio.run(session(ws_url))
    finally:
        browser.terminate()
        server.terminate()


if __name__ == "__main__":
    main()
