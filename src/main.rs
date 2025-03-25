mod shapes;
use shapes::{area::Area, collisions::Colliadable, rectangle::Rectangle};
use windows::{
    Win32::{
        Foundation::{ERROR_SUCCESS, HANDLE, WIN32_ERROR},
        NetworkManagement::WiFi::{
            self, WLAN_INTERFACE_INFO_LIST, WlanEnumInterfaces, WlanOpenHandle,
        },
    },
    core::Result,
};

struct WifiAP {
    macAdress: char,
    signalStrenght: u64,
}

unsafe fn initializeWlanHandle(handler: &mut HANDLE, pwdNegotiator: &mut u32) -> bool {
    let mut dwResult = WlanOpenHandle(2, None, pwdNegotiator, handler);
    if WIN32_ERROR(dwResult) != ERROR_SUCCESS {
        return false;
    }
    return true;
}

unsafe fn ProcessBssInfo(handler: &mut HANDLE, ){}

unsafe fn ProcessInterfaces(handler: &mut HANDLE)-> bool{
    let mut ppinterfacelist :*mut WLAN_INTERFACE_INFO_LIST= std::ptr::null_mut();
    let dwResults = WlanEnumInterfaces(*handler,None, &mut ppinterfacelist);
    if WIN32_ERROR(dwResults) != ERROR_SUCCESS {
        panic!("No interfaces were found!");
        return false;
    }
    return true;
}



fn main() {
    let mut handler = HANDLE::default();
    let mut pwdNegotiator: u32 = 0;

    unsafe {
        initializeWlanHandle(&mut handler, &mut pwdNegotiator);
        // WlanEnumInterfaces(handler, None, &mut ppinterfacelist);
    }
}
