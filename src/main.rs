mod app_state;
mod components;
mod effects;
mod event_handlers;
mod views;
use app_state::{Algorithm, AlgorithmInputState};
use components::algorithm_selection::AlgorithmList;
use components::navbar::NavBar;

use html::Div;
use js_sys::Array;
use leptos::html::{Canvas, Img, Input};
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos::{component, create_signal, view, IntoView};
use log::info;
use shared::{
    BoxBlurMessage, Command, GammaMessage, InvertMessage, NewImageMessage,
    SobelEdgeDetectionMessage, ToJsObject,
};
use views::{CurrentAlgorithm, InvisibleSelectFile};
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::{
    window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, MediaQueryListEvent,
};

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
    let should_algorithm_buttons_be_disabled = Signal::derive(move || image_url.get().is_empty());
    let image_ref = create_node_ref::<Img>();
    let canvas_wrapper = create_node_ref::<Div>();
    let selected_image_canvas = create_node_ref::<Canvas>();
    let algorithm_state = AlgorithmInputState::default();
    let gamma = algorithm_state.gamma();
    let invert = algorithm_state.invert();
    let box_blur_amount = algorithm_state.box_blur_amount();
    let sobel_edge_detector_threshold = algorithm_state.sobel_edge_detector_threshold();

    let worker = effects::use_worker(selected_image_canvas);
    let onload_worker = worker.clone();

    let handle_image_load = move |_ev| {
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
        set_algorithm.set(None);
        algorithm_state.reset();

        let new_image_message = NewImageMessage::new(
            Command::NewImage.to_string(),
            image_data.data(),
            center_x,
            center_y,
            scaled_width,
            scaled_height,
        );
        let array: Array = Array::new();
        array.push(&new_image_message.js_clamped_uint8_array().buffer());

        onload_worker
            .post_message_with_transfer(&new_image_message.to_js_object(), &array)
            .unwrap();
    };

    Effect::new(move |_| {
        let image_node = image_ref.get().unwrap();
        image_node.set_src(&image_url.get());
    });

    create_effect(move |_| {
        if let Some(current_algorithm) = algorithm.get() {
            let message = match current_algorithm {
                Algorithm::Gamma => {
                    GammaMessage::new(Command::Gamma.to_string(), gamma.get()).to_js_object()
                }
                Algorithm::Invert => {
                    InvertMessage::new(Command::Invert.to_string(), invert.get()).to_js_object()
                }
                Algorithm::BoxBlur => {
                    BoxBlurMessage::new(Command::BoxBlur.to_string(), box_blur_amount.get())
                        .to_js_object()
                }
                Algorithm::SobelEdgeDetector => SobelEdgeDetectionMessage::new(
                    Command::SobelEdgeDetector.to_string(),
                    sobel_edge_detector_threshold.get(),
                )
                .to_js_object(),
            };
            worker.post_message(&message).unwrap();
        }
    });

    let file_input_ref = create_node_ref::<Input>();
    let select_image_onclick = move |_event| {
        if let Some(node) = file_input_ref.get() {
            node.click();
        }
    };

    let query = "(min-width: 1024px)";
    let media_query = window().unwrap().match_media(query).unwrap().unwrap();
    let (is_screen_desktop_size, set_is_screen_desktop_size) = create_signal(media_query.matches());

    let on_screen_width_change: Closure<dyn FnMut(MediaQueryListEvent)> =
        Closure::new(move |event: MediaQueryListEvent| {
            // only gets called if the size changes from desktop to mobile or whatever i specified and vice versa
            info!("{:?}", event);
            set_is_screen_desktop_size.set(event.matches());
        });
    // put this in a create_effect
    match media_query
        .add_event_listener_with_callback("change", on_screen_width_change.as_ref().unchecked_ref())
    {
        Ok(_) => on_screen_width_change.forget(),
        Err(_) => log::error!("error setting up listener to observe screen width changes"),
    }

    let mobile_select_image_button = move || {
        if !is_screen_desktop_size.get() {
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
                    <InvisibleSelectFile file_input_ref=file_input_ref set_image_url=set_image_url/>
                    {mobile_select_image_button}
                </div>
                <img _ref=image_ref src="" style="display: none" on:load=handle_image_load/>

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
                        <CurrentAlgorithm
                            gamma=gamma
                            invert=invert
                            box_blur_amount=box_blur_amount
                            sobel_edge_detector_threshold=sobel_edge_detector_threshold
                            algorithm=algorithm
                        />
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

    let width_scale = canvas_client_width / image_width;
    let height_scale = canvas_client_height / image_height;
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
