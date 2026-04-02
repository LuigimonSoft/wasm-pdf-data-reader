import * as pdfjsLib from "https://cdn.jsdelivr.net/npm/pdfjs-dist@5.5.207/build/pdf.min.mjs";
                         
pdfjsLib.GlobalWorkerOptions.workerSrc =
  "https://cdn.jsdelivr.net/npm/pdfjs-dist@5.5.207/build/pdf.worker.min.mjs";

function splitTextItemIntoWords(item, pageNumber, viewport) {
  const rawText = (item.str ?? "").trim();

  if (!rawText) {
    return [];
  }

  const words = rawText.match(/\S+/g) ?? [];

  if (words.length === 0) {
    return [];
  }

  const [, , , d, e, f] = item.transform;
  const scale = viewport.scale;
  const [viewportLeft, viewportBottom] = viewport.convertToViewportPoint(e, f);
  const totalWidth = Math.max(item.width * scale, words.length);
  const totalHeight = Math.max(Math.abs((item.height ?? d) * scale), 12);

  let cursor = 0;

  return words.map((word) => {
    const relativeIndex = rawText.indexOf(word, cursor);
    const startIndex = relativeIndex >= 0 ? relativeIndex : cursor;
    const endIndex = startIndex + word.length;
    const left = viewportLeft + (totalWidth * startIndex) / rawText.length;
    const width = Math.max((totalWidth * word.length) / rawText.length, 1);

    cursor = endIndex;

    return {
      page: pageNumber,
      text: word,
      left,
      top: viewportBottom - totalHeight,
      width,
      height: totalHeight,
      transform: [...item.transform],
    };
  });
}

export async function load_pdf_and_extract(bytes, host) {
  const loadingTask = pdfjsLib.getDocument({ data: bytes });
  const pdf = await loadingTask.promise;
  const allItems = [];
  const scale = 1.5;

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

    pageShell.append(pageMeta, canvas);
    host.append(pageShell);

    const ctx = canvas.getContext("2d");

    await page.render({
      canvasContext: ctx,
      viewport,
    }).promise;

    const textContent = await page.getTextContent();

    for (const item of textContent.items) {
      allItems.push(...splitTextItemIntoWords(item, pageNumber, viewport));
    }
  }

  return {
    total_pages: pdf.numPages,
    items: allItems,
  };
}
