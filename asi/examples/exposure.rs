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

    let camera = Camera::open(camera_info.camera_id).unwrap();

    if color {
        camera.set_roi_format(width, height, 1, ImgType::Rgb24).unwrap();
    } else {
        camera.set_roi_format(width, height, 1, ImgType::Raw8).unwrap();
    }

    camera.start_exposure(false).unwrap();
    while camera.exposure_status().unwrap() == ExposureStatus::Working {}

    let buffer_size = width*height*(1+2*color as u32);
    let mut data = vec![0; buffer_size as usize];
    camera.get_data_after_exposure(&mut data).unwrap();
    
    let color = match color {
        true => image::ColorType::Rgb8,
        false => image::ColorType::L8,
    };

    image::save_buffer("exposure.jpg", &data, width as u32, height as u32, color).unwrap();

    camera.close().unwrap();
}
