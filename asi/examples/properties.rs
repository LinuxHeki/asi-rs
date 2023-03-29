use asi::*;

fn main() {
    let num = number_of_connected_cameras();
    if num == 0 {
        println!("No connected cameras");
        return;
    }
    
    for i in 0..num {
        let info = camera_property(i).unwrap();
        let camera = Camera::open(info.camera_id).unwrap();

        let serial_number = camera.serial_number().unwrap();
        let mode = camera.camera_mode().unwrap();
        let supported_modes = camera.camera_supported_mode().unwrap();
        let gain_offset = camera.gain_offset().unwrap();
        let lmh_gain_offset = camera.lmh_gain_offset().unwrap();
        let roi_format = camera.roi_format().unwrap();

        println!("Camera #0: {}", info.name);
        println!("Serial number: {}", serial_number);
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
        for i in 0..camera.number_of_controls().unwrap() {
            let cap = camera.control_caps(i).unwrap();
            println!("\tName: {}", cap.name);
            println!("\tDescription: {}", cap.description);
            println!("\tMax value: {}", cap.max_value);
            println!("\tMin value: {}", cap.min_value);
            println!("\tDefault value: {}", cap.default_value);
            println!("\tAuto supported: {}", cap.is_auto_supported);
            println!("\tWritable: {}", cap.is_writable);
            println!("\n");
        }

        camera.close().unwrap();
    }
}
