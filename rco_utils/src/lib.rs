use std::error::Error;

fn equalize_slice_len<T: std::clone::Clone>(slice_one: &[T], slice_two: &[T]) -> (Vec<T>, Vec<T>) {
    if slice_one.len() > slice_two.len() {
        (slice_one.to_vec(), slice_two.iter().cloned().cycle().take(slice_one.len()).collect())
    } else {
        (slice_one.iter().cloned().cycle().take(slice_two.len()).collect(), slice_two.to_vec())
    }
}

fn xor_u8_slices(slice_one: &[u8], slice_two: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if slice_one.len() != slice_two.len() {
        return Err("The given slices are not the same length".into());
    }
    Ok(slice_one.iter()
             .zip(slice_two.iter())
             .map(|(&x1, &x2)| x1 ^ x2)
             .collect())
}

pub fn xor_encrypt_decrypt(key: &[u8], text: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let equalilzed = equalize_slice_len(key, text);
    let key: &[u8] = &equalilzed.0[..];
    let text: &[u8] = &equalilzed.1[..];
    xor_u8_slices(key, text)
}

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
    let mut lpsz_agent: PSTR = unsafe { mem::zeroed() };
    lpsz_agent.0 = CString::new("Name in user-agent").unwrap().into_raw() as *mut u8;
    let lpsz_proxy: PSTR = unsafe { mem::zeroed() };
    let lpsz_proxy_bypass: PSTR = unsafe { mem::zeroed() };
    let internet_handle = unsafe { InternetOpenA(lpsz_agent, 0, lpsz_proxy, lpsz_proxy_bypass, 0) };
    let mut lpsz_url: PSTR = unsafe { mem::zeroed() };
    lpsz_url.0 = CString::new("https://www.thisisafakewebsiteorelsetheantisanboxcheckwillfail4sure.com").unwrap().into_raw() as *mut u8;
    let lpsz_headers: PSTR = unsafe { mem::zeroed() };
    let website = unsafe { InternetOpenUrlA(internet_handle, lpsz_url, lpsz_headers, 0, 0, 0) };
    if website != 0 as _ {
        return true
    }
    false
}