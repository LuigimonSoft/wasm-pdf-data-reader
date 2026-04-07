const PDFJS_VERSION = "4.4.168";
const PDFJS_BASE_URL = `https://cdn.jsdelivr.net/npm/pdfjs-dist@${PDFJS_VERSION}/legacy/build`;

let pdfjsLibPromise = null;

function ensurePromiseWithResolvers() {
  if (typeof Promise.withResolvers === "function") {
    return;
  }

  Promise.withResolvers = function withResolvers() {
    let resolve;
    let reject;
    const promise = new Promise((innerResolve, innerReject) => {
      resolve = innerResolve;
      reject = innerReject;
    });

    return { promise, resolve, reject };
  };
}

async function getPdfJs() {
  if (pdfjsLibPromise != null) {
    return pdfjsLibPromise;
  }

  pdfjsLibPromise = (async () => {
    ensurePromiseWithResolvers();

    try {
      const pdfjsLib = await import(`${PDFJS_BASE_URL}/pdf.min.mjs`);
      pdfjsLib.GlobalWorkerOptions.workerSrc = `${PDFJS_BASE_URL}/pdf.worker.min.mjs`;

      return pdfjsLib;
    } catch (error) {
      console.error("[pdf_bridge] failed to load pdf.js", error);
      throw new Error(
        "Failed to load the Safari-compatible pdf.js bundle. Check Safari version and console output."
      );
    }
  })();

  return pdfjsLibPromise;
}

function toFiniteNumber(value, fallback = 0) {
  const numericValue = Number(value);

  return Number.isFinite(numericValue) ? numericValue : fallback;
}

function isSupportedTextItem(item) {
  return (
    item != null &&
    typeof item.str === "string" &&
    Array.isArray(item.transform) &&
    item.transform.length >= 6
  );
}

function isSafariBrowser() {
  if (typeof navigator === "undefined") {
    return false;
  }

  const userAgent = navigator.userAgent;
  return /Safari\//.test(userAgent) && !/Chrome\//.test(userAgent) && !/Chromium\//.test(userAgent);
}

function splitTextItemIntoWords(item, pageNumber, viewport) {
  if (!isSupportedTextItem(item)) {
    return [];
  }

  const rawText = item.str.replace(/\s+/g, " ").trim();

  if (!rawText) {
    return [];
  }

  const words = rawText.match(/\S+/g) ?? [];

  if (words.length === 0) {
    return [];
  }

  const [, , , rawD, rawE, rawF] = item.transform;
  const d = toFiniteNumber(rawD, 1);
  const e = toFiniteNumber(rawE, 0);
  const f = toFiniteNumber(rawF, 0);
  const scale = toFiniteNumber(viewport.scale, 1);
  const [rawViewportLeft, rawViewportBottom] = viewport.convertToViewportPoint(e, f);
  const viewportLeft = toFiniteNumber(rawViewportLeft, 0);
  const viewportBottom = toFiniteNumber(rawViewportBottom, 0);
  const totalWidth = Math.max(toFiniteNumber(item.width, 0) * scale, rawText.length, 1);
  const totalHeight = Math.max(Math.abs(toFiniteNumber(item.height, d) * scale), 12);

  let cursor = 0;
  const extractedWords = [];

  for (const word of words) {
    const relativeIndex = rawText.indexOf(word, cursor);
    const startIndex = relativeIndex >= 0 ? relativeIndex : cursor;
    const endIndex = startIndex + word.length;
    const left = viewportLeft + (totalWidth * startIndex) / rawText.length;
    const width = Math.max((totalWidth * word.length) / rawText.length, 1);
    const top = viewportBottom - totalHeight;

    cursor = endIndex;

    if (![left, top, width, totalHeight].every(Number.isFinite)) {
      continue;
    }

    extractedWords.push({
      page: pageNumber,
      text: word,
      selected: true,
      left,
      top,
      width,
      height: totalHeight,
      transform: item.transform.map((value) => toFiniteNumber(value, 0)),
    });
  }

  return extractedWords;
}

function normalizeTextItems(items) {
  if (Array.isArray(items)) {
    return items;
  }

  if (items == null) {
    return [];
  }

  if (typeof items.length === "number") {
    const normalizedItems = [];

    for (let index = 0; index < items.length; index += 1) {
      normalizedItems.push(items[index]);
    }

    return normalizedItems;
  }

  try {
    return Array.from(items);
  } catch (_error) {
    return [];
  }
}

export async function load_pdf_and_extract(bytes, host) {
  const pdfjsLib = await getPdfJs();

  let isSafari = false;
  try {
    isSafari = isSafariBrowser();
    
  } catch (error) {
    console.error("[pdf_bridge] safari detection failed", error);
  }

  let byteLength = null;
  try {
    byteLength = bytes && typeof bytes.length === "number"
      ? bytes.length
      : bytes && typeof bytes.byteLength === "number"
        ? bytes.byteLength
        : null;
  } catch (error) {
    console.error("[pdf_bridge] byte length read failed", error);
  }

  const documentOptions = {
    data: bytes,
    useWorkerFetch: !isSafari,
    isOffscreenCanvasSupported: !isSafari,
    isImageDecoderSupported: !isSafari,
    useWasm: !isSafari,
    stopAtErrors: true,
    verbosity: pdfjsLib.VerbosityLevel?.INFOS ?? 1,
  };

  const loadingTask = pdfjsLib.getDocument(documentOptions);
  loadingTask.onProgress = (progress) => {
    
  };

  let pdf;
  try {
    pdf = await loadingTask.promise;
  } catch (error) {
    console.error("[pdf_bridge] loadingTask.promise failed", error);
    throw error;
  }

  const allItems = [];
  const pages = [];
  const scale = 1.5;
  const verbosityLevel = pdfjsLib.VerbosityLevel;

  host.innerHTML = "";

  for (let pageNumber = 1; pageNumber <= pdf.numPages; pageNumber += 1) {
    const page = await pdf.getPage(pageNumber);
    const viewport = page.getViewport({ scale });

    const pageShell = document.createElement("section");
    pageShell.className = "pdf-page-shell";

    const pageMeta = document.createElement("div");
    pageMeta.className = "pdf-page-meta";
    pageMeta.textContent = `Page ${pageNumber}`;

    const canvas = document.createElement("canvas");
    canvas.className = "pdf-page-canvas";
    canvas.width = Math.ceil(viewport.width);
    canvas.height = Math.ceil(viewport.height);
    canvas.style.width = `${Math.ceil(viewport.width)}px`;
    canvas.style.height = `${Math.ceil(viewport.height)}px`;

    const pageBody = document.createElement("div");
    pageBody.className = "pdf-page-body";
    pageBody.style.width = `${Math.ceil(viewport.width)}px`;
    pageBody.style.height = `${Math.ceil(viewport.height)}px`;

    pageBody.append(canvas);
    pageShell.append(pageMeta, pageBody);
    host.append(pageShell);
    pages.push({
      page: pageNumber,
      width: Math.ceil(viewport.width),
      height: Math.ceil(viewport.height),
    });

    const ctx = canvas.getContext("2d");

    try {
      await page.render({
        canvasContext: ctx,
        viewport,
      }).promise;
    } catch (error) {
      console.error("[pdf_bridge] page render failed", { pageNumber, error });
      throw error;
    }

    let textContent;
    try {
      textContent = await page.getTextContent();
    } catch (error) {
      console.error("[pdf_bridge] getTextContent failed", { pageNumber, error });
      throw error;
    }

    const rawItems = normalizeTextItems(textContent && textContent.items);
    const pageItems = [];

    try {
      for (let index = 0; index < rawItems.length; index += 1) {
        const item = rawItems[index];
        const extractedWords = splitTextItemIntoWords(item, pageNumber, viewport);

        for (let wordIndex = 0; wordIndex < extractedWords.length; wordIndex += 1) {
          pageItems.push(extractedWords[wordIndex]);
        }
      }
    } catch (error) {
      console.error("[pdf_bridge] textContent processing failed", {
        pageNumber,
        rawItemCount: rawItems.length,
        error,
      });
      throw error;
    }

    allItems.push(...pageItems);
  }

  return {
    total_pages: pdf.numPages,
    pages,
    items: allItems,
  };
}
