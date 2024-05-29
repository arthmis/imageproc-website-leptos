mod views;
use js_sys::{Array, ArrayBuffer, JsString, Object, Reflect, Uint8ClampedArray};
use leptos::html::{Canvas, Img, Input, ToHtmlElement};
use leptos::leptos_dom::Text;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos::{component, create_signal, svg::view, view, IntoView};
use log::{error, info};
use shared::{Command, WorkerMessage};
use std::rc::Rc;
use std::str::FromStr;
use views::{BoxBlur, Gamma, Invert, SobelEdgeDetector};
use wasm_bindgen::JsValue;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData, MessageEvent, Url,
    Worker, WorkerOptions, WorkerType,
};

#[derive(Debug, Copy, Clone)]
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
    let file_input_ref = create_node_ref::<Input>();
    let image_ref = create_node_ref::<Img>();
    let selected_image_canvas = create_node_ref::<Canvas>();
    let modified_image_canvas = create_node_ref::<Canvas>();
    let gamma = create_rw_signal(1.);
    let invert = create_rw_signal(false);
    let box_blur_amount = create_rw_signal(1u32);
    let sobel_edge_detector_threshold = create_rw_signal(128u8);

    let on_message: Closure<dyn FnMut(MessageEvent)> =
        Closure::new(move |message_event: MessageEvent| {
            // let data = message.data().as_string().unwrap();
            let message = &Reflect::get(&message_event.data(), &JsValue::from_str("message"))
                .unwrap()
                .as_string()
                .unwrap();
            let worker_message = WorkerMessage::from_str(&message).unwrap();
            info!("worker message: {}", worker_message.to_string());
            match worker_message {
                WorkerMessage::Invert => {
                    info!("image was inverted, worker message");
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
                        // let image_data = Uint8ClampedArray::new(&image_data).to_vec();

                        info!("{:?}", &wasm_bindgen::Clamped(image_data.to_vec()));
                        ImageData::new_with_u8_clamped_array(
                            wasm_bindgen::Clamped(&image_data.to_vec()),
                            width,
                        )
                        .unwrap()
                    };
                    let selected_image = selected_image_canvas.get().unwrap();
                    let canvas_context = selected_image
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>()
                        .unwrap();
                    canvas_context
                        .put_image_data(&image_data, 0.0, 0.0)
                        .unwrap();
                }
                WorkerMessage::BoxBlur => {
                    info!("image box blur completed, worker message");
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

                        info!("{:?}", &wasm_bindgen::Clamped(image_data.to_vec()));
                        ImageData::new_with_u8_clamped_array(
                            wasm_bindgen::Clamped(&image_data.to_vec()),
                            width,
                        )
                        .unwrap()
                    };
                    let selected_image = selected_image_canvas.get().unwrap();
                    let canvas_context = selected_image
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>()
                        .unwrap();
                    canvas_context
                        .put_image_data(&image_data, 0.0, 0.0)
                        .unwrap();
                }
                WorkerMessage::Gamma => {
                    info!("image gamma transformation completed, worker message");
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
                        // let image_data = Uint8ClampedArray::new(&image_data).to_vec();

                        info!("{:?}", &wasm_bindgen::Clamped(image_data.to_vec()));
                        ImageData::new_with_u8_clamped_array(
                            wasm_bindgen::Clamped(&image_data.to_vec()),
                            width,
                        )
                        .unwrap()
                    };
                    let selected_image = selected_image_canvas.get().unwrap();
                    let canvas_context = selected_image
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>()
                        .unwrap();
                    canvas_context
                        .put_image_data(&image_data, 0.0, 0.0)
                        .unwrap();
                }
                WorkerMessage::SobelEdgeDetector => todo!(),
                WorkerMessage::DisplayOriginalImage => {
                    info!("original image was returned, worker message");
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
                        // let image_data = Uint8ClampedArray::new(&image_data).to_vec();

                        info!("{:?}", &wasm_bindgen::Clamped(&image_data.to_vec()));
                        ImageData::new_with_u8_clamped_array(
                            wasm_bindgen::Clamped(&image_data.to_vec()),
                            width,
                        )
                        .unwrap()
                    };
                    let selected_image = selected_image_canvas.get().unwrap();
                    let canvas_context = selected_image
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>()
                        .unwrap();
                    canvas_context
                        .put_image_data(&image_data, 0.0, 0.0)
                        .unwrap();
                }
                _ => {}
            }
            // info!("received response {:?}", &data);
        });

    let mut worker_options = WorkerOptions::new();
    worker_options.type_(WorkerType::Module);
    // look into using Refcell like in the rustwasm example
    let worker = Rc::new(Worker::new_with_options("./worker_loader.js", &worker_options).unwrap());
    worker.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    // let (worker, set_worker) = create_signal(worker);
    on_message.forget();
    let onload_worker = worker.clone();
    let invert_worker = worker.clone();
    let gamma_worker = worker.clone();
    // let (value, set_value) = create_signal(0);

    // let current_algorithm = move || if value() > 5 { "Big" } else { "Small" };

    let on_load = move |ev| {
        info!("{}", "image loaded");
        let image_node = image_ref.get().unwrap();
        let selected_image = selected_image_canvas.get().unwrap();
        info!(
            "ui thread canvas, width: {}, height: {}",
            selected_image.width(),
            selected_image.height()
        );
        let modified_image = {
            let modified_image = modified_image_canvas.get().unwrap();
            modified_image.set_width(selected_image.width());
            modified_image.set_height(selected_image.height());
            modified_image.transfer_control_to_offscreen().unwrap()
        };

        // let modified_image = selected_image
        //     .clone_node()
        //     .unwrap()
        //     .dyn_into::<HtmlCanvasElement>()
        //     .unwrap()
        //     .transfer_control_to_offscreen()
        //     .unwrap();

        let canvas_context = selected_image
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let ((new_width, new_height), (center_x, center_y)) =
            resize_image_for_canvas(&image_node, &selected_image);
        // need to clear canvas rect, if new image is smaller than the previous, or you will still see the old image
        canvas_context.clear_rect(
            0.0,
            0.0,
            selected_image.width() as f64,
            selected_image.height() as f64,
        );
        canvas_context
            .draw_image_with_html_image_element_and_dw_and_dh(
                &image_node,
                center_x,
                center_y,
                new_width,
                new_height,
            )
            .unwrap();

        let image_data = canvas_context
            .get_image_data(center_x, center_y, new_width, new_height)
            .unwrap();
        let raw_data = image_data.data();
        let raw_data = Uint8ClampedArray::from(raw_data.0.as_ref());
        let mut array: Array = Array::new();
        array.push(&raw_data.buffer());
        array.push(&modified_image);
        let mut message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("image_data"),
            &raw_data.buffer(),
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str("offscreen_canvas"),
            &modified_image,
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str("message"),
            &JsValue::from_str(Command::NewImage.to_string().as_str()),
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str("center_x"),
            &JsValue::from_f64(center_x),
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str("center_y"),
            &JsValue::from_f64(center_y),
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str("new_width"),
            &JsValue::from_f64(new_width),
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str("new_height"),
            &JsValue::from_f64(new_height),
        )
        .unwrap();

        onload_worker
            .post_message_with_transfer(&message, &array)
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
                    &JsValue::from_f64(gamma.get()),
                )
                .unwrap();
                worker.post_message(&message).unwrap();
            }
            Algorithm::Invert => {
                if invert.get() {
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
                        &JsValue::from_bool(invert.get()),
                    )
                    .unwrap();
                    worker.post_message(&message).unwrap();
                }
            }
            Algorithm::BoxBlur => {
                info!("box bluring");
                let mut message = Object::new();
                Reflect::set(
                    &message,
                    &JsValue::from_str("message"),
                    &JsValue::from_str(Command::BoxBlur.to_string().as_ref()),
                )
                .unwrap();
                Reflect::set(
                    &message,
                    &JsValue::from_str(Command::BoxBlur.to_string().as_ref()),
                    &JsValue::from_f64(box_blur_amount.get() as f64),
                )
                .unwrap();
                info!("sending box blur message: {}", box_blur_amount.get());
                worker.post_message(&message).unwrap();
            }
            Algorithm::SobelEdgeDetector => todo!(),
        },
        None => (),
    });
    let current_algorithm = move || match algorithm() {
        Some(current_algorithm) => match current_algorithm {
            Algorithm::Gamma => Some(view! {<Gamma gamma=gamma/>}),
            Algorithm::Invert => Some(view! {<Invert invert=invert/>}),
            Algorithm::BoxBlur => Some(view! {<BoxBlur box_blur_amount=box_blur_amount/>}),
            Algorithm::SobelEdgeDetector => Some(view! {<SobelEdgeDetector />}),
        },
        None => None,
    };

    view! {
        <NavBar/>
        <label for="file-selection" class="some-custom-css">
            "Select Image"
        </label>
        <input
            id="file-selection"
            class=""
            type="file"
            _ref=file_input_ref
            on:change=on_change
            style="display: none"
        />
        <img
            _ref=image_ref
            src=""
            style="display: none"
            on:load=on_load
        />

        <AlgorithmList set_algorithm=set_algorithm/>
        <canvas _ref=selected_image_canvas id="selected-image"></canvas>
        <canvas _ref=modified_image_canvas id="modified-image"></canvas>
        // <Gamma worker={gamma_worker}/>
        // <Invert worker={invert_worker} />
        // <p>{current_algorithm}</p>
        // <AlgorithmSelect algorithm=algorithm/>
        {current_algorithm}
        // <AlgorithmSelect algorithm=algorithm gamma_worker=gamma_worker invert_worker=invert_worker/>

    }
}

fn resize_image_for_canvas(
    image_node: &HtmlImageElement,
    canvas: &HtmlCanvasElement,
) -> ((f64, f64), (f64, f64)) {
    let canvas_offset_width = canvas.offset_width() as f64;
    let canvas_offset_height = canvas.offset_height() as f64;
    let image_width = image_node.width() as f64;
    let image_height = image_node.height() as f64;
    let width_scale = canvas.offset_width() as f64 / image_node.width() as f64;
    let height_scale = canvas.offset_height() as f64 / image_node.height() as f64;
    let scale = if width_scale < height_scale {
        width_scale
    } else {
        height_scale
    };
    let canvas_width_less_than_image_width = canvas_offset_width < image_width;
    let canvas_height_less_than_image_height = canvas_offset_height < image_height;

    let (new_width, new_height) =
        if canvas_width_less_than_image_width || canvas_height_less_than_image_height {
            (
                (image_width * scale).round(),
                (image_height * scale).round(),
            )
        } else {
            (image_width, image_height)
        };

    let center_x = (canvas_offset_width as f64 - new_width) / 2.0;
    let center_y = (canvas_offset_height as f64 - new_height) / 2.0;

    ((new_width, new_height), (center_x, center_y))
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <a href="index.html">
                <i class="fa fa-home" aria-hidden="true" style="font-size:2em;"></i>
            </a>

            <a href="https://github.com/arthmis/imageproc-website">
                <i class="fa fa-github" aria-hidden="true" style="font-size:1.4em;"></i>
                "Github"
            </a>
        </nav>
    }
}

#[component]
fn AlgorithmList(set_algorithm: WriteSignal<Option<Algorithm>>) -> impl IntoView {
    let algorithms = vec![
        Algorithm::Invert,
        Algorithm::Gamma,
        Algorithm::BoxBlur,
        Algorithm::SobelEdgeDetector,
    ];

    view! {
        <ul>
            {algorithms
                .into_iter()
                .map(|algorithm| {
                    view! {
                        <li>
                            <button on:click=move |_| {
                                info!("set algorithm: {}", algorithm);
                                set_algorithm(Some(algorithm));
                            }>{algorithm.to_string()}</button>
                        </li>
                    }
                })
                .collect::<Vec<_>>()}
        </ul>
    }
}

// #[component]
// fn AlgorithmView(algorithm: ReadSignal<Algorithm>, view: impl View) -> impl IntoView {
//     view! { <p>{algorithm}</p> }
// }
