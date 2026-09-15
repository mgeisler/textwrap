import { expect, test } from "@playwright/test";

test.beforeEach(async ({ page }) => {
  await page.addInitScript(() => {
    window.__fillTextCalls = [];
    const originalClearRect = CanvasRenderingContext2D.prototype.clearRect;
    CanvasRenderingContext2D.prototype.clearRect = function (x, y, w, h) {
      window.__fillTextCalls = [];
      return originalClearRect.apply(this, arguments);
    };

    const originalFillText = CanvasRenderingContext2D.prototype.fillText;
    CanvasRenderingContext2D.prototype.fillText = function (
      text,
      x,
      y,
      maxWidth,
    ) {
      window.__fillTextCalls.push({ text: String(text), x, y });
      return originalFillText.apply(this, arguments);
    };
  });
});

async function getRenderedLines(page) {
  return await page.evaluate(() => {
    const calls = window.__fillTextCalls || [];
    const linesByY = new Map();
    for (const call of calls) {
      // Filter out width measurement annotations (e.g., "248.5px")
      if (/^\d+(\.\d+)?px$/.test(call.text)) {
        continue;
      }
      if (!linesByY.has(call.y)) {
        linesByY.set(call.y, []);
      }
      linesByY.get(call.y).push(call.text);
    }
    return Array.from(linesByY.values()).map((words) => words.join(""));
  });
}

test("loads Wasm bundle and wraps text when slider moves", async ({ page }) => {
  const errors = [];
  page.on("console", (msg) => {
    if (msg.type() === "error") {
      errors.push(msg.text());
    }
  });
  page.on("pageerror", (err) => {
    errors.push(err.message);
  });

  await page.goto("/");

  const canvas = page.locator("#canvas");
  await expect(canvas).toBeVisible();

  // Wait for initial draw_wrapped_text execution from Wasm.
  await page.waitForFunction(
    () => window.__fillTextCalls && window.__fillTextCalls.length > 0,
  );

  const initialLines = await getRenderedLines(page);
  expect(initialLines.length).toBeGreaterThan(0);
  expect(initialLines.join(" ")).toContain("Welcome");
  expect(initialLines.join(" ")).toContain("WebAssembly");

  // Narrow the line width slider to 120px and trigger redraw.
  await page.evaluate(() => {
    window.__fillTextCalls = [];
  });
  const slider = page.locator("#line-width");
  await slider.fill("120");
  await slider.dispatchEvent("input");

  await page.waitForFunction(
    () => window.__fillTextCalls && window.__fillTextCalls.length > 0,
  );
  const narrowLines = await getRenderedLines(page);
  expect(narrowLines.length).toBeGreaterThan(initialLines.length);

  // Widen the line width slider to 400px and trigger redraw.
  await page.evaluate(() => {
    window.__fillTextCalls = [];
  });
  await slider.fill("400");
  await slider.dispatchEvent("input");

  await page.waitForFunction(
    () => window.__fillTextCalls && window.__fillTextCalls.length > 0,
  );
  const wideLines = await getRenderedLines(page);
  expect(wideLines.length).toBeLessThan(narrowLines.length);

  expect(errors).toEqual([]);
});

test("re-wraps when editing text in textarea", async ({ page }) => {
  const errors = [];
  page.on("console", (msg) => {
    if (msg.type() === "error") {
      errors.push(msg.text());
    }
  });
  page.on("pageerror", (err) => {
    errors.push(err.message);
  });

  await page.goto("/");

  await page.waitForFunction(
    () => window.__fillTextCalls && window.__fillTextCalls.length > 0,
  );

  await page.evaluate(() => {
    window.__fillTextCalls = [];
  });
  const textarea = page.locator("#text");
  await textarea.fill("Single short line");

  await page.waitForFunction(
    () => window.__fillTextCalls && window.__fillTextCalls.length > 0,
  );
  const lines = await getRenderedLines(page);
  expect(lines).toEqual(["Single short line"]);

  expect(errors).toEqual([]);
});

test("renders build metadata in footer", async ({ page }) => {
  await page.route("**/build-info.json", async (route) => {
    await route.fulfill({
      status: 200,
      contentType: "application/json",
      body: JSON.stringify({
        date: "2026-09-15",
        commit: "0123456789abcdef",
      }),
    });
  });

  await page.goto("/");
  await expect(page.locator("#build-date")).toHaveText("2026-09-15");
  const commitLink = page.locator("#footer a");
  await expect(commitLink).toHaveText("0123456");
  await expect(commitLink).toHaveAttribute(
    "href",
    "https://github.com/mgeisler/textwrap/commit/0123456789abcdef",
  );
});
