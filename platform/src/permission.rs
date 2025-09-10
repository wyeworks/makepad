
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Permission {
    AudioInput,
    Camera,
    Location,
    // Future permissions can be added here
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PermissionStatus {
    Granted,
    Denied,
    NotDetermined, // iOS specific - hasn't asked yet
}

#[derive(Debug, Clone)]
pub struct PermissionRequest {
    pub permission: Permission,
}

#[derive(Debug, Clone)]
pub struct PermissionResult {
    pub permission: Permission,
}

impl Permission {
    #[cfg(target_os = "android")]
    pub fn to_android_permission(&self) -> &'static str {
        match self {
            Permission::AudioInput => "android.permission.RECORD_AUDIO",
            Permission::Camera => "android.permission.CAMERA",
            Permission::Location => "android.permission.ACCESS_FINE_LOCATION",
        }
    }
    
    #[cfg(target_os = "ios")]
    pub fn to_ios_usage_key(&self) -> &'static str {
        match self {
            Permission::AudioInput => "NSMicrophoneUsageDescription",
            Permission::Camera => "NSCameraUsageDescription", 
            Permission::Location => "NSLocationWhenInUseUsageDescription",
        }
    }
}