#![allow(unused_variables)]

/// # module cryif (Crypto interface)
/// 
/// 
/// 
/// 
/// /// ## AUTOSAR Details:
/// - **AUTOSAR Version:** R21-11
/// - **Classic Platform:** Yes
///
/// ## Version History:
/// ------------------------------------------------------------------------------------------------
/// | Version | Date       | Author         | Description                                           |
/// -----------------------------------------------------------------------------------------------
/// | 1.0.0   | 2023-10-01 | Divyan-coder   | Initial version of cryif.rs file.                     |
/// -----------------------------------------------------------------------------------------------
/// 
/// ## Implemented Requirements:
/// 
///
/// ## Not Implemented Requirements:
/// 
/// 



use crate::std_types::{StdReturnType, StdVersionInfoType};
use crate::crypto_general_types;



//SWS_cry_if_91118
//The content of the configuration data structure is implementation specific.
pub struct CryIfConfigType {
    dummy_field: u32 // Placeholder field
}

//[SWS_cry_if_91000]
/// Initialize the Crypto Interface
/// C prototype : void CryptoIf_Init(const CryptoIf_ConfigType* configPtr);
pub fn cry_if_init(config: Option<&CryIfConfigType>) {
	// stub: no logic yet
}


//SWS_cry_if_91001
/// Get version information
/// C prototype (approx): void CryptoIf_GetVersionInfo(Std_VersionInfoType* versioninfo);
pub fn cry_if_get_version_info() -> StdVersionInfoType {
	StdVersionInfoType {
		vendor_id: 0,
		sw_major_version: 0,
		sw_minor_version: 0,
		sw_patch_version: 0,
	}
}

/// Req : SWS_cry_if_91003
/// Service ID : 0x03
/// Sync/Async : Depends on configuration
/// Reentrancy : Reentrant
pub fn cry_if_process_job ( channel_id : u32 , job : Option< &mut crypto_general_types::CryptoJobType>) -> StdReturnType {
	// stub: no logic yet
	StdReturnType::NotOk
}

/// Req : SWS_cry_if_91014
/// Service ID : 0x0e
/// Sync/Async : synchronous
/// Reentrancy : Reentrant
pub fn cry_if_cancel_job ( channel_id : u32 ,
							job : Option< &mut crypto_general_types::CryptoJobType> ) -> StdReturnType {
	// stub: no logic yet
	return StdReturnType::NotOk;
}

/// Req : SWS_cry_if_91004
/// Service ID : 0x04
/// Sync/Async : synchronous
/// Reentrancy : Non Reentrant
pub fn cry_if_key_element_set ( cry_if_key_id:u32 ,
								key_element_id:u32 ,
								key_ptr : Option<& u8>,
								key_length:u32  ) -> StdReturnType {
	// stub: no logic yet
	StdReturnType::NotOk
}

/// Req : SWS_cry_if_91005
/// Service ID : 0x05
/// Sync/Async : synchronous
/// Reentrancy : Non Reentrant
pub fn cry_if_key_set_valid ( cry_if_key_id :u32  ) -> StdReturnType {
	// stub: no logic yet
	return StdReturnType::NotOk;
}

/// Req : SWS_cry_if_91005
/// Service ID : 0x014
/// Sync/Async : synchronous
/// Reentrancy : Non Reentrant
pub fn cry_if_key_set_invalid (cry_if_key_id:u32  ) -> StdReturnType {
	// stub: no logic yet
	return StdReturnType::NotOk;
}

/// Req : SWS_cry_if_91005
/// Service ID : 0x013
/// Sync/Async : synchronous
/// Reentrancy : Non Reentrant
pub fn cry_if_key_get_status ( cry_if_key_id:u32 ,
								key_status_ptr: Option<&mut crypto_general_types::CryptoKeyStatusType>  ) -> StdReturnType {
	// stub: no logic yet
	return StdReturnType::NotOk;
}

/// Req : SWS_cry_if_91006
/// Service ID : 0x06
/// Sync/Async : synchronous
/// Reentrancy : Reentrant
pub fn cry_if_key_element_get ( cry_if_key_id:u32 ,
								key_element_id:u32,
								result_ptr: Option<&mut u8>,
								result_length_ptr: Option<&mut u32>) -> StdReturnType {
	// stub: no logic yet
	StdReturnType::NotOk
}

/// Req : SWS_cry_if_91006
/// Service ID : 0x0F
/// Sync/Async : synchronous
/// Reentrancy : Reentrant
pub fn cry_if_key_element_copy ( 	cry_if_key_id:u32 ,
									key_element_id:u32 ,
									target_cry_if_key_id:u32 ,
									target_key_element_id:u32  ) -> StdReturnType {
	// stub: no logic yet
	StdReturnType::NotOk
}

/// Req : SWS_cry_if_91018
/// Service ID : 0x12
/// Sync/Async : synchronous
/// Reentrancy : Reentrant
pub fn cry_if_key_element_copy_partial (  cry_if_key_id:u32, 
											key_element_id:u32,
											key_element_source_offset:u32,
											key_element_target_offset:u32,  
											key_element_copy_length:u32,  
											target_cry_if_key_id:u32,
											target_key_element_id:u32 )-> StdReturnType {
	// stub: no logic yet
									StdReturnType::NotOk
}

/// Req : SWS_cry_if_91016
/// Service ID : 0x10
/// Sync/Async : synchronous
/// Reentrancy : Reentrant
pub fn cry_if_key_copy (  cry_if_key_id:u32,
							target_cry_if_key_id:u32 ) -> StdReturnType {
	// stub: no logic yet
	return StdReturnType::NotOk;
}

pub fn cry_if_random_seed (  cry_if_key_id:u32, 
							seed_ptr : Option<&u8>, 
							seed_length:u32) -> StdReturnType {
	// stub: no logic yet
	return StdReturnType::NotOk;
}

pub fn cry_if_key_generate (  cry_if_key_id:u32 ) -> StdReturnType {
	// stub: no logic yet
	return StdReturnType::NotOk;
}

pub fn cry_if_key_derive (  cry_if_key_id:u32,
							target_cry_if_key_id:u32 ) -> StdReturnType {
	// stub: no logic yet
	return StdReturnType::NotOk;
}

pub fn cry_if_key_exchange_calc_pub_val (  cry_if_key_id:u32,
											public_value_ptr : Option<&mut u8>,
											public_value_length_ptr : Option<&mut u32> ) -> StdReturnType {
	// stub: no logic yet
	return StdReturnType::NotOk;
}


pub fn cry_if_key_exchange_calc_secret (  cry_if_key_id:u32,
										partner_public_value_ptr:Option<&u8>,
										partner_public_value_length:u32 ) -> StdReturnType {
	// stub: no logic yet
	return StdReturnType::NotOk;
}

pub fn cry_if_custom_sync ( dispatch_id:u32,
							key_id:u32,
							key_element_id:u32,
							target_key_id:u32,
							target_key_element_id:u32,
	                    	input_ptr:Option<& u8>,
							input_length:u32,
							output_ptr:Option<&mut u8>, 
							output_length_ptr:Option<&mut u32>,
							secondary_output_ptr:Option<&mut u8>,
						 	secondary_output_length_ptr:Option<&mut u32> )-> StdReturnType {
	// stub: no logic yet
						return StdReturnType::NotOk;
}


///Call-back notifications

pub fn cry_if_callback_notification ( job : Option<& crypto_general_types::CryptoJobType>,
										result:crypto_general_types::CryptoResultType ) {
	// stub: no logic yet
}

// End of stubs

