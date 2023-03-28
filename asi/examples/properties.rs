use asi::*;

fn main() {
    let num = get_num_of_connected_cameras();
    if num == 0 {
        println!("No connected cameras");
        return;
    }
    
    for i in 0..num {
        open_camera(i).unwrap();
        let info = get_camera_property(i).unwrap();
        let id = info.camera_id;

        let sn = get_serial_number(id).unwrap();
        let mode = get_camera_mode(id).unwrap();
        let supported_modes = get_camera_support_mode(id).unwrap();
        let gain_offset = get_gain_offset(id).unwrap();
        let lmh_gain_offset = get_lmh_gain_offset(id).unwrap();
        let roi_format = get_roi_format(id).unwrap();

        println!("Camera #0: {}", info.name);
        println!("Serial number: {}", hex::encode(sn.id));
        println!("Max height: {}", info.max_height);
        println!("Max width: {}", info.max_width);
        println!("Color cam: {}", info.is_color_cam);
        println!("Bayer pattern: {:?}", info.bayer_pattern);
        println!("Supported bins: {:?}", info.supported_bins);
        println!("Supported video formats: {:?}", info.supported_video_formats);
        println!("Pixel size: {}", info.pixel_size);
        println!("Mechanical shutter: {}", info.mechanical_shutter);
        println!("ST4 port: {}", info.st4_port);
        println!("Cooler cam: {}", info.is_cooler_cam);
        println!("USB3 host: {}", info.is_usb3_host);
        println!("USB3 camera: {}", info.is_usb3_camera);
        println!("Elec/ADU: {}", info.elec_per_adu);
        println!("Bit depth: {}", info.bit_depth);
        println!("Trigger cam: {}", info.is_trigger_cam);

        println!("Camera mode: {:?}", mode);
        println!("Supported camera modes: {:?}", supported_modes);
        println!("Gain offset: {:?}", gain_offset);
        println!("LMH gain offset: {:?}", lmh_gain_offset);
        println!("ROI format: {:?}", roi_format);
        
        println!("Control caps:");
        for i in 0..get_num_of_controls(id).unwrap() {
            let cap = get_control_caps(id, i).unwrap();
            println!("\tName: {}", cap.name);
            println!("\tDescription: {}", cap.description);
            println!("\tMax value: {}", cap.max_value);
            println!("\tMin value: {}", cap.min_value);
            println!("\tDefault value: {}", cap.default_value);
            println!("\tAuto supported: {}", cap.is_auto_supported);
            println!("\tWritable: {}", cap.is_writable);
            println!("\n");
        }

        close_camera(id).unwrap();
    }
}
