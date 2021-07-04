use libva_sys::*;
use std::ffi::CStr;
use std::ffi::c_void;

fn main() {
    unsafe {
        let va_dpy = va_open_display();

        if va_dpy.is_null() {
            panic!("va_open_display failed.");
        }

        let mut major = 0i32;
        let mut minor = 0i32;
        let _va_status = vaInitialize(va_dpy, &mut major, &mut minor);

        println!("VA-API version: {}.{}", major, minor);

        let vendor_string = vaQueryVendorString(va_dpy);
        let vendor_string = CStr::from_ptr(vendor_string).to_string_lossy().to_string();
        println!("Driver version: {}", vendor_string);

        let mut max_num_entrypoints = vaMaxNumEntrypoints(va_dpy);
        let mut entrypoints = Vec::with_capacity(max_num_entrypoints as usize);

        let mut max_num_profiles = vaMaxNumProfiles(va_dpy);
        let mut profiles = Vec::with_capacity(max_num_profiles as usize);

        vaQueryConfigProfiles(va_dpy, profiles.as_mut_ptr(), &mut max_num_profiles);
        profiles.set_len(max_num_profiles as usize);

        println!("Found {} supported profiles and entrypoints", max_num_profiles);

        for profile in profiles {
            let va_status = vaQueryConfigEntrypoints(va_dpy, profile, entrypoints.as_mut_ptr(), &mut max_num_entrypoints);
            entrypoints.set_len(max_num_entrypoints as usize);

            if va_status == VA_STATUS_ERROR_UNSUPPORTED_PROFILE as i32 {
                continue;
            }

            for entrypoint in entrypoints.iter() {
                let profile_str = vaProfileStr(profile);
                let profile_str = CStr::from_ptr(profile_str).to_string_lossy().to_string();

                let entrypoint_str = vaEntrypointStr(*entrypoint);
                let entrypoint_str = CStr::from_ptr(entrypoint_str).to_string_lossy().to_string();

                println!("    {:<36}:{}", profile_str, entrypoint_str);
            }
        }
    }
}
