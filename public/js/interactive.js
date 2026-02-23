// =============================================================================
// Interactive Code Examples — Live preview for Ply documentation
// =============================================================================

(function () {
  "use strict";

  // Sizing macro parser
  // Returns { type, min, max, value } or null on parse error
  function parseSizing(raw) {
    const s = raw.trim();

    // grow!() / grow!(min) / grow!(min, max)
    let m = s.match(/^grow!\(\s*\)$/);
    if (m) return { type: "grow", min: 0, max: Infinity };

    m = s.match(/^grow!\(\s*([\d.]+)\s*\)$/);
    if (m) return { type: "grow", min: parseFloat(m[1]), max: Infinity };

    m = s.match(/^grow!\(\s*([\d.]+)\s*,\s*([\d.]+)\s*\)$/);
    if (m) return { type: "grow", min: parseFloat(m[1]), max: parseFloat(m[2]) };

    // fit!() / fit!(min) / fit!(min, max)
    m = s.match(/^fit!\(\s*\)$/);
    if (m) return { type: "fit", min: 0, max: Infinity };

    m = s.match(/^fit!\(\s*([\d.]+)\s*\)$/);
    if (m) return { type: "fit", min: parseFloat(m[1]), max: Infinity };

    m = s.match(/^fit!\(\s*([\d.]+)\s*,\s*([\d.]+)\s*\)$/);
    if (m) return { type: "fit", min: parseFloat(m[1]), max: parseFloat(m[2]) };

    // fixed!(value)
    m = s.match(/^fixed!\(\s*([\d.]+)\s*\)$/);
    if (m) return { type: "fixed", value: parseFloat(m[1]) };

    // percent!(value) — 0.0 to 1.0
    m = s.match(/^percent!\(\s*([\d.]+)\s*\)$/);
    if (m) {
      const v = parseFloat(m[1]);
      if (v < 0 || v > 1) return null;
      return { type: "percent", value: v };
    }

    return null;
  }

  // Convert parsed sizing to CSS styles
  function sizingToCSS(parsed, axis) {
    const styles = {};
    const prop = axis === "width" ? "width" : "height";
    const minProp = axis === "width" ? "minWidth" : "minHeight";
    const maxProp = axis === "width" ? "maxWidth" : "maxHeight";

    switch (parsed.type) {
      case "grow":
        styles.flex = "1";
        styles[prop] = "100%";
        if (parsed.min > 0) styles[minProp] = parsed.min + "px";
        if (parsed.max < Infinity) styles[maxProp] = parsed.max + "px";
        break;
      case "fit":
        styles[prop] = "fit-content";
        if (parsed.min > 0) styles[minProp] = parsed.min + "px";
        if (parsed.max < Infinity) styles[maxProp] = parsed.max + "px";
        break;
      case "fixed":
        styles[prop] = parsed.value + "px";
        break;
      case "percent":
        styles[prop] = (parsed.value * 100) + "%";
        break;
    }
    return styles;
  }

  // Map Ply alignment to CSS
  function alignToCSS(alignX, alignY, direction) {
    const isRow = direction === "LeftToRight" || direction === "RightToLeft";

    // In CSS flexbox:
    // - justify-content = main axis (row: horizontal, column: vertical)
    // - align-items = cross axis (row: vertical, column: horizontal)

    const xMap = { Left: "flex-start", CenterX: "center", Right: "flex-end" };
    const yMap = { Top: "flex-start", CenterY: "center", Bottom: "flex-end" };

    if (isRow) {
      return {
        justifyContent: xMap[alignX] || "flex-start",
        alignItems: yMap[alignY] || "flex-start",
      };
    } else {
      return {
        justifyContent: yMap[alignY] || "flex-start",
        alignItems: xMap[alignX] || "flex-start",
      };
    }
  }

  // Map Ply direction to CSS flex-direction
  function directionToCSS(dir) {
    const map = {
      LeftToRight: "row",
      RightToLeft: "row-reverse",
      TopToBottom: "column",
      BottomToTop: "column-reverse",
    };
    return map[dir] || "column";
  }

  // Color hex conversion (Ply hex to CSS)
  function plyColorToCSS(hexStr) {
    // Input could be "0xFF0000" or "#FF0000"
    const s = hexStr.trim().replace(/^0x/i, "#");
    if (/^#[0-9a-f]{6}$/i.test(s)) return s;
    return null;
  }

  // Initialize all interactive examples on the page
  function initExamples() {
    document.querySelectorAll(".interactive-example").forEach(initExample);
  }

  function initExample(container) {
    const element = container.querySelector(".ix-element");
    if (!element) return;

    const allInputs = container.querySelectorAll(".ix-input");
    const allSelects = container.querySelectorAll(".ix-select");
    const allColors = container.querySelectorAll(".ix-color");

    function update() {
      // Gather values
      const vals = {};
      allInputs.forEach((inp) => {
        vals[inp.dataset.param] = inp.value;
      });
      allSelects.forEach((sel) => {
        vals[sel.dataset.param] = sel.value;
      });
      allColors.forEach((col) => {
        vals[col.dataset.param] = col.value;
      });

      // Parse and apply width
      if (vals.width !== undefined) {
        const widthInput = container.querySelector('[data-param="width"]');
        const parsed = parseSizing(vals.width);
        if (parsed) {
          widthInput.classList.remove("ix-error");
          const css = sizingToCSS(parsed, "width");
          element.style.width = css.width || "";
          element.style.minWidth = css.minWidth || "";
          element.style.maxWidth = css.maxWidth || "";
          if (css.flex) element.style.flex = css.flex;
          else element.style.flex = "";
        } else {
          widthInput.classList.add("ix-error");
        }
      }

      // Parse and apply height
      if (vals.height !== undefined) {
        const heightInput = container.querySelector('[data-param="height"]');
        const parsed = parseSizing(vals.height);
        if (parsed) {
          heightInput.classList.remove("ix-error");
          const css = sizingToCSS(parsed, "height");
          element.style.height = css.height || "";
          element.style.minHeight = css.minHeight || "";
          element.style.maxHeight = css.maxHeight || "";
        } else {
          heightInput.classList.add("ix-error");
        }
      }

      // Apply background color
      if (vals.bg !== undefined) {
        element.style.backgroundColor = vals.bg;
      }

      // Apply direction
      const dir = vals.direction || "TopToBottom";
      element.style.flexDirection = directionToCSS(dir);

      // Apply gap
      if (vals.gap !== undefined) {
        const gapVal = parseInt(vals.gap, 10);
        if (!isNaN(gapVal) && gapVal >= 0) {
          element.style.gap = gapVal + "px";
          const gapInput = container.querySelector('[data-param="gap"]');
          if (gapInput) gapInput.classList.remove("ix-error");
        } else {
          const gapInput = container.querySelector('[data-param="gap"]');
          if (gapInput) gapInput.classList.add("ix-error");
        }
      }

      // Apply alignment
      const alignX = vals["align-x"] || "Left";
      const alignY = vals["align-y"] || "Top";
      const alignment = alignToCSS(alignX, alignY, dir);
      element.style.justifyContent = alignment.justifyContent;
      element.style.alignItems = alignment.alignItems;

      // Apply corner radius
      if (vals.radius !== undefined) {
        const r = parseFloat(vals.radius);
        if (!isNaN(r) && r >= 0) {
          element.style.borderRadius = r + "px";
          const rInput = container.querySelector('[data-param="radius"]');
          if (rInput) rInput.classList.remove("ix-error");
        } else {
          const rInput = container.querySelector('[data-param="radius"]');
          if (rInput) rInput.classList.add("ix-error");
        }
      }

      // Apply padding
      if (vals.padding !== undefined) {
        const p = parseInt(vals.padding, 10);
        if (!isNaN(p) && p >= 0) {
          element.style.padding = p + "px";
          const pInput = container.querySelector('[data-param="padding"]');
          if (pInput) pInput.classList.remove("ix-error");
        } else {
          const pInput = container.querySelector('[data-param="padding"]');
          if (pInput) pInput.classList.add("ix-error");
        }
      }
    }

    // Bind event listeners
    allInputs.forEach((inp) => {
      inp.addEventListener("input", () => { autoSize(inp); update(); });
      autoSize(inp);
    });
    allSelects.forEach((sel) => {
      sel.addEventListener("change", () => { autoSizeSelect(sel); update(); });
      autoSizeSelect(sel);
    });
    allColors.forEach((col) => {
      col.addEventListener("input", () => {
        update();
        // Update hex label next to color picker
        const label = col.nextElementSibling;
        if (label && label.classList.contains("ix-color-label")) {
          label.textContent = "0x" + col.value.slice(1).toUpperCase();
        }
      });
    });

    // Initial render
    update();
  }

  // Auto-size a text input to fit its value
  function autoSize(input) {
    const min = parseInt(input.dataset.minch, 10) || 3;
    const len = Math.max(input.value.length, min);
    input.style.width = (len + 1) + "ch";
  }

  // Auto-size a select to fit its selected option
  function autoSizeSelect(select) {
    const text = select.options[select.selectedIndex].text;
    const min = 4;
    select.style.width = Math.max(text.length, min) + 1 + "ch";
  }

  // Run on DOM ready
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", initExamples);
  } else {
    initExamples();
  }
})();
