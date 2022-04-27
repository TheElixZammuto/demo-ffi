#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::mem::transmute_copy;
use windows::Win32::Foundation::HINSTANCE as OtherHinstance;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Direct3D::*;

fn main(){
    unsafe {
        println!("Trying to create the COM Object from C and consume it from C");
        main_c();
        println!("Trying to create the COM Object from C and consume it from Rust");
        main_rust();
    }
}


fn main_c() {
    unsafe {
        c_test();
    }
}

unsafe fn main_rust(){
    let mut device = None;
    let mut device_context = None;
    let _ = match windows::Win32::Graphics::Direct3D11::D3D11CreateDevice(None, D3D_DRIVER_TYPE_HARDWARE, OtherHinstance::default(), D3D11_CREATE_DEVICE_DEBUG, &[], D3D11_SDK_VERSION, &mut device, std::ptr::null_mut(), &mut device_context) {
        Ok(e) => e,
        Err(e) => panic!("Creation Failed: {}", e)
    };
    let mut device = match device {
        Some(e) => e,
        None => panic!("Creation Failed2")
    };
    let mut f2 : ID3D11Device = transmute_copy(&device); //Transmuting the WinAPI into a bindgen ID3D11Device
    test_ffi(&mut f2);
}