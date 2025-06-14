/// # Module: DET (Default Error Tracer)
///
/// This module provides functionality for error tracing and reporting in the AUTOSAR Classic Platform.
///
/// ## Description:
/// The DET module is responsible for capturing and reporting runtime errors that occur in the base software.
/// It adheres to the AUTOSAR specifications and ensures standardized error handling across modules.
///
/// ## AUTOSAR Details:
/// - **AUTOSAR Version:** R21-11
/// - **Classic Platform:** Yes
///
/// ## Version History:
/// --------------------------------------------------------------------------------------------
/// | Version | Date       | Author         | Description                                      |
/// --------------------------------------------------------------------------------------------
/// | 1.0.0   | 2023-10-01 | Divyan-coder   | Initial version of det.rs file.                 |
/// --------------------------------------------------------------------------------------------
///
/// ## Notes:
/// - The DET module is designed to be lightweight and efficient for embedded systems.
/// - It is intended for use in environments where runtime error detection is critical.
/// - Ensure compatibility with the specified AUTOSAR version before integrating.
///
/// ## Implemented Requirements:
/// 
///
/// ## Not Implemented Requirements:
/// 

use crate::std_types::{StdReturnType, StdVersionInfoType};
 
/// Structure to represent an error reported to DET
#[derive(Debug, Clone)]
pub struct DetError {
    pub module_id: u16,
    pub instance_id: u8,
    pub api_id: u8,
    pub error_id: u8,
}

#[derive(Debug)]
pub struct DetConfigType {

}
/// DET Module implementation
pub struct Det {
    errors: Vec<DetError>, // In-memory storage for errors
    runtime_errors: Vec<DetError>, // Runtime errors
    transient_faults: Vec<DetError>, // Transient faults
}

impl Det {
    /// Create a new DET instance
    pub fn new() -> Self {
        Det {
            errors: Vec::new(),
            runtime_errors : Vec::new(),
            transient_faults: Vec::new(),
        }
    }

    /// Initialize the DET module
    pub fn init(config_ptr: &DetConfigType){
        // Initialization logic for DET can be added here
        println!("Init configPtr = {:?}", config_ptr);
    }

    pub fn start() {
        // Initialization logic for DET can be added here
        println!("DET started");
    }

    /// Report an error to DET
    pub fn report_error(&mut self, module_id: u16, instance_id: u8, api_id: u8, error_id: u8) -> StdReturnType{
        let ret: StdReturnType = StdReturnType::Ok;

        let error = DetError {
            module_id,
            instance_id,
            api_id,
            error_id,
        };

        // Log the error (for debugging purposes)
        println!("DET Error Reported: {:?}", error);

        // Store the error in the internal list
        self.errors.push(error);

        ret
    }

    /// Report an runtime error to DET
    pub fn report_runtime_error(&mut self, module_id: u16, instance_id: u8, api_id: u8, error_id: u8)->StdReturnType {
        let ret: StdReturnType = StdReturnType::Ok;
        let runtime_error = DetError {
            module_id,
            instance_id,
            api_id,
            error_id,
        };

        // Log the error (for debugging purposes)
        println!("DET Error Reported: {:?}", runtime_error);

        // Store the error in the internal list
        self.runtime_errors.push(runtime_error);

        ret
    }
    
    /// Report a transient fault to DET
    pub fn report_transient_fault(&mut self, module_id: u16, instance_id: u8, api_id: u8, error_id: u8) -> StdReturnType{
        let ret: StdReturnType = StdReturnType::Ok;
        let transient_fault = DetError {
            module_id,
            instance_id,
            api_id,
            error_id,
        };

        // Log the error (for debugging purposes)
        println!("DET Transient Fault Reported: {:?}", transient_fault);

        // Store the fault in the internal list
        self.transient_faults.push(transient_fault);

        ret
    }

    pub fn get_version_info() -> StdVersionInfoType {
        // Return the version information for DET
        StdVersionInfoType {
            vendor_id: 0x0001,  // Rustsar vender ID
            sw_major_version: 0x01, // Major version
            sw_minor_version: 0x00, // Minor version
            sw_patch_version: 0x00, // Patch version
        }
    }

    /// Retrieve all reported errors
    pub fn get_errors(&self) -> &Vec<DetError> {
        &self.errors
    }

    /// Clear all reported errors
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_error() {
        let mut det = Det::new();

        // Report an error
        det.report_error(1, 1, 1, 1);

        // Verify the error is stored
        let errors = det.get_errors();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].module_id, 1);
        assert_eq!(errors[0].instance_id, 1);
        assert_eq!(errors[0].api_id, 1);
        assert_eq!(errors[0].error_id, 1);
    }

    #[test]
    fn test_clear_errors() {
        let mut det = Det::new();

        // Report an error
        det.report_error(1, 1, 1, 1);

        // Clear errors
        det.clear_errors();

        // Verify the errors are cleared
        assert!(det.get_errors().is_empty());
    }
}