/// # Module: Std_Types
///
/// This module defines standard types used in the AUTOSAR Classic Platform base software.
///
/// ## Description:
/// This file contains standard type definitions used in the AUTOSAR Classic Platform.
/// It adheres to the AUTOSAR specifications and provides compatibility with the specified version.
///
/// ## AUTOSAR Details:
/// - **AUTOSAR Version:** R21-11
/// - **Classic Platform:** Yes
///
/// ## Version History:
/// --------------------------------------------------------------------------------------------
/// | Version | Date       | Author         | Description                                      |
/// --------------------------------------------------------------------------------------------
/// | 1.0.0   | 2023-10-01 | Divyan-coder   | Initial version of std_types.rs file.            |
/// --------------------------------------------------------------------------------------------
///
/// ## Notes:
/// - This module is intended for use in embedded systems following AUTOSAR standards.
/// - Ensure compatibility with the specified AUTOSAR version before integrating.
/// 
/// - ** Not implemented requirements:**
///   - SWS_Std_00031 : null pointer is already define in rust standard library

// Standard type definitions for AUTOSAR Classic Platform

    /// Standard Return Type
    /// This enum represents the standard return type used in AUTOSAR.
    /// It defines two possible return values: `Ok` and `NotOk`.
    /// SWS REQUIREMENT " SWS_Std_00005"
    #[derive(Debug, PartialEq)]
    pub enum StdReturnType {
        Ok,
        NotOk,
    }
    /// defining constant of StdReturnType
    /// This struct provides constants for the standard return type.
    /// It includes `E_OK` for successful operations and `E_NOT_OK` for failed operations.
    /// SWS REQUIREMENT " SWS_Std_00005"
    impl StdReturnType {
        // Associated constants
        pub const E_OK: u8 = 0;
        pub const E_NOT_OK: u8 = 1;
    }

    /// Standard Version Information Type
    /// This stcut provides version information for the AUTOSAR standard.
    /// It includes fields for vendor ID, software major version, minor version, and patch version.
    /// SWS REQUIREMENT " SWS_Std_00015"
    pub struct StdVersionInfoType {
        pub vendor_id: u16,
        pub sw_major_version: u8,
        pub sw_minor_version: u8,
        pub sw_patch_version: u8,
    }

    /// SWS REQUIREMENT " SWS_Std_00022"
    type StdTransformerErrorCode = u8;

    /// SWS REQUIREMENT " SWS_Std_00024"
    pub enum StdTransformerClass{
        StdTransformerUnspecified,
        StdTransformerSerializer,
        StdTransformerSafety,
        StdTransformerSecurity,
        StdTransformerCustom
    }

    /// SWS REQUIREMENT " SWS_Std_00021"
    pub struct StdTransformerError {
        pub error_code : StdTransformerErrorCode,
        pub transformar_class : StdTransformerClass
    }

    /// SWS REQUIREMENT " SWS_Std_91001"
    pub enum StdMessageTypeType {
        StdMessageTypeRequest,
        StdMessageTypeResponse,
        StdMessageTypeReserved,
    }

    /// SWS REQUIREMENT " SWS_Std_91002"
    pub enum StdMessageResultType {
        StdMessageResultOk,
        StdMessageResultError,
        StdMessageResultReserver,
    }

    /// SWS REQUIREMENT " SWS_Std_91003"
    type _StdExtractProtocolHeaderFieldsType = fn(
        buffer: *const u8, 
        buffer_length: u32, 
        message_type: *mut StdMessageTypeType, 
        message_result: *mut StdMessageResultType
    ) -> StdReturnType;

    /// SWS REQUIREMENT " SWS_Std_00006"
    const _E_OK: u8 = 0; // Operation successful
    const _E_NOT_OK: u8 = 1; // Operation failed
    
    /// SWS REQUIREMENT " SWS_Std_00007"
    const _STD_LOW: u8 = 0;  // Logical low state
    const _STD_HIGH: u8 = 1; // Logical high state

    /// SWS REQUIREMENT " SWS_Std_00013"
    const _STD_IDLE: u8 = 0; // Idle state
    const _STD_ACTIVE: u8 = 1; // Active state

    /// SWS REQUIREMENT " SWS_Std_00010"
    const _STD_ON: u8 = 1; // Feature enabled
    const _STD_OFF: u8 = 0; // Feature disabled

