use crate::views::{BoxBlur, Gamma, Invert, SobelEdgeDetector};
use ev::MouseEvent;
use html::Div;
use js_sys::{Array, ArrayBuffer, JsString, Object, Reflect, Uint8ClampedArray};
use leptos::html::{Canvas, Img, Input, ToHtmlElement};
use leptos::leptos_dom::Text;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos::{component, create_signal, svg::view, view, IntoView};
use log::{error, info};
use shared::{
    BoxBlurMessage, Command, GammaMessage, InvertMessage, NewImageMessage,
    SobelEdgeDetectionMessage, ToJsObject, WorkerResponseMessage,
};
use std::rc::Rc;
use std::str::FromStr;
use wasm_bindgen::JsValue;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::{
    window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData,
    MediaQueryListEvent, MessageEvent, Url, WorkerOptions, WorkerType,
};

use crate::Algorithm;

#[component]
pub fn AlgorithmList<F>(
    is_screen_desktop_size: ReadSignal<bool>,
    set_algorithm: WriteSignal<Option<Algorithm>>,
    disabled: Signal<bool>,
    current_algorithm: ReadSignal<Option<Algorithm>>,
    select_image_onclick: F,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let algorithms = vec![
        Algorithm::Invert,
        Algorithm::Gamma,
        Algorithm::BoxBlur,
        Algorithm::SobelEdgeDetector,
    ];

    let desktop_sidebar = view! {
        <div class="sidebar w-full h-full justify-start grow mr-2">
            <section class="sidebar-content h-fit overflow-visible">
                <section class="sidebar-header items-center p-4">
                    // needs to be undelegated because of behavior from wasm bindgen explained here
                    // https://github.com/leptos-rs/leptos/issues/2104
                    <button
                        id="select-image"
                        class="btn btn-rounded btn-primary"
                        on:click:undelegated=select_image_onclick
                    >
                        "Select Image"
                    </button>
                </section>
                <div class="divider my-0"></div>
                <nav class="menu rounded-md">
                    <section class="menu-section px-4">
                        <span class="menu-title">"Algorithms"</span>
                        <ul class="menu-items">
                            {algorithms
                                .clone()
                                .into_iter()
                                .map(|algorithm| {
                                    view! {
                                        <li
                                            class="menu-item"
                                            on:click=move |_| {
                                                set_algorithm(Some(algorithm));
                                            }
                                        >

                                            <span disabled=disabled>{algorithm.to_string()}</span>
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </ul>
                    </section>
                </nav>
            </section>
        </div>
    };

    let mobile_bottombar = view! {
        <div class="">
            <ul class="flex flex-row h-24 bg-gray-200 w-full" disabled=disabled>
                {algorithms
                    .clone()
                    .into_iter()
                    .map(|algorithm| {
                        let is_selected = move || match current_algorithm() {
                            Some(current_algorithm) => current_algorithm == algorithm,
                            None => false,
                        };
                        view! {
                            <li
                                class="w-48 border p-3 hover:bg-gray-300"
                                class=("bg-gray-500", is_selected)
                            >
                                <span
                                    class="flex items-center justify-center w-full h-full"
                                    disabled=disabled
                                    on:click=move |_| {
                                        set_algorithm(Some(algorithm));
                                    }
                                >

                                    {algorithm.to_string()}
                                </span>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    };

    view! {
        {move || {
            if is_screen_desktop_size.get() {
                desktop_sidebar.clone()
            } else {
                mobile_bottombar.clone()
            }
        }}
    }
}
