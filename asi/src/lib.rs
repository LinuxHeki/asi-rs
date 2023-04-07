use std::ffi::CString;

use asi_sys::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BayerPattern {
    Rg,
    Bg,
    Gr,
    Gb,
}

impl From<u32> for BayerPattern {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Rg,
            1 => Self::Bg,
            2 => Self::Gr,
            3 => Self::Gb,
            i => panic!("Invalid bayer pattern: {}", i),
        }
    }
}

/// Supported Video Format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImgType {
    Raw8,
    Rgb24,
    Raw16,
    Y8,
}

impl From<i32> for ImgType {
    fn from(img_type: i32) -> Self {
        match img_type {
            0 => Self::Raw8,
            1 => Self::Rgb24,
            2 => Self::Raw16,
            3 => Self::Y8,
            i => panic!("Invalid image type: {}", i),
        }
    }
}

/// Guider Direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuideDirection {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlipStatus {
    None,
    Horizontal,
    Vertical,
    Both,
}

impl From<i32> for FlipStatus {
    fn from(flip_status: i32) -> Self {
        match flip_status {
            0 => FlipStatus::None,
            1 => FlipStatus::Horizontal,
            2 => FlipStatus::Vertical,
            3 => FlipStatus::Both,
            i => panic!("Invalid flip status: {}", i),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraMode {
    Normal,
    SoftEdge,
    RiseEdge,
    FallEdge,
    SoftLevel,
    HighLevel,
    LowLevel,
}

impl From<i32> for CameraMode {
    fn from(camera_mode: i32) -> Self {
        match camera_mode {
            0 => Self::Normal,
            1 => Self::SoftEdge,
            2 => Self::RiseEdge,
            3 => Self::FallEdge,
            4 => Self::SoftLevel,
            5 => Self::HighLevel,
            6 => Self::LowLevel,
            i => panic!("Invalid camera mode: {}", i),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrigOutput {
    /// Only pin A output
    PinA,
    /// Only pin B output
    PinB,
    None = -1,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorCode {
    Success,
    /// No camera connected or index value out of boundary.
    InvalidIndex,
    /// Invalid ID
    InvalidId,
    /// Invalid control type
    InvalidControlType,
    /// Camera didn't open.
    CameraClosed,
    /// Failed to find the camera, maybe the camera has been removed.
    CameraRemoved,
    /// Cannot find the path of the file.
    InvalidPath,
    InvalidFileFormat,
    /// Wrong video format size
    InvalidSize,
    /// Unsupported image formate
    InvalidImgType,
    /// The startpos is out of boundary.
    OutOfBoundary,
    /// Timeout
    Timeout,
    /// Stop capture first.
    InvalidSequence,
    /// Buffer size is not big enough.
    BufferTooSmall,
    VideoModeActive,
    ExposureInProgress,
    /// General error, eg: value is out of valid range.
    GeneralError,
    /// The current mode is wrong.
    InvalidMode,
    Unknown,
}

impl From<i32> for ErrorCode {
    fn from(error: i32) -> Self {
        match error {
            0 => Self::Success,
            1 => Self::InvalidIndex,
            2 => Self::InvalidId,
            3 => Self::InvalidControlType,
            4 => Self::CameraClosed,
            5 => Self::CameraRemoved,
            6 => Self::InvalidPath,
            7 => Self::InvalidFileFormat,
            8 => Self::InvalidSize,
            9 => Self::InvalidImgType,
            10 => Self::OutOfBoundary,
            11 => Self::Timeout,
            12 => Self::InvalidSequence,
            13 => Self::BufferTooSmall,
            14 => Self::VideoModeActive,
            15 => Self::ExposureInProgress,
            16 => Self::GeneralError,
            17 => Self::InvalidMode,
            _ => Self::Unknown,
        }
    }
}

impl ErrorCode {
    pub fn to_result<T>(self, value: T) -> Result<T, ErrorCode> {
        if self == Self::Success {
            Ok(value)
        } else {
            Err(self)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CameraInfo {
    /// The name of the camera.
    pub name: String,
    /// This is used to control everything of the camera in other functions. Start from 0.
    pub camera_id: u8,
    /// The max height of the camera.
    pub max_height: u32,
    /// The max width of the camera.
    pub max_width: u32,
    pub is_color_cam: bool,
    pub bayer_pattern: BayerPattern,
    /// 1 means bin1 which is supported by every camera, 2 means bin 2 etc..
    pub supported_bins: Vec<u32>,
    /// This array will content with the support output format type.IMG_END is the end of supported video format.
    pub supported_video_formats: Vec<ImgType>,
    /// The pixel size of the camera, unit is um. such like 5.6um.
    pub pixel_size: f32,
    pub mechanical_shutter: bool,
    pub st4_port: bool,
    pub is_cooler_cam: bool,
    pub is_usb3_host: bool,
    pub is_usb3_camera: bool,
    pub elec_per_adu: f32,
    pub bit_depth: u32,
    pub is_trigger_cam: bool,
}

impl From<ASI_CAMERA_INFO> for CameraInfo {
    fn from(info: ASI_CAMERA_INFO) -> Self {
        Self {
            name: unsafe {std::str::from_utf8_unchecked(std::mem::transmute(info.Name.as_slice())).to_string()},
            camera_id: info.CameraID as u8,
            max_height: info.MaxHeight as u32,
            max_width: info.MaxWidth as u32,
            is_color_cam: info.IsColorCam == 1,
            bayer_pattern: BayerPattern::from(info.BayerPattern),
            supported_bins: {
                let mut bins = Vec::new();
                info.SupportedBins.iter().cloned().take_while(|&x| x != 0).for_each(|x| bins.push(x as u32));
                bins
            },
            supported_video_formats: {
                let mut formats = Vec::new();
                info.SupportedVideoFormat.iter().cloned().take_while(|&x| x != -1).for_each(|x| { formats.push(ImgType::from(x)); });
                formats
            },
            pixel_size: info.PixelSize as f32,
            mechanical_shutter: info.MechanicalShutter == 1,
            st4_port: info.ST4Port == 1,
            is_cooler_cam: info.IsCoolerCam == 1,
            is_usb3_host: info.IsUSB3Host == 1,
            is_usb3_camera: info.IsUSB3Camera == 1,
            elec_per_adu: info.ElecPerADU,
            bit_depth: info.BitDepth as u32,
            is_trigger_cam: info.IsTriggerCam == 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlType {
    Gain,
    Exposure,
    Gamma,
    WbR,
    WbB,
    Offset,
    BandwidthOverflow,
    Overclock,
    /// Returns 10*temperature.
    Temperature,
    /// Use like this: ```FlipStatus::Both as i32```
    Flip,
    AutoMaxGain,
    /// In microseconds
    AutoMaxExp,
    /// Target brightness
    AutoTargetBrightness,
    HardwareBin,
    HighSpeedMode,
    CoolerPowerPerc,
    /// No need for *10.
    TargetTemp,
    CoolerOn,
    /// Leads to less grid at software bin mode for color camera.
    MonoBin,
    FanOn,
    PatternAdjust,
    AntiDewHeater,
    Gps,
}

impl From<u32> for ControlType {
    fn from(control_type: u32) -> Self {
        match control_type {
            0 => ControlType::Gain,
            1 => ControlType::Exposure,
            2 => ControlType::Gamma,
            3 => ControlType::WbR,
            4 => ControlType::WbB,
            5 => ControlType::Offset,
            6 => ControlType::BandwidthOverflow,
            7 => ControlType::Overclock,
            8 => ControlType::Temperature,
            9 => ControlType::Flip,
            10 => ControlType::AutoMaxGain,
            11 => ControlType::AutoMaxExp,
            12 => ControlType::AutoTargetBrightness,
            13 => ControlType::HardwareBin,
            14 => ControlType::HighSpeedMode,
            15 => ControlType::CoolerPowerPerc,
            16 => ControlType::TargetTemp,
            17 => ControlType::CoolerOn,
            18 => ControlType::MonoBin,
            19 => ControlType::FanOn,
            20 => ControlType::PatternAdjust,
            21 => ControlType::AntiDewHeater,
            22 => ControlType::Gps,
            i => panic!("Invalid control type: {}", i),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ControlCaps {
    /// The name of the Control like Exposure, Gain etc..
    pub name: String,
    /// Description of this control.
    pub description: String,
    pub max_value: i32,
    pub min_value: i32,
    pub default_value: i32,
    /// Support auto set 1, don't support 0.
    pub is_auto_supported: bool,
    /// Some control like temperature can only be read by some cameras.
    pub is_writable: bool,
    /// This is used to get value and set value of the control.
    pub control_type: ControlType,
}

impl From<ASI_CONTROL_CAPS> for ControlCaps {
    fn from(caps: ASI_CONTROL_CAPS) -> Self {
        Self {
            name: unsafe {std::str::from_utf8_unchecked(std::mem::transmute(caps.Name.as_slice())).to_string()},
            description: unsafe {std::str::from_utf8_unchecked(std::mem::transmute(caps.Description.as_slice())).to_string()},
            max_value: caps.MaxValue as i32,
            min_value: caps.MinValue as i32,
            default_value: caps.DefaultValue as i32,
            is_auto_supported: caps.IsAutoSupported == 1,
            is_writable: caps.IsWritable == 1,
            control_type: ControlType::from(caps.ControlType),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExposureStatus {
    /// Idle states, you can start exposure now.
    Idle,
    /// Exposing.
    Working,
    /// Exposure finished and waiting for download.
    Success,
    /// Exposure failed, you need to start exposure again.
    Failed,
}

impl From<u32> for ExposureStatus {
    fn from(exposure_status: u32) -> Self {
        match exposure_status {
            0 => Self::Idle,
            1 => Self::Working,
            2 => Self::Success,
            3 => Self::Failed,
            i => panic!("Invalid exposure status: {}", i),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Id {
    pub id: [u8; 8]
}

impl From<ASI_ID> for Id {
    fn from(id: ASI_ID) -> Self {
        Self { id: id.id }
    }
}

impl Id {
    pub fn to_asi_id(&self) -> ASI_ID {
        ASI_ID {
            id: self.id
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupportedMode {
    /// This vector will content with the support camera mode type.
    pub camera_mode: Vec<CameraMode>,
}

impl From<ASI_SUPPORTED_MODE> for SupportedMode {
    fn from(supported_mode: ASI_SUPPORTED_MODE) -> Self {
        let mut modes = Vec::new();
        for mode in supported_mode.SupportedCameraMode {
            if mode == -1 {break;}
            modes.push(CameraMode::from(mode));
        }

        Self { camera_mode: modes }
    }
}

#[derive(Debug)]
pub struct Camera {
    camera_id: u8
}

impl From<u8> for Camera {
    /// Get Camera struct without opening or initializing the camera.
    fn from(id: u8) -> Self {
        Camera { camera_id: id }
    }
}

impl Camera {
    /// Opens and initializes the camera.
    pub fn open(camera_id: u8) -> Result<Self, ErrorCode> {
        let error = unsafe {ASIOpenCamera(camera_id.into())};
        ErrorCode::from(error).to_result(())?;

        let error = unsafe {ASIInitCamera(camera_id.into())};
        let camera = Camera { camera_id };
        ErrorCode::from(error).to_result(camera)
    }

    /// Close the camera to free all the resource.
    pub fn close(self) -> Result<(), ErrorCode> {
        let error = unsafe {ASICloseCamera(self.camera_id.into())};
        ErrorCode::from(error).to_result(())
    }

    /// Get number of controls available for this camera.
    pub fn number_of_controls(&self) -> Result<usize, ErrorCode> {
        let mut num = 0;
        let error = unsafe {ASIGetNumOfControls(self.camera_id.into(), &mut num)};
        ErrorCode::from(error).to_result(num as usize)
    }

    /// Get controls property available for this camera.
    pub fn control_caps(&self, control_index: usize) -> Result<ControlCaps, ErrorCode> {
        let mut control_caps = ASI_CONTROL_CAPS::default();
        let status = unsafe {ASIGetControlCaps(self.camera_id.into(), control_index as i32, &mut control_caps)};
        ErrorCode::from(status).to_result(ControlCaps::from(control_caps))
    }

    /// Get controls property value and auto value. Returns the value and if it is writtable or not.
    /// 
    /// For ```ControlType::Flip``` convert it to ```FlipStatus``` with ```FlipStatus::from(value)```.
    pub fn control_value(&self, control_type: ControlType) -> Result<(isize, bool), ErrorCode> {
        let (mut value, mut auto) = (0, 0);
        let status = unsafe {ASIGetControlValue(self.camera_id.into(), control_type as i32, &mut value, &mut auto)};
        ErrorCode::from(status).to_result((value as isize, auto == 1))
    }

    /// Set controls property value and auto value.
    pub fn set_control_value(&self, control_type: ControlType, value: i32, auto: bool) -> Result<(), ErrorCode> {
        let status = unsafe {ASISetControlValue(self.camera_id.into(), control_type as i32, value.into(), auto.into())};
        ErrorCode::from(status).to_result(())
    }

    /// Get the current ROI area setting.
    pub fn roi_format(&self) -> Result<(u32, u32, i32, ImgType), ErrorCode> {
        let (mut width, mut height, mut bin, mut img_type) = (0, 0, 0, ASI_IMG_TYPE::default());
        let status = unsafe {ASIGetROIFormat(self.camera_id.into(), &mut width, &mut height, &mut bin, &mut img_type)};
        ErrorCode::from(status).to_result((width as u32, height as u32, bin, ImgType::from(img_type)))
    }

    /// Set the ROI area before capture.
    /// You must stop the capture before call it.
    /// The width and height is the value after binning.
    pub fn set_roi_format(&self, width: u32, height: u32, bin: u32, img_type: ImgType) -> Result<(), ErrorCode> {
        let status = unsafe {ASISetROIFormat(self.camera_id.into(), width as i32, height as i32, bin as i32, img_type as i32)};
        ErrorCode::from(status).to_result(())
    }

    /// Get the start position of current ROI area.
    pub fn start_position(&self) -> Result<(u32, u32), ErrorCode> {
        let (mut start_x, mut start_y) = (0, 0);
        let status = unsafe {ASIGetStartPos(self.camera_id.into(), &mut start_x, &mut start_y)};
        ErrorCode::from(status).to_result((start_x as u32, start_y as u32))
    }

    /// Set the start position of the ROI area.
    /// You can call this API to move the ROI area when video is streaming.
    /// The camera will set the ROI area to the center of the full image as default.
    /// At bin2 or bin3 mode, the position is relative to the image after binning.
    pub fn set_start_position(&self, start_x: u32, start_y: u32) -> Result<(), ErrorCode> {
        let status = unsafe {ASISetStartPos(self.camera_id.into(), start_x as i32, start_y as i32)};
        ErrorCode::from(status).to_result(())
    }

    /// Get the dropped frames.
    /// Dropped frames happen when USB traffic or harddisk write speed is slow.
    /// It will reset to 0 after stop capture.
    pub fn get_dropped_frames(&self) -> Result<u32, ErrorCode> {
        let mut dropped_frames = 0;
        let status = unsafe {ASIGetDroppedFrames(self.camera_id.into(), &mut dropped_frames)};
        ErrorCode::from(status).to_result(dropped_frames as u32)
    }

    /// Provide a dark file's path to the function and enable dark subtract.
    /// This is used when there is hot pixel or need to do long exposure.
    /// You'd better make this dark file from the "dark subtract" funtion 
    /// of the "video capture filter" directshow page.
    /// The dark file's size should be the same of camera's max width and height 
    /// and should be RGB8 raw format. It will be on even if you change the ROI setting.
    /// It only corrects hot pixels if output isn't 16bit.
    /// It will be remembered in registry, so "Dark subtract" is on next time if you close your app.
    pub fn enable_dark_subtract(&self, path: &str) -> Result<(), ErrorCode> {
        let path = CString::new(path).unwrap().into_raw();
        let status = unsafe {ASIEnableDarkSubtract(self.camera_id.into(), path)};
        ErrorCode::from(status).to_result(())
    }

    /// Disable the dark subtract function.
    /// You'd better call it at start if you don't want to use it,
    /// because dark subtract function is remembered on windows platform.
    pub fn disable_dark_subtract(&self) -> Result<(), ErrorCode> {
        let status = unsafe {ASIDisableDarkSubtract(self.camera_id.into())};
        ErrorCode::from(status).to_result(())
    }

    /// Start video capture.
    /// Then you can get the data from function get_video_data.
    pub fn start_video_capture(&self) -> Result<(), ErrorCode> {
        let status = unsafe {ASIStartVideoCapture(self.camera_id.into())};
        ErrorCode::from(status).to_result(())
    }

    /// Stop video capture.
    pub fn stop_video_capture(&self) -> Result<(), ErrorCode> {
        let status = unsafe {ASIStopVideoCapture(self.camera_id.into())};
        ErrorCode::from(status).to_result(())
    }

    /// Get data from the video buffer. The buffer is very small.
    /// You need to call this API as fast as possible, otherwise frame will be discarded.
    /// The best way is maintain one buffer loop and call this API in a loop.
    /// Please make sure the buffer size is big enough to hold one image
    /// otherwise the this API will crash.
    pub fn get_video_data(&self, buffer: &mut [u8], wait_ms: u32) -> Result<(), ErrorCode> {
        let status = unsafe {ASIGetVideoData(self.camera_id.into(), buffer.as_mut_ptr(), (buffer.len() as i32).into(), wait_ms as i32)};
        ErrorCode::from(status).to_result(())
    }

    /// PulseGuide of the ST4 port on. This function only works on modules which have ST4 port.
    pub fn pulse_guide_on(&self, direction: GuideDirection) -> Result<(), ErrorCode> {
        let status = unsafe {ASIPulseGuideOn(self.camera_id.into(), direction as i32)};
        ErrorCode::from(status).to_result(())
    }

    /// PulseGuide of the ST4 port off. This function only works on modules which have ST4 port.
    pub fn pulse_guide_off(&self, direction: GuideDirection) -> Result<(), ErrorCode> {
        let status = unsafe {ASIPulseGuideOff(self.camera_id.into(), direction as i32)};
        ErrorCode::from(status).to_result(())
    }

    /// Start camera exposure.
    /// Start exposure and check the exposure status then get the data.
    /// ```is_dark``` means dark frame if there is mechanical shutter on the camera otherwise useless.
    pub fn start_exposure(&self, is_dark: bool) -> Result<(), ErrorCode> {
        let status = unsafe {ASIStartExposure(self.camera_id.into(), is_dark as i32)};
        ErrorCode::from(status).to_result(())
    }

    /// To cancel the long exposure which is on.
    pub fn stop_exposure(&self) -> Result<(), ErrorCode> {
        let status = unsafe {ASIStopExposure(self.camera_id.into())};
        ErrorCode::from(status).to_result(())
    }

    /// To get the exposure status, work with start_exposure.
    /// You can read the data if you get ```ExposureStatus::Success``` or you have to restart exposure again
    /// if you get ```ExposureStatus::Failed```
    pub fn exposure_status(&self) -> Result<ExposureStatus, ErrorCode> {
        let mut exposure_status = ASI_EXPOSURE_STATUS::default();
        let status = unsafe {ASIGetExpStatus(self.camera_id.into(), &mut exposure_status)};
        ErrorCode::from(status).to_result(ExposureStatus::from(exposure_status))
    }

    /// Get data after exposure.
    /// Please make sure the buffer size is big enough to hold one image
    /// otherwise the this API will crash.
    pub fn get_data_after_exposure(&self, buffer: &mut [u8]) -> Result<(), ErrorCode> {
        let status = unsafe {ASIGetDataAfterExp(self.camera_id.into(), buffer.as_mut_ptr(), (buffer.len() as i32).into())};
        ErrorCode::from(status).to_result(())
    }

    /// Get camera id stored in flash, only available for USB3.0 cameras.
    pub fn id(&self) -> Result<Id, ErrorCode> {
        let mut id = ASI_ID::default();
        let status = unsafe {ASIGetID(self.camera_id.into(), &mut id)};
        ErrorCode::from(status).to_result(Id::from(id))
    }

    /// Write camera id to flash, only available for USB3.0 cameras.
    pub fn set_id(&self, id: Id) -> Result<(), ErrorCode> {
        let status = unsafe {ASISetID(self.camera_id.into(), id.to_asi_id())};
        ErrorCode::from(status).to_result(())
    }

    /// Get pre-setting parameter.
    pub fn gain_offset(&self) -> Result<(u32, u32, u32, u32), ErrorCode> {
        let (mut off_hig_dr, mut off_unity_gain, mut gain_low_rn, mut off_low_rn) = (0, 0, 0, 0);
        let status = unsafe {ASIGetGainOffset(self.camera_id.into(), &mut off_hig_dr, &mut off_unity_gain, &mut gain_low_rn, &mut off_low_rn)};
        ErrorCode::from(status).to_result((off_hig_dr as u32, off_unity_gain as u32, gain_low_rn as u32, off_low_rn as u32))
    }

    /// Get the frequently-used gain and offset.
    pub fn lmh_gain_offset(&self) -> Result<(u32, u32, u32, u32), ErrorCode> {
        let (mut l_gain, mut m_gain, mut h_gain, mut h_offset) = (0, 0, 0, 0);
        let status = unsafe {ASIGetLMHGainOffset(self.camera_id.into(), &mut l_gain, &mut m_gain, &mut h_gain, &mut h_offset)};
        ErrorCode::from(status).to_result((l_gain as u32, m_gain as u32, h_gain as u32, h_offset as u32))
    }

    /// Get the camera supported mode, only needs to call when the ```is_trigger_cam``` in the ```CameraInfo``` is ```true```.
    pub fn camera_supported_mode(&self) -> Result<SupportedMode, ErrorCode> {
        let mut supported_mode = ASI_SUPPORTED_MODE::default();
        let status = unsafe {ASIGetCameraSupportMode(self.camera_id.into(), &mut supported_mode)};
        ErrorCode::from(status).to_result(SupportedMode::from(supported_mode))
    }

    /// Get the camera current mode, only needs to call when the ```is_trigger_cam``` in the ```CameraInfo``` is ```true```.
    pub fn camera_mode(&self) -> Result<CameraMode, ErrorCode> {
        let mut camera_mode = ASI_CAMERA_MODE::default();
        let status = unsafe {ASIGetCameraMode(self.camera_id.into(), &mut camera_mode)};
        ErrorCode::from(status).to_result(CameraMode::from(camera_mode))
    }

    /// Set the camera mode, only needs to call when the ```is_trigger_cam``` in the ```CameraInfo``` is ```true```.
    pub fn set_camera_mode(&self, camera_mode: CameraMode) -> Result<(), ErrorCode> {
        let status = unsafe {ASISetCameraMode(self.camera_id.into(), camera_mode as i32)};
        ErrorCode::from(status).to_result(())
    }

    /// Send out a softTrigger. For edge trigger, it only needs to set true which means send a
    /// rising trigger to start exposure. For level trigger, it needs to set true first means 
    /// start exposure, and set false means stop exposure. It only needs to call when the 
    /// ```is_trigger_cam``` in the ```CameraInfo``` is ```true```.
    pub fn send_soft_trigger(&self, start: bool) -> Result<(), ErrorCode> {
        let status = unsafe {ASISendSoftTrigger(self.camera_id.into(), start as i32)};
        ErrorCode::from(status).to_result(())
    }

    /// Get a serial number from the camera.
    pub fn serial_number(&self) -> Result<String, ErrorCode> {
        let mut sn = ASI_SN::default();
        let status = unsafe {ASIGetSerialNumber(self.camera_id.into(), &mut sn)};
        let serial_number = hex::encode(sn.id);
        ErrorCode::from(status).to_result(serial_number)
    }

    /// Get the output pin configuration, it only needs to call when the is_trigger_cam in the CameraInfo is true.
    pub fn trigger_output_io_conf(&self, pin: TrigOutput) -> Result<(bool, usize, usize), ErrorCode> {
        let (mut pin_high, mut delay, mut duration) = (0, 0, 0);
        let status = unsafe {ASIGetTriggerOutputIOConf(self.camera_id.into(), pin as i32, &mut pin_high, &mut delay, &mut duration)};
        ErrorCode::from(status).to_result((pin_high == 1, delay as usize, duration as usize))
    }

    /// Config the output pin (A or B) of Trigger port. If duration <= 0, this output pin will be closed. 
    /// It only needs to call when the is_trigger_cam in the CameraInfo is true.
    pub fn set_trigger_output_io_conf(&self, pin: TrigOutput, pin_high: bool, delay: usize, duration: usize) -> Result<(), ErrorCode> {
        let status = unsafe {ASISetTriggerOutputIOConf(self.camera_id.into(), pin as i32, pin_high as i32, (delay as i32).into(), (duration as i32).into())};
        ErrorCode::from(status).to_result(())
    }
}

/// This should be the first API to be called.
/// Get number of connected ASI cameras.
pub fn number_of_connected_cameras() -> u8 {
    unsafe {ASIGetNumOfConnectedCameras() as u8}
}

/// Get the product ID of each supported camera.
pub fn product_ids() -> Vec<i32> {
    let mut pids = Vec::new();
    unsafe {ASIGetProductIDs(pids.as_mut_ptr())};
    pids
}

/// Check if the device is ASI Camera.
pub fn camera_check(vid: i32, pid: i32) -> bool {
    unsafe {ASICameraCheck(vid, pid) == 1}
}

/// Get the property of connected cameras, you can do this without open the camera.
pub fn camera_property(camera_index: u8) -> Result<CameraInfo, ErrorCode> {
    let mut camera_info = ASI_CAMERA_INFO::default();
    let error = unsafe {ASIGetCameraProperty(&mut camera_info, camera_index as i32)};
    ErrorCode::from(error).to_result(CameraInfo::from(camera_info))
}

/// Get the property of the connected cameras by ID.
pub fn camera_property_by_id(camera_id: u8) -> Result<CameraInfo, ErrorCode> {
    let mut camera_info = ASI_CAMERA_INFO::default();
    let error = unsafe {ASIGetCameraPropertyByID(camera_id.into(), &mut camera_info)};
    ErrorCode::from(error).to_result(CameraInfo::from(camera_info))
}

/// Get version string, like "1, 13, 0503", for ASI SDK
pub fn sdk_version() -> String {
    unsafe {CString::from_raw(ASIGetSDKVersion()).to_string_lossy().to_string()}
}