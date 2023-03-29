use asi::*;

fn main() {
    let num = number_of_connected_cameras();
    if num == 0 {
        println!("No connected cameras");
        return;
    }

    let camera_info = camera_property(0).unwrap();

    let width = camera_info.max_width;
    let height = camera_info.max_height;
    let color = camera_info.is_color_cam;
    let bit_depth = camera_info.bit_depth > 1;

    let camera = Camera::open(camera_info.camera_id).unwrap();

    if color {
        camera.set_roi_format(width, height, 1, ImgType::Rgb24).unwrap();
    } else {
        camera.set_roi_format(width, height, 1, ImgType::Raw8).unwrap();
    }

    camera.start_exposure(false).unwrap();
    while camera.exposure_status().unwrap() == ExposureStatus::Working {}

    let mut buffer_size = width*height;
    if color {
        buffer_size *= 3;
    } else if bit_depth {
        buffer_size *= 2;
    }

    let mut data = vec![0; buffer_size as usize];
    camera.get_data_after_exposure(&mut data).unwrap();
    
    let color = match (color, bit_depth) {
        (true, _) => image::ColorType::Rgb8,
        (false, true) => image::ColorType::L16,
        (false, false) => image::ColorType::L8,
    };

    image::save_buffer("exposure.jpg", &data, width as u32, height as u32, color).unwrap();
    camera.close().unwrap();
}
