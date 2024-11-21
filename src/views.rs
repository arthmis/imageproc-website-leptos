use std::ops::Not;
use std::rc::Rc;
use std::thread::current;

use js_sys::{Object, Reflect};
use leptos::wasm_bindgen::JsCast;
use leptos::{
    component, create_node_ref, create_signal, html::Input, view, IntoView, ReadSignal, RwSignal,
    SignalSet,
};
use leptos::{NodeRef, SignalGet, WriteSignal};
use log::{error, info};
use shared::Command;
use wasm_bindgen::JsValue;
use web_sys::{Event, HtmlInputElement, InputEvent, MouseEvent, Url, Worker};

use crate::Algorithm;

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
            "gamma "
            {gamma}
        </label>
        <input
            id="gamma-slider"
            class="range sm:w-4/5 lg:w-64"
            type="range"
            name="gamma"
            min="0.2"
            max="5"
            step="0.1"
            value=default_gamma.to_string()
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
        <button class="btn lg:w-32 sm:w-9/12" on:click=click>
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
            "box blur "
            {box_blur_amount}
        </label>
        <input
            id="box-blur-slider"
            class="range"
            type="range"
            name="box-blur"
            min="1"
            max="99"
            step="2"
            value=box_blur.to_string()
            on:change=slider
        />
    }
}

#[component]
pub fn SobelEdgeDetector(threshold: RwSignal<u32>) -> impl IntoView {
    let default_threshold = 128;
    threshold.set(default_threshold);

    let slider = move |ev: Event| {
        let element = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        let value = element.value();
        let value = value.parse::<f64>().unwrap() as u32;
        threshold.set(value);
        info!("sliding for sobel edge detector: {}", threshold.get());
    };

    view! {
        <label for="sobel-edge-detector-slider" class="some-custom-css">
            "sobel edge detector "
            {threshold}
        </label>
        <input
            id="box-blur-slider"
            class="range"
            type="range"
            name="box-blur"
            min="1"
            max="255"
            step="1"
            value=default_threshold.to_string()
            on:change=slider
        />
    }
}

// #[component]
// pub fn Image() -> impl IntoView {
//     let image_ref = create_node_ref::<Img>();

//     let handle_image_load = move |ev| {
//         info!("{}", "image loaded");
//         let image_node = image_ref.get().unwrap();
//         let canvas_wrapper_node = canvas_wrapper.get().unwrap();
//         let selected_image_canvas = selected_image_canvas.get().unwrap();

//         // let canvas_wrapper_width = canvas_wrapper_node.client_width();
//         // let canvas_wrapper_height = canvas_wrapper_node.client_height();
//         let selected_image_canvas = selected_image_canvas.style("width", "100%");
//         let selected_image_canvas = selected_image_canvas.style("height", "100%");
//         // let new_canvas_width = selected_image_canvas.client_width();
//         // let new_canvas_height = selected_image_canvas.client_height();
//         let new_canvas_width = selected_image_canvas.offset_width();
//         let new_canvas_height = selected_image_canvas.offset_height();
//         info!(
//             "selected_image_canvas client width: {}",
//             selected_image_canvas.client_width()
//         );
//         info!(
//             "selected_image_canvas client height: {}",
//             selected_image_canvas.client_height()
//         );

//         selected_image_canvas.set_width(new_canvas_width as u32);
//         // TODO: setting the height directly with using .offset_height() or any other height
//         // functions
//         // doesn't work correctly. However if I place the value into a variable first then it works
//         // no idea how this is happening
//         selected_image_canvas.set_height(new_canvas_width as u32);

//         let (scaled_width, scaled_height) =
//             get_scaled_image_dimensions_to_canvas(&image_node, &selected_image_canvas);

//         let new_canvas_width = selected_image_canvas.client_width();
//         let new_canvas_height = selected_image_canvas.client_height();

//         selected_image_canvas.set_width(new_canvas_width as u32);
//         // TODO: setting the height directly with using .offset_height() or any other height
//         // functions
//         // doesn't work correctly. However if I place the value into a variable first then it works
//         // no idea how this is happening
//         selected_image_canvas.set_height(new_canvas_height as u32);

//         let center_x = (selected_image_canvas.width() as f64 - scaled_width) / 2.;
//         let center_y = (selected_image_canvas.height() as f64 - scaled_height) / 2.;
//         let canvas_context = selected_image_canvas
//             .get_context("2d")
//             .unwrap()
//             .unwrap()
//             .dyn_into::<CanvasRenderingContext2d>()
//             .unwrap();
//         canvas_context
//             .draw_image_with_html_image_element_and_dw_and_dh(
//                 &image_node,
//                 center_x,
//                 center_y,
//                 scaled_width,
//                 scaled_height,
//             )
//             .unwrap();

//         let image_data = canvas_context
//             .get_image_data(center_x, center_y, scaled_width, scaled_height)
//             .unwrap();

//         // reset current algorithm to be None for a new image
//         // reset algorithm values
//         // TODO look into making this a function or something
//         set_algorithm(None);
//         invert.set(false);
//         box_blur_amount.set(1);
//         gamma.set(1.);

//         let new_image_message = NewImageMessage::new(
//             Command::NewImage.to_string(),
//             image_data.data(),
//             center_x,
//             center_y,
//             scaled_width,
//             scaled_height,
//         );
//         let mut array: Array = Array::new();
//         array.push(&new_image_message.js_clamped_uint8_array().buffer());

//         onload_worker
//             .post_message_with_transfer(&new_image_message.to_js_object(), &array)
//             .unwrap();
//     };

//     view! { <img _ref=image_ref src="" style="display: none" on:load=handle_image_load/> }
// }

#[component]
pub fn InvisibleSelectFile(
    file_input_ref: NodeRef<Input>,
    set_image_url: WriteSignal<String>,
) -> impl IntoView {
    let on_change = move |ev| {
        let node = file_input_ref.get().unwrap();
        let files = node.files().unwrap();
        let file = files.item(0).unwrap();
        let image_url_raw = Url::create_object_url_with_blob(&file).unwrap();
        set_image_url(image_url_raw);
    };

    view! {
        <input
            type="file"
            id="file-input"
            accept="image/png, image/jpeg"
            style="display: none;"
            _ref=file_input_ref
            on:change=on_change
        />
    }
}

#[component]
pub fn CurrentAlgorithm(
    gamma: RwSignal<f64>,
    invert: RwSignal<bool>,
    box_blur_amount: RwSignal<u32>,
    sobel_edge_detector_threshold: RwSignal<u32>,
    algorithm: ReadSignal<Option<Algorithm>>,
) -> impl IntoView {
    let current_algorithm = move || match algorithm() {
        Some(current_algorithm) => match current_algorithm {
            Algorithm::Gamma => Some(view! { <Gamma gamma=gamma/> }),
            Algorithm::Invert => Some(view! { <Invert invert=invert/> }),
            Algorithm::BoxBlur => Some(view! { <BoxBlur box_blur_amount=box_blur_amount/> }),
            Algorithm::SobelEdgeDetector => {
                Some(view! { <SobelEdgeDetector threshold=sobel_edge_detector_threshold/> })
            }
        },
        None => None,
    };
    view! { {current_algorithm} }
}
