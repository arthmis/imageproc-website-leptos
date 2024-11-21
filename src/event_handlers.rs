// use web_sys::Event;

// pub fn handle_image_load(ev: Event) {
//     info!("{}", "image loaded");
//     let image_node = image_ref.get().unwrap();
//     let canvas_wrapper_node = canvas_wrapper.get().unwrap();
//     let selected_image_canvas = selected_image_canvas.get().unwrap();

//     // let canvas_wrapper_width = canvas_wrapper_node.client_width();
//     // let canvas_wrapper_height = canvas_wrapper_node.client_height();
//     let selected_image_canvas = selected_image_canvas.style("width", "100%");
//     let selected_image_canvas = selected_image_canvas.style("height", "100%");
//     // let new_canvas_width = selected_image_canvas.client_width();
//     // let new_canvas_height = selected_image_canvas.client_height();
//     let new_canvas_width = selected_image_canvas.offset_width();
//     let new_canvas_height = selected_image_canvas.offset_height();
//     info!(
//         "selected_image_canvas client width: {}",
//         selected_image_canvas.client_width()
//     );
//     info!(
//         "selected_image_canvas client height: {}",
//         selected_image_canvas.client_height()
//     );

//     selected_image_canvas.set_width(new_canvas_width as u32);
//     // TODO: setting the height directly with using .offset_height() or any other height
//     // functions
//     // doesn't work correctly. However if I place the value into a variable first then it works
//     // no idea how this is happening
//     selected_image_canvas.set_height(new_canvas_width as u32);

//     let (scaled_width, scaled_height) =
//         get_scaled_image_dimensions_to_canvas(&image_node, &selected_image_canvas);

//     let new_canvas_width = selected_image_canvas.client_width();
//     let new_canvas_height = selected_image_canvas.client_height();

//     selected_image_canvas.set_width(new_canvas_width as u32);
//     // TODO: setting the height directly with using .offset_height() or any other height
//     // functions
//     // doesn't work correctly. However if I place the value into a variable first then it works
//     // no idea how this is happening
//     selected_image_canvas.set_height(new_canvas_height as u32);

//     let center_x = (selected_image_canvas.width() as f64 - scaled_width) / 2.;
//     let center_y = (selected_image_canvas.height() as f64 - scaled_height) / 2.;
//     let canvas_context = selected_image_canvas
//         .get_context("2d")
//         .unwrap()
//         .unwrap()
//         .dyn_into::<CanvasRenderingContext2d>()
//         .unwrap();
//     canvas_context
//         .draw_image_with_html_image_element_and_dw_and_dh(
//             &image_node,
//             center_x,
//             center_y,
//             scaled_width,
//             scaled_height,
//         )
//         .unwrap();

//     let image_data = canvas_context
//         .get_image_data(center_x, center_y, scaled_width, scaled_height)
//         .unwrap();

//     // reset current algorithm to be None for a new image
//     // reset algorithm values
//     // TODO look into making this a function or something
//     set_algorithm(None);
//     invert.set(false);
//     box_blur_amount.set(1);
//     gamma.set(1.);

//     let new_image_message = NewImageMessage::new(
//         Command::NewImage.to_string(),
//         image_data.data(),
//         center_x,
//         center_y,
//         scaled_width,
//         scaled_height,
//     );
//     let mut array = Array::new();
//     array.push(&new_image_message.js_clamped_uint8_array().buffer());

//     onload_worker
//         .post_message_with_transfer(&new_image_message.to_js_object(), &array)
//         .unwrap();
// }
