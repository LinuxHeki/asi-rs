use std::ffi::CString;

use asi_sys::*;

pub const ID_MAX: u32 = 128;

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
            _ => panic!("Invalid bayer pattern"),
        }
    }
}

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
            _ => panic!("Invalid image type"),
        }
    }
}

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
    Horiz,
    Vert,
    Both,
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
            _ => panic!("Invalid camera mode"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrigOutput {
    PinA,
    PinB,
    None = -1,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorCode {
    Success,
    InvalidIndex,
    InvalidId,
    InvalidControlType,
    CameraClosed,
    CameraRemoved,
    InvalidPath,
    InvalidFileFormat,
    InvalidSize,
    InvalidImgType,
    OutOfBoundary,
    Timeout,
    InvalidSequence,
    BufferTooSmall,
    VideoModeActive,
    ExposureInProgress,
    GeneralError,
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
    pub name: String,
    pub camera_id: i32,
    pub max_height: i32,
    pub max_width: i32,
    pub is_color_cam: bool,
    pub bayer_pattern: BayerPattern,
    pub supported_bins: Vec<i32>,
    pub supported_video_formats: Vec<ImgType>,
    pub pixel_size: f64,
    pub mechanical_shutter: bool,
    pub st4_port: bool,
    pub is_cooler_cam: bool,
    pub is_usb3_host: bool,
    pub is_usb3_camera: bool,
    pub elec_per_adu: f32,
    pub bit_depth: i32,
    pub is_trigger_cam: bool,
}

impl From<ASI_CAMERA_INFO> for CameraInfo {
    fn from(info: ASI_CAMERA_INFO) -> Self {
        Self {
            name: unsafe {std::str::from_utf8_unchecked(std::mem::transmute(info.Name.as_slice())).to_string()},
            camera_id: info.CameraID,
            max_height: info.MaxHeight as i32,
            max_width: info.MaxWidth as i32,
            is_color_cam: info.IsColorCam == 1,
            bayer_pattern: BayerPattern::from(info.BayerPattern),
            supported_bins: info.SupportedBins.iter().cloned().take_while(|&x| x != 0).collect(),
            supported_video_formats: {
                let mut formats = Vec::new();
                info.SupportedVideoFormat.iter().cloned().take_while(|&x| x != -1).for_each(|x| { formats.push(ImgType::from(x)); });
                formats
            },
            pixel_size: info.PixelSize,
            mechanical_shutter: info.MechanicalShutter == 1,
            st4_port: info.ST4Port == 1,
            is_cooler_cam: info.IsCoolerCam == 1,
            is_usb3_host: info.IsUSB3Host == 1,
            is_usb3_camera: info.IsUSB3Camera == 1,
            elec_per_adu: info.ElecPerADU,
            bit_depth: info.BitDepth,
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
    Temperature,
    Flip,
    AutoMaxGain,
    AutoMaxExp,
    AutoTargetBrightness,
    HardwareBin,
    HighSpeedMode,
    CoolerPowerPerc,
    TargetTemp,
    CoolerOn,
    MonoBin,
    FanOn,
    PatternAdjust,
    AntiDewHeater,
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
            _ => panic!("Invalid control type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ControlCaps {
    pub name: String,
    pub description: String,
    pub max_value: i32,
    pub min_value: i32,
    pub default_value: i32,
    pub is_auto_supported: bool,
    pub is_writable: bool,
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
    Idle,
    Working,
    Success,
    Failed,
}

impl From<u32> for ExposureStatus {
    fn from(exposure_status: u32) -> Self {
        match exposure_status {
            0 => Self::Idle,
            1 => Self::Working,
            2 => Self::Success,
            3 => Self::Failed,
            _ => panic!("Invalid exposure status"),
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

pub type Sn = Id;

#[derive(Debug, Clone, PartialEq)]
pub struct SupportedMode {
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

pub fn get_num_of_connected_cameras() -> i32 {
    unsafe {
        ASIGetNumOfConnectedCameras()
    }
}

pub fn get_product_ids() -> Vec<i32> {
    unsafe {
        let mut pids = Vec::new();
        ASIGetProductIDs(pids.as_mut_ptr());
        pids
    }
}

pub fn camera_check(vid: i32, pid: i32) -> bool {
    unsafe {
        ASICameraCheck(vid, pid) == 1
    }
}

pub fn get_camera_property(camera_index: i32) -> Result<CameraInfo, ErrorCode> {
    unsafe {
        let mut camera_info = ASI_CAMERA_INFO::default();
        let result = ASIGetCameraProperty(&mut camera_info, camera_index);
        ErrorCode::from(result).to_result(CameraInfo::from(camera_info))
    }
}

pub fn get_camera_property_by_id(camera_id: i32) -> Result<CameraInfo, ErrorCode> {
    unsafe {
        let mut camera_info = ASI_CAMERA_INFO::default();
        let result = ASIGetCameraPropertyByID(camera_id, &mut camera_info);
        ErrorCode::from(result).to_result(CameraInfo::from(camera_info))
    }
}

pub fn open_camera(camera_id: i32) -> Result<(), ErrorCode> {
    unsafe {
        let result = ASIOpenCamera(camera_id);
        ErrorCode::from(result).to_result(())
    }
}

pub fn init_camera(camera_id: i32) -> Result<(), ErrorCode> {
    unsafe {
        let result = ASIInitCamera(camera_id);
        ErrorCode::from(result).to_result(())
    }
}

pub fn close_camera(camera_id: i32) -> Result<(), ErrorCode> {
    unsafe {
        let result = ASICloseCamera(camera_id);
        ErrorCode::from(result).to_result(())
    }
}

pub fn get_num_of_controls(camera_id: i32) -> Result<i32, ErrorCode> {
    unsafe {
        let mut num = 0;
        let result = ASIGetNumOfControls(camera_id, &mut num);
        ErrorCode::from(result).to_result(num)
    }
}

pub fn get_control_caps(camera_id: i32, control_index: i32) -> Result<ControlCaps, ErrorCode> {
    unsafe {
        let mut control_caps = ASI_CONTROL_CAPS::default();
        let status = ASIGetControlCaps(camera_id, control_index, &mut control_caps);
        ErrorCode::from(status).to_result(ControlCaps::from(control_caps))
    }
}

pub fn get_control_value(camera_id: i32, control_type: ControlType) -> Result<(i32, bool), ErrorCode> {
    unsafe {
        let (mut value, mut auto) = (0, 0);
        let status = ASIGetControlValue(camera_id, control_type as i32, &mut value, &mut auto);
        ErrorCode::from(status).to_result((value as i32, auto == 1))
    }
}

pub fn set_control_value(camera_id: i32, control_type: ControlType, value: i32, auto: bool) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASISetControlValue(camera_id, control_type as i32, value.into(), auto.into());
        ErrorCode::from(status).to_result(())
    }
}

pub fn set_roi_format(camera_id: i32, width: i32, height: i32, bin: i32, img_type: ImgType) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASISetROIFormat(camera_id, width, height, bin, img_type as i32);
        ErrorCode::from(status).to_result(())
    }
}

pub fn get_roi_format(camera_id: i32) -> Result<(i32, i32, i32, ImgType), ErrorCode> {
    unsafe {
        let (mut width, mut height, mut bin, mut img_type) = (0, 0, 0, ASI_IMG_TYPE::default());
        let status = ASIGetROIFormat(camera_id, &mut width, &mut height, &mut bin, &mut img_type);
        ErrorCode::from(status).to_result((width, height, bin, ImgType::from(img_type)))
    }
}

pub fn set_start_pos(camera_id: i32, start_x: i32, start_y: i32) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASISetStartPos(camera_id, start_x, start_y);
        ErrorCode::from(status).to_result(())
    }
}

pub fn get_start_pos(camera_id: i32) -> Result<(i32, i32), ErrorCode> {
    unsafe {
        let (mut start_x, mut start_y) = (0, 0);
        let status = ASIGetStartPos(camera_id, &mut start_x, &mut start_y);
        ErrorCode::from(status).to_result((start_x, start_y))
    }
}

pub fn get_dropped_frames(camera_id: i32) -> Result<Vec<i32>, ErrorCode> {
    unsafe {
        let mut dropped_frames = Vec::new();
        let status = ASIGetDroppedFrames(camera_id, dropped_frames.as_mut_ptr());
        ErrorCode::from(status).to_result(dropped_frames)
    }
}

pub fn enable_dark_subtract(camera_id: i32, path: &str) -> Result<(), ErrorCode> {
    unsafe {
        let path = CString::new(path).unwrap().into_raw();
        let status = ASIEnableDarkSubtract(camera_id, path);
        ErrorCode::from(status).to_result(())
    }
}

pub fn disable_dark_subtract(camera_id: i32) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASIDisableDarkSubtract(camera_id);
        ErrorCode::from(status).to_result(())
    }
}

pub fn start_video_capture(camera_id: i32) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASIStartVideoCapture(camera_id);
        ErrorCode::from(status).to_result(())
    }
}

pub fn stop_video_capture(camera_id: i32) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASIStopVideoCapture(camera_id);
        ErrorCode::from(status).to_result(())
    }
}

pub fn get_video_data(camera_id: i32, buffer: &mut [u8], wait_ms: i32) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASIGetVideoData(camera_id, buffer.as_mut_ptr(), (buffer.len() as i32).into(), wait_ms);
        ErrorCode::from(status).to_result(())
    }
}

pub fn pulse_guide_on(camera_id: i32, direction: GuideDirection) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASIPulseGuideOn(camera_id, direction as i32);
        ErrorCode::from(status).to_result(())
    }
}

pub fn pulse_guide_off(camera_id: i32, direction: GuideDirection) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASIPulseGuideOff(camera_id, direction as i32);
        ErrorCode::from(status).to_result(())
    }
}

pub fn start_exposure(camera_id: i32, is_dark: bool) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASIStartExposure(camera_id, is_dark as i32);
        ErrorCode::from(status).to_result(())
    }
}

pub fn stop_exposure(camera_id: i32) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASIStopExposure(camera_id);
        ErrorCode::from(status).to_result(())
    }
}

pub fn get_exp_status(camera_id: i32) -> Result<ExposureStatus, ErrorCode> {
    unsafe {
        let mut exposure_status = ASI_EXPOSURE_STATUS::default();
        let status = ASIGetExpStatus(camera_id, &mut exposure_status);
        ErrorCode::from(status).to_result(ExposureStatus::from(exposure_status))
    }
}

pub fn get_data_after_exp(camera_id: i32, buffer: &mut [u8]) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASIGetDataAfterExp(camera_id, buffer.as_mut_ptr(), (buffer.len() as i32).into());
        ErrorCode::from(status).to_result(())
    }
}

pub fn get_id(camera_id: i32) -> Result<Id, ErrorCode> {
    unsafe {
        let mut id = ASI_ID::default();
        let status = ASIGetID(camera_id, &mut id);
        ErrorCode::from(status).to_result(Id::from(id))
    }
}

pub fn set_id(camera_id: i32, id: Id) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASISetID(camera_id, id.to_asi_id());
        ErrorCode::from(status).to_result(())
    }
}

pub fn get_gain_offset(camera_id: i32) -> Result<(i32, i32, i32, i32), ErrorCode> {
    unsafe {
        let (mut off_hig_dr, mut off_unity_gain, mut gain_low_rn, mut off_low_rn) = (0, 0, 0, 0);
        let status = ASIGetGainOffset(camera_id, &mut off_hig_dr, &mut off_unity_gain, &mut gain_low_rn, &mut off_low_rn);
        ErrorCode::from(status).to_result((off_hig_dr, off_unity_gain, gain_low_rn, off_low_rn))
    }
}

pub fn get_lmh_gain_offset(camera_id: i32) -> Result<(i32, i32, i32, i32), ErrorCode> {
    unsafe {
        let (mut l_gain, mut m_gain, mut h_gain, mut h_offset) = (0, 0, 0, 0);
        let status = ASIGetLMHGainOffset(camera_id, &mut l_gain, &mut m_gain, &mut h_gain, &mut h_offset);
        ErrorCode::from(status).to_result((l_gain, m_gain, h_gain, h_offset))
    }
}

pub fn get_sdk_version() -> String {
    unsafe {
        CString::from_raw(ASIGetSDKVersion()).to_string_lossy().to_string()
    }
}

pub fn get_camera_support_mode(camera_id: i32) -> Result<SupportedMode, ErrorCode> {
    unsafe {
        let mut supported_mode = ASI_SUPPORTED_MODE::default();
        let status = ASIGetCameraSupportMode(camera_id, &mut supported_mode);
        ErrorCode::from(status).to_result(SupportedMode::from(supported_mode))
    }
}

pub fn get_camera_mode(camera_id: i32) -> Result<CameraMode, ErrorCode> {
    unsafe {
        let mut camera_mode = ASI_CAMERA_MODE::default();
        let status = ASIGetCameraMode(camera_id, &mut camera_mode);
        ErrorCode::from(status).to_result(CameraMode::from(camera_mode))
    }
}

pub fn set_camera_mode(camera_id: i32, camera_mode: CameraMode) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASISetCameraMode(camera_id, camera_mode as i32);
        ErrorCode::from(status).to_result(())
    }
}

pub fn send_soft_trigger(camera_id: i32, start: bool) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASISendSoftTrigger(camera_id, start as i32);
        ErrorCode::from(status).to_result(())
    }
}

pub fn get_serial_number(camera_id: i32) -> Result<Sn, ErrorCode> {
    unsafe {
        let mut sn = ASI_SN::default();
        let status = ASIGetSerialNumber(camera_id, &mut sn);
        ErrorCode::from(status).to_result(Sn::from(sn))
    }
}

pub fn set_trigger_output_io_conf(camera_id: i32, pin: TrigOutput, pin_high: bool, delay: i32, duration: i32) -> Result<(), ErrorCode> {
    unsafe {
        let status = ASISetTriggerOutputIOConf(camera_id, pin as i32, pin_high as i32, delay.into(), duration.into());
        ErrorCode::from(status).to_result(())
    }
}

pub fn get_trigger_output_io_conf(camera_id: i32, pin: TrigOutput) -> Result<(bool, i32, i32), ErrorCode> {
    unsafe {
        let (mut pin_high, mut delay, mut duration) = (0, 0, 0);
        let status = ASIGetTriggerOutputIOConf(camera_id, pin as i32, &mut pin_high, &mut delay, &mut duration);
        ErrorCode::from(status).to_result((pin_high == 1, delay as i32, duration as i32))
    }
}