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
fn get_last_error() -> io::Error
{
    io::Error::from_raw_os_error(unsafe {GetLastError().try_into().unwrap()} )
}

fn create_file(file_name: &str, desired_access: u32, share_mode: u32, creation_mode: u32, flags: u32) -> Result<isize, std::io::Error>
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

pub fn get_volume_handle(path: &str) -> io::Result<isize>
{
    let mut device_path = String::from("\\\\.\\");
    device_path.push_str(path);

    create_file(device_path.as_str(), GENERIC_READ, FILE_SHARE_READ | FILE_SHARE_WRITE, OPEN_EXISTING, 0)
}

pub fn query_usn_journal(vol_handle: isize) -> io::Result<USN_JOURNAL_DATA_V0>
{
    let mut journal_data = USN_JOURNAL_DATA_V0
    {
        UsnJournalID: 0,
        FirstUsn: 0,
        NextUsn: 0,
        LowestValidUsn: 0,
        MaxUsn: 0,
        MaximumSize: 0,
        AllocationDelta: 0,
    };
    
    let mut dw_bytes = 0;

    let res = unsafe { DeviceIoControl(
        vol_handle,
        FSCTL_QUERY_USN_JOURNAL,
        std::ptr::null(),
        0,
        std::ptr::addr_of_mut!(journal_data) as *mut std::ffi::c_void,
        std::mem::size_of::<USN_JOURNAL_DATA_V0>().try_into().unwrap(),
        std::ptr::addr_of_mut!(dw_bytes),
        std::ptr::null_mut()
    ) };

    if res == 0
    {
        return Err(get_last_error());
    }
    Ok(journal_data)

}


#[cfg(test)]
mod tests
{

    #[test]
    fn get_volume_handle()
    {
        super::get_volume_handle("C:").unwrap();
    }

    #[test]
    fn query_usn_journal()
    {
        let handle = super::get_volume_handle("C:").unwrap();
        let journal_data = super::query_usn_journal(handle).unwrap();

        println!("UsnJournalID: {}", journal_data.UsnJournalID);
    }
}