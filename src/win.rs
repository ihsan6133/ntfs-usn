use windows_sys::{
    core::*,
    Win32::Foundation::*,
    Win32::System::IO::*,
    Win32::System::Ioctl::*,
    Win32::System::Threading::*, 
    Win32::UI::WindowsAndMessaging::*,
    Win32::Storage::FileSystem::*,
};
use std::io;
pub fn get_last_error() -> io::Error
{
    io::Error::from_raw_os_error(unsafe {GetLastError().try_into().unwrap()} )
}

pub fn create_file(file_name: &str, desired_access: u32, share_mode: u32, creation_mode: u32, flags: u32) -> Result<isize, std::io::Error>
{
    let mut vec: Vec<u16> = file_name.encode_utf16().collect();
    vec.push(0);

    let handle = unsafe { CreateFileW(vec.as_ptr(),
        desired_access,
        share_mode,
        std::ptr::null(),
        creation_mode,
        flags,
    0 )};

    if handle == INVALID_HANDLE_VALUE
    {
        Err(get_last_error())
    }
    else
    {
        Ok(handle)
    }
} 

