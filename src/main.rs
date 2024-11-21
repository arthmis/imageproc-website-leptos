mod views;
use ev::MouseEvent;
use html::Div;
use js_sys::{Array, ArrayBuffer, JsString, Object, Reflect, Uint8ClampedArray};
use leptos::html::{Canvas, Img, Input, ToHtmlElement};
use leptos::leptos_dom::Text;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos::{component, create_signal, svg::view, view, IntoView};
use leptos_use::use_media_query;
use log::{error, info};
use shared::{
    BoxBlurMessage, Command, GammaMessage, InvertMessage, NewImageMessage,
    SobelEdgeDetectionMessage, ToJsObject, WorkerResponseMessage,
};
use std::rc::Rc;
use std::str::FromStr;
use views::{BoxBlur, Gamma, Invert, SobelEdgeDetector};
use wasm_bindgen::JsValue;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::{
    window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData,
    MediaQueryListEvent, MessageEvent, Url, WorkerOptions, WorkerType,
};

mod components;
use components::algorithm_selection::AlgorithmList;
use components::navbar::NavBar;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Algorithm {
    Gamma,
    Invert,
    BoxBlur,
    SobelEdgeDetector,
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Algorithm::Gamma => "gamma",
            Algorithm::Invert => "invert",
            Algorithm::BoxBlur => "box blur",
            Algorithm::SobelEdgeDetector => "sobel edge detector",
        };
        write!(f, "{}", text)
    }
}

impl IntoView for Algorithm {
    fn into_view(self) -> View {
        View::Text(Text::new(self.to_string().into()))
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    leptos::mount_to_body(|| view! { <App/> })
}

// think about using create_effect instead of passing the worker as a prop
// i can create the state for each algorithm in the main app component
// pass those states like the gammma state, invert state into their respective components
// create an effect where when those signals are updated in their respective components then
// the effect function will post the message to the worker, this way only the app component has
// to the worker
// i think this way I wouldn't need a reference counter to the Worker because I don't need to share
// it
#[component]
fn App() -> impl IntoView {
    let (algorithm, set_algorithm) = create_signal(Option::None);
    let (image_url, set_image_url) = create_signal("".to_string());
    let should_algorithm_buttons_be_disabled = Signal::derive(move || image_url().is_empty());
    let file_input_ref = create_node_ref::<Input>();
    let image_ref = create_node_ref::<Img>();
    let canvas_wrapper = create_node_ref::<Div>();
    let selected_image_canvas = create_node_ref::<Canvas>();
    let gamma = create_rw_signal(1.);
    let invert = create_rw_signal(false);
    let box_blur_amount = create_rw_signal(1u32);
    let sobel_edge_detector_threshold = create_rw_signal(128u32);

    let on_worker_message: Closure<dyn FnMut(MessageEvent)> =
        Closure::new(move |message_event: MessageEvent| {
            let message = &Reflect::get(&message_event.data(), &JsValue::from_str("message"))
                .unwrap()
                .as_string()
                .unwrap();
            let worker_message = WorkerResponseMessage::from_str(&message).unwrap();
            match worker_message {
                WorkerResponseMessage::Initialized => {
                    info!("worker message: {}", worker_message.to_string());
                }
                WorkerResponseMessage::Invert
                | WorkerResponseMessage::BoxBlur
                | WorkerResponseMessage::Gamma
                | WorkerResponseMessage::DisplayOriginalImage
                | WorkerResponseMessage::SobelEdgeDetector => {
                    let image_data = {
                        let image_data = Uint8ClampedArray::new(
                            &Reflect::get(&message_event.data(), &JsValue::from_str("image_data"))
                                .unwrap()
                                .dyn_into::<ArrayBuffer>()
                                .unwrap(),
                        );
                        let width = Reflect::get(&message_event.data(), &JsValue::from_str("width"))
                            .unwrap()
                            .as_f64()
                            .unwrap() as u32;

                        ImageData::new_with_u8_clamped_array(
                            wasm_bindgen::Clamped(&image_data.to_vec()),
                            width,
                        )
                        .unwrap()
                    };
                    let center_x =
                        Reflect::get(&message_event.data(), &JsValue::from_str("center_x"))
                            .unwrap()
                            .as_f64()
                            .unwrap();
                    let center_y =
                        Reflect::get(&message_event.data(), &JsValue::from_str("center_y"))
                            .unwrap()
                            .as_f64()
                            .unwrap();
                    let selected_image = selected_image_canvas.get().unwrap();

                    let canvas_context = selected_image
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>()
                        .unwrap();

                    canvas_context.clear_rect(
                        0.0,
                        0.0,
                        selected_image.width() as f64,
                        selected_image.height() as f64,
                    );

                    canvas_context
                        .put_image_data(&image_data, center_x, center_y)
                        .unwrap();
                }
                _ => {
                    panic!("unknown worker response message: {}", worker_message);
                }
            }
        });

    let mut worker_options = WorkerOptions::new();
    worker_options.type_(WorkerType::Module);
    // look into using Refcell like in the rustwasm example
    let worker =
        Rc::new(web_sys::Worker::new_with_options("./worker_loader.js", &worker_options).unwrap());
    worker.set_onmessage(Some(on_worker_message.as_ref().unchecked_ref()));
    on_worker_message.forget();
    let onload_worker = worker.clone();

    let on_image_load = move |ev| {
        info!("{}", "image loaded");
        let image_node = image_ref.get().unwrap();
        let canvas_wrapper_node = canvas_wrapper.get().unwrap();
        let selected_image_canvas = selected_image_canvas.get().unwrap();

        // let canvas_wrapper_width = canvas_wrapper_node.client_width();
        // let canvas_wrapper_height = canvas_wrapper_node.client_height();
        let selected_image_canvas = selected_image_canvas.style("width", "100%");
        let selected_image_canvas = selected_image_canvas.style("height", "100%");
        // let new_canvas_width = selected_image_canvas.client_width();
        // let new_canvas_height = selected_image_canvas.client_height();
        let new_canvas_width = selected_image_canvas.offset_width();
        let new_canvas_height = selected_image_canvas.offset_height();
        info!(
            "selected_image_canvas client width: {}",
            selected_image_canvas.client_width()
        );
        info!(
            "selected_image_canvas client height: {}",
            selected_image_canvas.client_height()
        );

        selected_image_canvas.set_width(new_canvas_width as u32);
        // TODO: setting the height directly with using .offset_height() or any other height
        // functions
        // doesn't work correctly. However if I place the value into a variable first then it works
        // no idea how this is happening
        selected_image_canvas.set_height(new_canvas_width as u32);

        let (scaled_width, scaled_height) =
            get_scaled_image_dimensions_to_canvas(&image_node, &selected_image_canvas);

        let new_canvas_width = selected_image_canvas.client_width();
        let new_canvas_height = selected_image_canvas.client_height();

        selected_image_canvas.set_width(new_canvas_width as u32);
        // TODO: setting the height directly with using .offset_height() or any other height
        // functions
        // doesn't work correctly. However if I place the value into a variable first then it works
        // no idea how this is happening
        selected_image_canvas.set_height(new_canvas_height as u32);

        let center_x = (selected_image_canvas.width() as f64 - scaled_width) / 2.;
        let center_y = (selected_image_canvas.height() as f64 - scaled_height) / 2.;
        let canvas_context = selected_image_canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        canvas_context
            .draw_image_with_html_image_element_and_dw_and_dh(
                &image_node,
                center_x,
                center_y,
                scaled_width,
                scaled_height,
            )
            .unwrap();

        let image_data = canvas_context
            .get_image_data(center_x, center_y, scaled_width, scaled_height)
            .unwrap();

        // reset current algorithm to be None for a new image
        // reset algorithm values
        // TODO look into making this a function or something
        set_algorithm(None);
        invert.set(false);
        box_blur_amount.set(1);
        gamma.set(1.);

        let new_image_message = NewImageMessage::new(
            Command::NewImage.to_string(),
            image_data.data(),
            center_x,
            center_y,
            scaled_width,
            scaled_height,
        );
        let mut array: Array = Array::new();
        array.push(&new_image_message.js_clamped_uint8_array().buffer());

        onload_worker
            .post_message_with_transfer(&new_image_message.to_js_object(), &array)
            .unwrap();
    };

    let on_change = move |ev| {
        let node = file_input_ref.get().unwrap();
        let image_node = image_ref.get().unwrap();
        let files = node.files().unwrap();
        let file = files.item(0).unwrap();
        let image_url_raw = Url::create_object_url_with_blob(&file).unwrap();
        set_image_url(image_url_raw);
        image_node.set_src(&image_url());
    };

    create_effect(move |_| match algorithm() {
        Some(current_algorithm) => match current_algorithm {
            Algorithm::Gamma => {
                let message =
                    GammaMessage::new(Command::Gamma.to_string(), gamma.get()).to_js_object();
                worker.post_message(&message).unwrap();
            }
            Algorithm::Invert => {
                let message =
                    InvertMessage::new(Command::Invert.to_string(), invert.get()).to_js_object();
                worker.post_message(&message).unwrap();
            }
            Algorithm::BoxBlur => {
                let message =
                    BoxBlurMessage::new(Command::BoxBlur.to_string(), box_blur_amount.get())
                        .to_js_object();
                worker.post_message(&message).unwrap();
            }
            Algorithm::SobelEdgeDetector => {
                let message = SobelEdgeDetectionMessage::new(
                    Command::SobelEdgeDetector.to_string(),
                    sobel_edge_detector_threshold.get(),
                )
                .to_js_object();
                worker.post_message(&message).unwrap();
            }
        },
        None => (),
    });

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

    let select_image_onclick = move |event| {
        if let Some(node) = file_input_ref.get() {
            node.click();
        }
    };

    let query = "(min-width: 1024px)";
    let media_query = window().unwrap().match_media(query).unwrap().unwrap();
    let (is_screen_desktop_size, set_is_screen_desktop_size) = create_signal(media_query.matches());
    let on_screen_width_change: Closure<dyn FnMut(MediaQueryListEvent)> =
        Closure::new(move |event: MediaQueryListEvent| {
            set_is_screen_desktop_size(event.matches());
        });
    // put this in a create_effect
    media_query.add_event_listener_with_callback(
        "change",
        on_screen_width_change.as_ref().unchecked_ref(),
    );
    on_screen_width_change.forget();

    let mobile_select_image_button = move || {
        if !is_screen_desktop_size() {
            // needs to be undelegated because of behavior from wasm bindgen explained here
            // https://github.com/leptos-rs/leptos/issues/2104
            Some(view! {
                <button
                    id="select-image"
                    class="btn btn-rounded btn-primary"
                    on:click:undelegated=select_image_onclick
                >
                    // <i class="fa fa-upload" aria-hidden="true" style="font-size:1em;"></i>
                    "Select Image"
                </button>
            })
        } else {
            None
        }
    };

    view! {
        <div class="flex flex-col h-screen">
            <NavBar/>
            <main class="flex flex-col h-full">
                // <div class="flex p-3">
                <div
                    class="flex p-3 justify-center items-center"
                    class=("hidden", is_screen_desktop_size)
                >
                    // class=("flex", is_screen_desktop_size)
                    <input
                        type="file"
                        id="file-input"
                        accept="image/png, image/jpeg"
                        style="display: none;"
                        _ref=file_input_ref
                        on:change=on_change
                    />
                    {mobile_select_image_button}
                </div>
                <img _ref=image_ref src="" style="display: none" on:load=on_image_load/>

                <div class="flex flex-col lg:flex-row lg:flex-row-reverse h-full justify-between">
                    <div class="flex flex-col w-full justify-center items-center">
                        // <div class="flex flex-col grow w-full min-h-[70dvh] max-h-[70dvh] justify-center items-center">
                        <div
                            id="canvas-wrapper"
                            class="flex justify-center items-center w-full h-full grow p-4"
                            _ref=canvas_wrapper
                        >
                            <canvas _ref=selected_image_canvas id="selected-image"></canvas>
                        </div>
                        <div>{current_algorithm}</div>
                    </div>
                    <AlgorithmList
                        is_screen_desktop_size=is_screen_desktop_size
                        disabled=should_algorithm_buttons_be_disabled
                        set_algorithm=set_algorithm
                        current_algorithm=algorithm
                        select_image_onclick=select_image_onclick
                    />
                </div>
            </main>
        </div>
    }
}

fn get_scaled_image_dimensions_to_canvas(
    image_node: &HtmlImageElement,
    canvas: &HtmlCanvasElement,
) -> (f64, f64) {
    let canvas_client_width = canvas.client_width() as f64;
    let canvas_client_height = canvas.client_height() as f64;
    let image_width = image_node.width() as f64;
    let image_height = image_node.height() as f64;

    let width_scale = canvas_client_width as f64 / image_width as f64;
    let height_scale = canvas_client_height as f64 / image_height as f64;
    let scale = if width_scale < height_scale {
        width_scale
    } else {
        height_scale
    };

    let (new_width, new_height) =
        if canvas_client_width < image_width || canvas_client_height < image_height {
            (
                (image_width * scale).round(),
                (image_height * scale).round(),
            )
        } else {
            (image_width, image_height)
        };

    (new_width, new_height)
}
