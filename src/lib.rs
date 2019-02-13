pub mod webcam;

extern crate libc;

use libc::c_char;
use libc::c_int;
use libc::c_uchar;
use libc::c_void;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn named_window(title: *mut c_char) -> () {
    let slice = unsafe { CStr::from_ptr(title) };
    webcam::named_window(slice.to_str().unwrap());
}
#[no_mangle]
pub extern "C" fn video_capture(camnum: c_int) -> *mut webcam::ffi::CvVideoCapture {
    unsafe { webcam::ffi::cv_video_capture(camnum) }
}
#[no_mangle]
pub extern "C" fn destroy_all_windows() -> c_void {
    unsafe { webcam::ffi::cv_destroy_all_windows() }
}
#[no_mangle]
pub extern "C" fn read(
    capture: *mut webcam::ffi::CvVideoCapture,
    frame: *mut webcam::ffi::Cv2Mat,
) -> c_void {
    unsafe { webcam::ffi::cv_read(capture, frame) }
}
#[no_mangle]
pub extern "C" fn imshow(winname: *const c_char, mat: *const webcam::ffi::Cv2Mat) -> c_void {
    unsafe { webcam::ffi::cv_imshow(winname, mat) }
}
#[no_mangle]
pub extern "C" fn wait_key(delay: c_int) -> c_int {
    unsafe { webcam::ffi::cv_wait_key(delay) }
}
#[no_mangle]
pub extern "C" fn release_video_capture(capture: *mut webcam::ffi::CvVideoCapture) -> c_void {
    unsafe { webcam::ffi::cv_release_video_capture(capture) }
}
#[no_mangle]
pub extern "C" fn create_mat() -> *mut webcam::ffi::Cv2Mat {
    unsafe { webcam::ffi::cv_create_mat() }
}
#[no_mangle]
pub extern "C" fn imwrite(filename: *const c_char, mat: *const webcam::ffi::Cv2Mat) -> c_int {
    unsafe { webcam::ffi::cv_imwrite(filename, mat) }
}
#[no_mangle]
pub extern "C" fn mat_cols(mat: *const webcam::ffi::Cv2Mat) -> c_int {
    unsafe { webcam::ffi::cv_mat_cols(mat) }
}
#[no_mangle]
pub extern "C" fn mat_data(mat: *const webcam::ffi::Cv2Mat) -> *mut c_uchar {
    unsafe { webcam::ffi::cv_mat_data(mat) }
}
#[no_mangle]
pub extern "C" fn imencode(
    ext: *const c_char,
    img: *const webcam::ffi::Cv2Mat,
    params: *const c_int,
) -> *mut webcam::ffi::ImgBuffer {
    unsafe { webcam::ffi::cv_imencode(ext, img, params) }
}
#[no_mangle]
pub extern "C" fn free_img_buffer(buf: *mut webcam::ffi::ImgBuffer) -> c_void {
    unsafe { webcam::ffi::cv_free_img_buffer(buf) }
}
