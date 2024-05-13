/*
 * Copyright (c) 2022 XXIV
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use std::{
    ffi::CStr,
    fs::File,
    os::raw::{c_char, c_int, c_void},
    path::Path,
    slice,
};

use mtzip::ZipArchive;

#[repr(C)]
struct mtzip_zip_archive_t {
    zip_archive: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn mtzip_zip_archive_default() -> mtzip_zip_archive_t {
    let zipper = ZipArchive::default();
    return mtzip_zip_archive_t {
        zip_archive: Box::into_raw(Box::new(zipper)) as *mut c_void,
    };
}

#[no_mangle]
unsafe extern "C" fn mtzip_zip_archive_add_file(
    zip_archive: *mut mtzip_zip_archive_t,
    fs_path: *const c_char,
    archive_name: *const c_char,
) -> c_int {
    let fs_path_rs = match CStr::from_ptr(fs_path).to_str() {
        Ok(v) => v,
        Err(_) => return -1,
    };
    let archive_name_rs = match CStr::from_ptr(archive_name).to_str() {
        Ok(v) => v,
        Err(_) => return -1,
    };
    let zipper = &*((*zip_archive).zip_archive as *mut ZipArchive);
    zipper.add_file_from_fs(
        Path::new(fs_path_rs),
        archive_name_rs.to_owned(),
        None,
        None,
    );
    0
}

#[no_mangle]
unsafe extern "C" fn mtzip_zip_archive_add_file_from_bytes(
    zip_archive: *mut mtzip_zip_archive_t,
    data: *const u8,
    data_len: usize,
    archive_name: *const c_char,
) -> c_int {
    let data_rs = slice::from_raw_parts(data, data_len);
    let archive_name_rs = match CStr::from_ptr(archive_name).to_str() {
        Ok(v) => v,
        Err(_) => return -1,
    };
    let zipper = &*((*zip_archive).zip_archive as *mut ZipArchive);
    zipper.add_file_from_memory(data_rs, archive_name_rs.to_owned(), None, None, None, None);
    0
}

#[no_mangle]
unsafe extern "C" fn mtzip_zip_archive_add_directory(
    zip_archive: *mut mtzip_zip_archive_t,
    archive_name: *const c_char,
) -> c_int {
    let archive_name_rs = match CStr::from_ptr(archive_name).to_str() {
        Ok(v) => v,
        Err(_) => return -1,
    };
    let zipper = &*((*zip_archive).zip_archive as *mut ZipArchive);
    zipper.add_directory(archive_name_rs.to_owned(), None);
    0
}

#[no_mangle]
unsafe extern "C" fn mtzip_zip_archive_compress(
    zip_archive: *mut mtzip_zip_archive_t,
    threads: usize,
) -> c_int {
    let zipper = &*((*zip_archive).zip_archive as *mut ZipArchive);
    zipper.compress_with_threads(threads);
    0
}

#[no_mangle]
unsafe extern "C" fn mtzip_zip_archive_compress_autothread(
    zip_archive: *mut mtzip_zip_archive_t,
) -> c_int {
    let zipper = &*((*zip_archive).zip_archive as *mut ZipArchive);
    zipper.compress();
    0
}

#[no_mangle]
unsafe extern "C" fn mtzip_zip_archive_write(
    zip_archive: *mut mtzip_zip_archive_t,
    file_name: *const c_char,
    threads: usize,
) -> c_int {
    let file_name_rs = match CStr::from_ptr(file_name).to_str() {
        Ok(v) => v,
        Err(_) => return -1,
    };
    let zipper = &*((*zip_archive).zip_archive as *mut ZipArchive);
    let mut file = match File::create(file_name_rs) {
        Ok(v) => v,
        Err(_) => return -1,
    };
    let result = if threads == 0 {
        zipper.write_with_threads(&mut file, 1)
    } else {
        zipper.write_with_threads(&mut file, threads)
    };
    match result {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn mtzip_zip_archive_write_autothread(
    zip_archive: *mut mtzip_zip_archive_t,
    file_name: *const c_char,
) -> c_int {
    let file_name_rs = match CStr::from_ptr(file_name).to_str() {
        Ok(v) => v,
        Err(_) => return -1,
    };
    let zipper = &*((*zip_archive).zip_archive as *mut ZipArchive);
    let mut file = match File::create(file_name_rs) {
        Ok(v) => v,
        Err(_) => return -1,
    };
    match zipper.write(&mut file) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn mtzip_zip_archive_clean(zip_archive: *mut mtzip_zip_archive_t) {
    if !zip_archive.is_null() {
        let _ = Box::from_raw((*zip_archive).zip_archive as *mut ZipArchive);
    }
}
