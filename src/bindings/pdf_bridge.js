import * as pdfjsLib from "https://cdn.jsdelivr.net/npm/pdfjs-dist@5.5.207/build/pdf.min.mjs";
                         
pdfjsLib.GlobalWorkerOptions.workerSrc =
  "https://cdn.jsdelivr.net/npm/pdfjs-dist@5.5.207/build/pdf.worker.min.mjs";

export async function load_pdf_and_extract(bytes, canvas) {
  const loadingTask = pdfjsLib.getDocument({ data: bytes });
  const pdf = await loadingTask.promise;

  const allItems = [];
  const scale = 1.5;

  const page = await pdf.getPage(1);
  const viewport = page.getViewport({ scale });

  const ctx = canvas.getContext("2d");
  canvas.width = Math.ceil(viewport.width);
  canvas.height = Math.ceil(viewport.height);

  await page.render({
    canvasContext: ctx,
    viewport,
  }).promise;

  const textContent = await page.getTextContent();

  for (const item of textContent.items) {
    const [a, b, c, d, e, f] = item.transform;
    const [vx, vy] = viewport.convertToViewportPoint(e, f);

    const width = item.width * scale;
    const height = Math.abs(d) * scale || 12;

    allItems.push({
      page: 1,
      text: item.str,
      left: vx,
      top: vy - height,
      width,
      height,
      transform: [a, b, c, d, e, f],
    });
  }

  return allItems;
}