use std::rc::Rc;

use js_sys::{Object, Reflect};
use leptos::wasm_bindgen::JsCast;
use leptos::SignalGet;
use leptos::{component, create_node_ref, create_signal, html::Input, view, IntoView};
use log::{error, info};
use shared::Command;
use wasm_bindgen::JsValue;
use web_sys::{Event, HtmlInputElement, InputEvent, MouseEvent, Worker};

#[component]
pub fn Gamma(worker: Rc<Worker>) -> impl IntoView {
    let (gamma_display, set_gamma_display) = create_signal("2.2".to_string());
    let slider = move |ev: Event| {
        let element = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        let value = element.value();
        set_gamma_display(value);
        info!("sliding for gamma: {}", gamma_display.get());
        let mut message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("message"),
            &JsValue::from_str(Command::Gamma.to_string().as_ref()),
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str(Command::Gamma.to_string().as_ref()),
            &JsValue::from_f64(gamma_display.get().parse::<f64>().unwrap()),
        )
        .unwrap();
        worker.post_message(&message).unwrap();
    };
    view! {
        <label for="gamma-slider" class="some-custom-css">
            gamma {gamma_display}
        </label>
        <input
            id="gamma-slider"
            class=""
            type="range"
            name="gamma"
            min="0.2"
            max="5"
            step="0.1"
            value="2.2"
            on:change=slider
        />
    }
}

#[component]
pub fn Invert(worker: Rc<Worker>) -> impl IntoView {
    let (inverted, set_inverted) = create_signal(false);
    let click = move |ev: MouseEvent| {
        set_inverted(!inverted.get());
        let mut message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("message"),
            &JsValue::from_str(Command::Invert.to_string().as_ref()),
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str(Command::Invert.to_string().as_ref()),
            &JsValue::from_bool(inverted.get()),
        )
        .unwrap();
        worker.post_message(&message).unwrap();
    };
    view! {
        <button on:click=click
        >
            Invert
        </button>
    }
}
