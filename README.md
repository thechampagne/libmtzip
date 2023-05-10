# libmtzip

[![](https://img.shields.io/github/v/tag/thechampagne/libmtzip?label=version)](https://github.com/thechampagne/libmtzip/releases/latest) [![](https://img.shields.io/github/license/thechampagne/libmtzip)](https://github.com/thechampagne/libmtzip/blob/main/LICENSE)

A **C** library for making zip files, focused on multithreading the process.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/libmtzip.git
```
#### 2. Navigate to the root
```
cd libmtzip
```
#### 3. Build the project
```
cargo build --release
```

### API

```c
typedef struct mtzip_zip_archive_t mtzip_zip_archive_t;

mtzip_zip_archive_t mtzip_zip_archive_default(void);

int mtzip_zip_archive_add_file(mtzip_zip_archive_t* zip_archive, const char* fs_path, const char* archive_name);

int mtzip_zip_archive_add_file_from_bytes(mtzip_zip_archive_t* zip_archive, uint8_t* data, size_t data_len, const char* archive_name);

int mtzip_zip_archive_add_directory(mtzip_zip_archive_t* zip_archive, const char* archive_name);

int mtzip_zip_archive_compress(mtzip_zip_archive_t* zip_archive, size_t threads);

int mtzip_zip_archive_write(mtzip_zip_archive_t* zip_archive, const char* file_name, size_t threads);

void mtzip_zip_archive_clean(mtzip_zip_archive_t* zip_archive);
```

### References
 - [mtzip](https://github.com/JohnTheCoolingFan/mtzip)

### License

This repo is released under the [MIT](https://github.com/thechampagne/libmtzip/blob/main/LICENSE).
