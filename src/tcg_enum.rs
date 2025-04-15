use crate::parser::parsers::*;
use crate::parser::DescriptionParser;
use std::fmt;

use num_enum::TryFromPrimitive;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
pub enum TcgEventType {
    EvPrebootCert = 0x0,
    EvPostCode = 0x1,
    EvUnused = 0x2,
    EvNoAction = 0x3,
    EvSeparator = 0x4,
    EvAction = 0x5,
    EvEventTag = 0x6,
    EvSCrtmContents = 0x7,
    EvSCrtmVersion = 0x8,
    EvCpuMicrocode = 0x9,
    EvPlatformConfigFlags = 0xa,
    EvTableOfDevices = 0xb,
    EvCompactHash = 0xc,
    EvIpl = 0xd,
    EvIplPartitionData = 0xe,
    EvNonhostCode = 0xf,
    EvNonhostConfig = 0x10,
    EvNonhostInfo = 0x11,
    EvOmitBootDeviceEvents = 0x12,

    // TCG EFI Platform Specification For TPM Family 1.1 or 1.2
    EvEfiEventBase = 0x80000000,
    EvEfiVariableDriverConfig = 0x80000001,
    EvEfiVariableBoot = 0x80000002,
    EvEfiBootServicesApplication = 0x80000003,
    EvEfiBootServicesDriver = 0x80000004,
    EvEfiRuntimeServicesDriver = 0x80000005,
    EvEfiGptEvent = 0x80000006,
    EvEfiAction = 0x80000007,
    EvEfiPlatformFirmwareBlob = 0x80000008,
    EvEfiHandoffTables = 0x80000009,
    EvEfiPlatformFirmwareBlob2 = 0x8000000a,
    EvEfiHandoffTables2 = 0x8000000b,
    EvEfiVariableBoot2 = 0x8000000c,
    EvEfiHcrtmEvent = 0x80000010,
    EvEfiVariableAuthority = 0x800000e0,
    EvEfiSpdmFirmwareBlob = 0x800000e1,
    EvEfiSpdmFirmwareConfig = 0x800000e2,
}

impl TcgEventType {
    pub (crate) fn get_parser(&self) -> Box<dyn DescriptionParser> {
        match self {
            // Self::EvPrebootCert => "EV_PREBOOT_CERT",
            Self::EvPostCode => Box::new(EvSimpleParser),
            // Self::EvUnused => "EV_UNUSED",
            // Self::EvNoAction => "EV_NO_ACTION",
            Self::EvSeparator => Box::new(EvBlankParser),
            Self::EvAction => Box::new(EvSimpleParser),
            Self::EvEventTag => Box::new(EvEventTagParser),
            // Self::EvSCrtmContents => "EV_S_CRTM_CONTENTS",
            // Self::EvSCrtmVersion => "EV_S_CRTM_VERSION",
            // Self::EvCpuMicrocode => "EV_CPU_MICROCODE",
            Self::EvPlatformConfigFlags => Box::new(EvSimpleParser),
            // Self::EvTableOfDevices => "EV_TABLE_OF_DEVICES",
            Self::EvCompactHash => Box::new(EvSimpleParser),
            Self::EvIpl => Box::new(EvSimpleParser),
            // Self::EvIplPartitionData => "EV_IPL_PARTITION_DATA",
            // Self::EvNonhostCode => "EV_NONHOST_CODE",
            // Self::EvNonhostConfig => "EV_NONHOST_CONFIG",
            // Self::EvNonhostInfo => "EV_NONHOST_INFO",
            Self::EvOmitBootDeviceEvents => Box::new(EvSimpleParser),

            // Self::EvEfiEventBase => "EV_EFI_EVENT_BASE",
            Self::EvEfiVariableDriverConfig => Box::new(EvEfiVariableParser),
            Self::EvEfiVariableBoot => Box::new(EvEfiVariableParser),
            Self::EvEfiBootServicesApplication => Box::new(EvBootServicesAppParser),
            // Self::EvEfiBootServicesDriver => "EV_EFI_BOOT_SERVICES_DRIVER",
            // Self::EvEfiRuntimeServicesDriver => "EV_EFI_RUNTIME_SERVICES_DRIVER",
            // Self::EvEfiGptEvent => "EV_EFI_GPT_EVENT",
            Self::EvEfiAction => Box::new(EvSimpleParser),
            // Self::EvEfiPlatformFirmwareBlob => "EV_EFI_PLATFORM_FIRMWARE_BLOB",
            // Self::EvEfiHandoffTables => "EV_EFI_HANDOFF_TABLES",
            Self::EvEfiPlatformFirmwareBlob2 => Box::new(EvHandoffTableParser),
            Self::EvEfiHandoffTables2 => Box::new(EvHandoffTableParser),
            Self::EvEfiVariableBoot2 => Box::new(EvEfiVariableParser),
            // Self::EvEfiHcrtmEvent => "EV_EFI_HCRTM_EVENT",
            Self::EvEfiVariableAuthority => Box::new(EvEfiVariableParser),
            // Self::EvEfiSpdmFirmwareBlob => "EV_EFI_SPDM_FIRMWARE_BLOB",
            // Self::EvEfiSpdmFirmwareConfig => "EV_EFI_SPDM_FIRMWARE_CONFIG",
            _ => Box::new(EvBlankParser),
        }
    }

    fn variant_name(&self) -> String {
        let name = format!("{:?}", self);

        let mut result = String::new();
        for (i, ch) in name.chars().enumerate() {
            if ch.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(ch.to_ascii_uppercase());
        }

        result
    }
}

impl fmt::Display for TcgEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.variant_name())
    }
}
