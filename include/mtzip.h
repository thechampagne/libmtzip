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
#ifndef __MTZIP_H__
#define __MTZIP_H__

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  void* zip_archive;
} mtzip_zip_archive_t;

extern mtzip_zip_archive_t mtzip_zip_archive_default(void);

extern int mtzip_zip_archive_add_file(mtzip_zip_archive_t* zip_archive, const char* fs_path, const char* archive_name);

extern int mtzip_zip_archive_add_file_from_bytes(mtzip_zip_archive_t* zip_archive, uint8_t* data, size_t data_len, const char* archive_name);

extern int mtzip_zip_archive_add_directory(mtzip_zip_archive_t* zip_archive, const char* archive_name);

extern int mtzip_zip_archive_compress(mtzip_zip_archive_t* zip_archive, size_t threads);

extern int mtzip_zip_archive_compress_autothread(mtzip_zip_archive_t* zip_archive);

extern int mtzip_zip_archive_write(mtzip_zip_archive_t* zip_archive, const char* file_name, size_t threads);

extern int mtzip_zip_archive_write_autothread(mtzip_zip_archive_t* zip_archive, const char* file_name);

extern void mtzip_zip_archive_clean(mtzip_zip_archive_t* zip_archive);

#ifdef __cplusplus
}
#endif

#endif // __MTZIP_H__
