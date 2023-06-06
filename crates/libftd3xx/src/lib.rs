//#![deny(missing_docs, unsafe_code)]
//! The D3XX interface is a proprietary interface specifically for FTDI SuperSpeed USB devices (FT60x series). 
//! D3XX implements a proprietary protocol different from D2XX in order to maximize USB 3.0 bandwidth. 

/// struct/enum/constant objects/values
pub mod types;

/// Safe ftd3xx functions
pub mod functions;