use super::KeyBuilder;
use crate::get_slice_mut;
use libc;
use std;

/// [C binding] Generate a random key.
///
/// ## Parameters
///
/// - `buff`: pointer to the buffer that will be filled with the random key
/// - `buff_len`: length of the buffer, in bytes
///
/// ## Return code
///
/// 0 in case of success, 1 if anything failed.
///
/// ## Examples
/// ```c
/// char key[DEFAULT_KEY_SIZE + 1] = {0};
/// int32_t ret = libreauth_keygen(key, DEFAULT_KEY_SIZE);
/// if (ret != EXIT_SUCCESS) {
///     // Handle the error.
/// }
/// ```
#[no_mangle]
pub extern "C" fn libreauth_keygen(buff: *mut u8, buff_len: libc::size_t) -> i32 {
    let key_size = buff_len as usize;
    if key_size == 0 || buff.is_null() {
        return 1;
    };
    let key = get_slice_mut!(buff, key_size);
    let out = KeyBuilder::new().size(key_size).generate().as_vec();
    let len = out.len();
    for i in 0..len {
        key[i] = out[i];
    }
    0
}
