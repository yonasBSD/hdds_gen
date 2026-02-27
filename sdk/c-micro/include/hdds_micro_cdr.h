/**
 * @file hdds_micro_cdr.h
 * @brief Minimal CDR2 encoder/decoder for embedded systems
 *
 * Header-only, zero dependencies beyond stdint.h and string.h.
 * Compatible with C89/C99, suitable for AVR, STM32, PIC, ESP32.
 *
 * @copyright Copyright (c) 2025-2026 naskel.com
 * @license MIT
 *
 * Generated code uses this runtime for serialization.
 * Usage:
 *   uint8_t buffer[256];
 *   hdds_cdr_t cdr;
 *   hdds_cdr_init(&cdr, buffer, sizeof(buffer));
 *   temperature_encode(&temp, &cdr);
 *   int len = cdr.pos;
 */

#ifndef HDDS_MICRO_CDR_H
#define HDDS_MICRO_CDR_H

#include <stdint.h>
#include <string.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Error codes */
#define HDDS_CDR_OK           0
#define HDDS_CDR_ERR_NULL    -1
#define HDDS_CDR_ERR_OVERFLOW -2
#define HDDS_CDR_ERR_UNDERFLOW -3
#define HDDS_CDR_ERR_INVALID  -4

/**
 * @brief CDR encoder/decoder state
 */
typedef struct hdds_cdr {
    uint8_t* buf;      /**< Buffer pointer */
    uint32_t size;     /**< Buffer size */
    uint32_t pos;      /**< Current position */
} hdds_cdr_t;

/* ============================================================================
 * Initialization
 * ============================================================================ */

/**
 * @brief Initialize CDR encoder/decoder
 */
static inline void hdds_cdr_init(hdds_cdr_t* cdr, uint8_t* buf, uint32_t size) {
    cdr->buf = buf;
    cdr->size = size;
    cdr->pos = 0;
}

/**
 * @brief Reset position to beginning
 */
static inline void hdds_cdr_reset(hdds_cdr_t* cdr) {
    cdr->pos = 0;
}

/**
 * @brief Get remaining space for writing
 */
static inline uint32_t hdds_cdr_remaining(const hdds_cdr_t* cdr) {
    return cdr->size - cdr->pos;
}

/* ============================================================================
 * Alignment (CDR2 requires natural alignment)
 * ============================================================================ */

static inline void hdds_cdr_align(hdds_cdr_t* cdr, uint32_t alignment) {
    uint32_t mask = alignment - 1u;
    cdr->pos = (cdr->pos + mask) & ~mask;
}

/* ============================================================================
 * Write primitives (Little-Endian CDR2)
 * ============================================================================ */

static inline int32_t hdds_cdr_write_u8(hdds_cdr_t* cdr, uint8_t v) {
    if (cdr->pos + 1u > cdr->size) { return HDDS_CDR_ERR_OVERFLOW; }
    cdr->buf[cdr->pos++] = v;
    return HDDS_CDR_OK;
}

static inline int32_t hdds_cdr_write_i8(hdds_cdr_t* cdr, int8_t v) {
    return hdds_cdr_write_u8(cdr, (uint8_t)v);
}

static inline int32_t hdds_cdr_write_bool(hdds_cdr_t* cdr, uint8_t v) {
    return hdds_cdr_write_u8(cdr, v ? 1u : 0u);
}

static inline int32_t hdds_cdr_write_u16(hdds_cdr_t* cdr, uint16_t v) {
    hdds_cdr_align(cdr, 2u);
    if (cdr->pos + 2u > cdr->size) { return HDDS_CDR_ERR_OVERFLOW; }
    cdr->buf[cdr->pos++] = (uint8_t)(v);
    cdr->buf[cdr->pos++] = (uint8_t)(v >> 8);
    return HDDS_CDR_OK;
}

static inline int32_t hdds_cdr_write_i16(hdds_cdr_t* cdr, int16_t v) {
    return hdds_cdr_write_u16(cdr, (uint16_t)v);
}

static inline int32_t hdds_cdr_write_u32(hdds_cdr_t* cdr, uint32_t v) {
    hdds_cdr_align(cdr, 4u);
    if (cdr->pos + 4u > cdr->size) { return HDDS_CDR_ERR_OVERFLOW; }
    cdr->buf[cdr->pos++] = (uint8_t)(v);
    cdr->buf[cdr->pos++] = (uint8_t)(v >> 8);
    cdr->buf[cdr->pos++] = (uint8_t)(v >> 16);
    cdr->buf[cdr->pos++] = (uint8_t)(v >> 24);
    return HDDS_CDR_OK;
}

static inline int32_t hdds_cdr_write_i32(hdds_cdr_t* cdr, int32_t v) {
    return hdds_cdr_write_u32(cdr, (uint32_t)v);
}

static inline int32_t hdds_cdr_write_u64(hdds_cdr_t* cdr, uint64_t v) {
    hdds_cdr_align(cdr, 8u);
    if (cdr->pos + 8u > cdr->size) { return HDDS_CDR_ERR_OVERFLOW; }
    cdr->buf[cdr->pos++] = (uint8_t)(v);
    cdr->buf[cdr->pos++] = (uint8_t)(v >> 8);
    cdr->buf[cdr->pos++] = (uint8_t)(v >> 16);
    cdr->buf[cdr->pos++] = (uint8_t)(v >> 24);
    cdr->buf[cdr->pos++] = (uint8_t)(v >> 32);
    cdr->buf[cdr->pos++] = (uint8_t)(v >> 40);
    cdr->buf[cdr->pos++] = (uint8_t)(v >> 48);
    cdr->buf[cdr->pos++] = (uint8_t)(v >> 56);
    return HDDS_CDR_OK;
}

static inline int32_t hdds_cdr_write_i64(hdds_cdr_t* cdr, int64_t v) {
    return hdds_cdr_write_u64(cdr, (uint64_t)v);
}

static inline int32_t hdds_cdr_write_f32(hdds_cdr_t* cdr, float v) {
    uint32_t bits;
    memcpy(&bits, &v, sizeof(bits));
    return hdds_cdr_write_u32(cdr, bits);
}

static inline int32_t hdds_cdr_write_f64(hdds_cdr_t* cdr, double v) {
    uint64_t bits;
    memcpy(&bits, &v, sizeof(bits));
    return hdds_cdr_write_u64(cdr, bits);
}

/**
 * @brief Write string (CDR: length prefix + bytes + null terminator)
 */
static inline int32_t hdds_cdr_write_string(hdds_cdr_t* cdr, const char* s) {
    uint32_t len = (uint32_t)strlen(s) + 1u; /* +1 for null terminator */
    int32_t rc = hdds_cdr_write_u32(cdr, len);
    if (rc != HDDS_CDR_OK) { return rc; }
    if (cdr->pos + len > cdr->size) { return HDDS_CDR_ERR_OVERFLOW; }
    memcpy(&cdr->buf[cdr->pos], s, len);
    cdr->pos += len;
    return HDDS_CDR_OK;
}

/**
 * @brief Write sequence length prefix
 */
static inline int32_t hdds_cdr_write_seq_len(hdds_cdr_t* cdr, uint32_t len) {
    return hdds_cdr_write_u32(cdr, len);
}

/**
 * @brief Write raw bytes (for sequence<uint8>)
 */
static inline int32_t hdds_cdr_write_bytes(hdds_cdr_t* cdr, const uint8_t* data, uint32_t len) {
    int32_t rc = hdds_cdr_write_u32(cdr, len);
    if (rc != HDDS_CDR_OK) { return rc; }
    if (cdr->pos + len > cdr->size) { return HDDS_CDR_ERR_OVERFLOW; }
    memcpy(&cdr->buf[cdr->pos], data, len);
    cdr->pos += len;
    return HDDS_CDR_OK;
}

/* ============================================================================
 * Read primitives (Little-Endian CDR2)
 * ============================================================================ */

static inline int32_t hdds_cdr_read_u8(hdds_cdr_t* cdr, uint8_t* v) {
    if (cdr->pos + 1u > cdr->size) { return HDDS_CDR_ERR_UNDERFLOW; }
    *v = cdr->buf[cdr->pos++];
    return HDDS_CDR_OK;
}

static inline int32_t hdds_cdr_read_i8(hdds_cdr_t* cdr, int8_t* v) {
    uint8_t u;
    int32_t rc = hdds_cdr_read_u8(cdr, &u);
    *v = (int8_t)u;
    return rc;
}

static inline int32_t hdds_cdr_read_bool(hdds_cdr_t* cdr, uint8_t* v) {
    uint8_t u;
    int32_t rc = hdds_cdr_read_u8(cdr, &u);
    *v = (u != 0u) ? 1u : 0u;
    return rc;
}

static inline int32_t hdds_cdr_read_u16(hdds_cdr_t* cdr, uint16_t* v) {
    hdds_cdr_align(cdr, 2u);
    if (cdr->pos + 2u > cdr->size) { return HDDS_CDR_ERR_UNDERFLOW; }
    *v = (uint16_t)cdr->buf[cdr->pos]
       | ((uint16_t)cdr->buf[cdr->pos + 1u] << 8);
    cdr->pos += 2u;
    return HDDS_CDR_OK;
}

static inline int32_t hdds_cdr_read_i16(hdds_cdr_t* cdr, int16_t* v) {
    uint16_t u;
    int32_t rc = hdds_cdr_read_u16(cdr, &u);
    *v = (int16_t)u;
    return rc;
}

static inline int32_t hdds_cdr_read_u32(hdds_cdr_t* cdr, uint32_t* v) {
    hdds_cdr_align(cdr, 4u);
    if (cdr->pos + 4u > cdr->size) { return HDDS_CDR_ERR_UNDERFLOW; }
    *v = (uint32_t)cdr->buf[cdr->pos]
       | ((uint32_t)cdr->buf[cdr->pos + 1u] << 8)
       | ((uint32_t)cdr->buf[cdr->pos + 2u] << 16)
       | ((uint32_t)cdr->buf[cdr->pos + 3u] << 24);
    cdr->pos += 4u;
    return HDDS_CDR_OK;
}

static inline int32_t hdds_cdr_read_i32(hdds_cdr_t* cdr, int32_t* v) {
    uint32_t u;
    int32_t rc = hdds_cdr_read_u32(cdr, &u);
    *v = (int32_t)u;
    return rc;
}

static inline int32_t hdds_cdr_read_u64(hdds_cdr_t* cdr, uint64_t* v) {
    hdds_cdr_align(cdr, 8u);
    if (cdr->pos + 8u > cdr->size) { return HDDS_CDR_ERR_UNDERFLOW; }
    *v = (uint64_t)cdr->buf[cdr->pos]
       | ((uint64_t)cdr->buf[cdr->pos + 1u] << 8)
       | ((uint64_t)cdr->buf[cdr->pos + 2u] << 16)
       | ((uint64_t)cdr->buf[cdr->pos + 3u] << 24)
       | ((uint64_t)cdr->buf[cdr->pos + 4u] << 32)
       | ((uint64_t)cdr->buf[cdr->pos + 5u] << 40)
       | ((uint64_t)cdr->buf[cdr->pos + 6u] << 48)
       | ((uint64_t)cdr->buf[cdr->pos + 7u] << 56);
    cdr->pos += 8u;
    return HDDS_CDR_OK;
}

static inline int32_t hdds_cdr_read_i64(hdds_cdr_t* cdr, int64_t* v) {
    uint64_t u;
    int32_t rc = hdds_cdr_read_u64(cdr, &u);
    *v = (int64_t)u;
    return rc;
}

static inline int32_t hdds_cdr_read_f32(hdds_cdr_t* cdr, float* v) {
    uint32_t bits;
    int32_t rc = hdds_cdr_read_u32(cdr, &bits);
    if (rc == HDDS_CDR_OK) {
        memcpy(v, &bits, sizeof(*v));
    }
    return rc;
}

static inline int32_t hdds_cdr_read_f64(hdds_cdr_t* cdr, double* v) {
    uint64_t bits;
    int32_t rc = hdds_cdr_read_u64(cdr, &bits);
    if (rc == HDDS_CDR_OK) {
        memcpy(v, &bits, sizeof(*v));
    }
    return rc;
}

/**
 * @brief Read string into fixed-size buffer
 * @param cdr CDR decoder
 * @param s Destination buffer
 * @param max_len Maximum length including null terminator
 */
static inline int32_t hdds_cdr_read_string(hdds_cdr_t* cdr, char* s, uint32_t max_len) {
    uint32_t len;
    int32_t rc = hdds_cdr_read_u32(cdr, &len);
    if (rc != HDDS_CDR_OK) { return rc; }
    if (len > max_len) { return HDDS_CDR_ERR_OVERFLOW; }
    if (cdr->pos + len > cdr->size) { return HDDS_CDR_ERR_UNDERFLOW; }
    memcpy(s, &cdr->buf[cdr->pos], len);
    cdr->pos += len;
    return HDDS_CDR_OK;
}

/**
 * @brief Read sequence length prefix
 */
static inline int32_t hdds_cdr_read_seq_len(hdds_cdr_t* cdr, uint32_t* len) {
    return hdds_cdr_read_u32(cdr, len);
}

/**
 * @brief Read raw bytes
 */
static inline int32_t hdds_cdr_read_bytes(hdds_cdr_t* cdr, uint8_t* data, uint32_t max_len, uint32_t* actual_len) {
    int32_t rc = hdds_cdr_read_u32(cdr, actual_len);
    if (rc != HDDS_CDR_OK) { return rc; }
    if (*actual_len > max_len) { return HDDS_CDR_ERR_OVERFLOW; }
    if (cdr->pos + *actual_len > cdr->size) { return HDDS_CDR_ERR_UNDERFLOW; }
    memcpy(data, &cdr->buf[cdr->pos], *actual_len);
    cdr->pos += *actual_len;
    return HDDS_CDR_OK;
}

#ifdef __cplusplus
}
#endif

#endif /* HDDS_MICRO_CDR_H */
