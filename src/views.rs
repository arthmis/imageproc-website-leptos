use std::ops::Not;
use std::rc::Rc;

use js_sys::{Object, Reflect};
use leptos::wasm_bindgen::JsCast;
use leptos::SignalGet;
use leptos::{
    component, create_node_ref, create_signal, html::Input, view, IntoView, ReadSignal, RwSignal,
    SignalSet,
};
use log::{error, info};
use shared::Command;
use wasm_bindgen::JsValue;
use web_sys::{Event, HtmlInputElement, InputEvent, MouseEvent, Worker};

#[component]
pub fn Gamma(gamma: RwSignal<f64>) -> impl IntoView {
    let default_gamma = 1.;
    gamma.set(default_gamma);

    let slider = move |ev: Event| {
        let element = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        let value = element.value();
        let value = gamma.set(value.parse::<f64>().unwrap());
        info!("sliding for gamma: {}", gamma.get());
    };

    view! {
        <label for="gamma-slider" class="some-custom-css">
            "gamma "{gamma}
        </label>
        <input
            id="gamma-slider"
            class="range sm:w-4/5 lg:w-64"
            type="range"
            name="gamma"
            min="0.2"
            max="5"
            step="0.1"
            value={default_gamma.to_string()}
            on:change=slider
        />
    }
}

#[component]
pub fn Invert(invert: RwSignal<bool>) -> impl IntoView {
    let click = move |ev: MouseEvent| {
        invert.set(invert.get().not());
    };
    view! {
        <button class="btn lg:w-32 sm:w-9/12" on:click=click
        >
            "Invert"
        </button>
    }
}

#[component]
pub fn BoxBlur(box_blur_amount: RwSignal<u32>) -> impl IntoView {
    let box_blur = 1;
    box_blur_amount.set(box_blur);

    let slider = move |ev: Event| {
        let element = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        let value = element.value();
        let value = value.parse::<f64>().unwrap() as u32;
        box_blur_amount.set(value);
        info!("sliding for box blur: {}", box_blur_amount.get());
    };

    view! {
        <label for="box-blur-slider" class="some-custom-css">
            "box blur "{box_blur_amount}
        </label>
        <input
            id="box-blur-slider"
            class="range"
            type="range"
            name="box-blur"
            min="1"
            max="99"
            step="2"
            value={box_blur.to_string()}
            on:change=slider
        />
    }
}

#[component]
pub fn SobelEdgeDetector() -> impl IntoView {
    view! {
        <p>"Sobel Edge Detector"</p>
    }
}
