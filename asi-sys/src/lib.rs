#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for ASI_CAMERA_INFO {
    fn default() -> Self {
        Self {
            Name: [Default::default(); 64],
            CameraID: Default::default(),
            MaxHeight: Default::default(),
            MaxWidth: Default::default(),
            IsColorCam: Default::default(),
            BayerPattern: Default::default(),
            SupportedBins: Default::default(),
            SupportedVideoFormat: Default::default(),
            PixelSize: Default::default(),
            MechanicalShutter: Default::default(),
            ST4Port: Default::default(),
            IsCoolerCam: Default::default(),
            IsUSB3Host: Default::default(),
            IsUSB3Camera: Default::default(),
            ElecPerADU: Default::default(),
            BitDepth: Default::default(),
            IsTriggerCam: Default::default(),
            Unused: Default::default(),
        }
    }
}

impl Default for ASI_CONTROL_CAPS {
    fn default() -> Self {
        Self {
            Name: [Default::default(); 64],
            Description: [Default::default(); 128],
            MaxValue: Default::default(),
            MinValue: Default::default(),
            DefaultValue: Default::default(),
            IsAutoSupported: Default::default(),
            IsWritable: Default::default(),
            ControlType: Default::default(),
            Unused: Default::default(),
        }
    }
}

impl Default for ASI_ID {
    fn default() -> Self {
        Self { id: [Default::default(); 8] }
    }
}

impl Default for ASI_SUPPORTED_MODE {
    fn default() -> Self {
        Self { SupportedCameraMode: [Default::default(); 16] }
    }
}