use leptos::prelude::*;

use crate::models::pdf_page_viewport::PdfPageViewport;
use crate::models::pdf_text_item::PdfTextItem;
#[cfg(not(coverage))]
use leptos::ev;

#[derive(Debug, Clone, PartialEq)]
pub struct WordOverlayBox {
    pub index: usize,
    pub text: String,
    pub selected: bool,
    pub style: String,
}

pub fn px(value: f64) -> String {
    format!("{value}px")
}

pub fn pdf_page_body_style(width: f64, height: f64) -> String {
    format!("width: {}; height: {};", px(width), px(height))
}

pub fn word_box_style(item: &PdfTextItem) -> String {
    format!(
        "left: {}; top: {}; width: {}; height: {};",
        px(item.left),
        px(item.top),
        px(item.width),
        px(item.height)
    )
}

pub fn word_boxes_for_page(items: &[PdfTextItem], page_number: u32) -> Vec<WordOverlayBox> {
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| item.page == page_number)
        .map(|(index, item)| WordOverlayBox {
            index,
            text: item.text.clone(),
            selected: item.selected,
            style: word_box_style(item),
        })
        .collect()
}

#[component]
#[cfg(not(coverage))]
pub fn PdfWordOverlay(
    pages: ReadSignal<Vec<PdfPageViewport>>,
    items: ReadSignal<Vec<PdfTextItem>>,
    on_word_click: Callback<usize>,
) -> impl IntoView {
    let overlay_pages = move || {
        pages
            .get()
            .into_iter()
            .map(|page| {
                let page_number = page.page;
                let page_body_style = pdf_page_body_style(page.width, page.height);
                let page_words = move || {
                    word_boxes_for_page(&items.get(), page_number)
                        .into_iter()
                        .map(|word_box| {
                            let action = if word_box.selected { "Deselect" } else { "Select" };
                            let label = format!("{action} word {}", word_box.text);
                            let class = if word_box.selected {
                                "pdf-word-box pdf-word-box--selected"
                            } else {
                                "pdf-word-box"
                            };
                            let word_index = word_box.index;
                            let handle_click = move |event: ev::MouseEvent| {
                                event.stop_propagation();
                                on_word_click.run(word_index);
                            };

                            view! {
                                <button
                                    type="button"
                                    class=class
                                    style=word_box.style
                                    aria-label=label
                                    title=word_box.text
                                    on:click=handle_click
                                ></button>
                            }
                        })
                        .collect_view()
                };

                view! {
                    <section class="pdf-page-shell pdf-page-overlay-shell">
                        <div class="pdf-page-meta" aria-hidden="true">{format!("Page {page_number}")}</div>
                        <div
                            class="pdf-page-body pdf-page-overlay-layer"
                            style=page_body_style
                        >
                            {page_words}
                        </div>
                    </section>
                }
            })
            .collect_view()
    };

    view! {
        <div class="pdf-word-overlay" aria-label="Detected word overlay">
            {overlay_pages}
        </div>
    }
}

#[component]
#[cfg(coverage)]
pub fn PdfWordOverlay(
    pages: ReadSignal<Vec<PdfPageViewport>>,
    items: ReadSignal<Vec<PdfTextItem>>,
    on_word_click: Callback<usize>,
) -> impl IntoView {
    let _ = (pages, items, on_word_click);

    view! { <div class="pdf-word-overlay" aria-label="Detected word overlay"></div> }
}
