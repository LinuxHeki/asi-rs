use asi::*;

fn main() {
    let num = get_num_of_connected_cameras();
    if num == 0 {
        println!("No connected cameras");
        return;
    }

    let camera_info = get_camera_property(0).unwrap();

    let width = camera_info.max_width;
    let height = camera_info.max_height;
    let color = camera_info.is_color_cam;

    open_camera(camera_info.camera_id).unwrap();
    init_camera(camera_info.camera_id).unwrap();

    if color {
        set_roi_format(camera_info.camera_id, width, height, 1, ImgType::Rgb24).unwrap();
    } else {
        set_roi_format(camera_info.camera_id, width, height, 1, ImgType::Raw8).unwrap();
    }

    start_exposure(camera_info.camera_id, false).unwrap();
    while get_exp_status(camera_info.camera_id).unwrap() == ExposureStatus::Working {}

    let buffer_size = width*height*(1+2*color as i32);
    let mut data = vec![0; buffer_size as usize];
    get_data_after_exp(camera_info.camera_id, &mut data).unwrap();
    
    let color = match color {
        true => image::ColorType::Rgb8,
        false => image::ColorType::L8,
    };

    image::save_buffer("exposure.jpg", &data, width as u32, height as u32, color).unwrap();

    close_camera(camera_info.camera_id).unwrap();
}
