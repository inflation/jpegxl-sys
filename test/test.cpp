#include "../wrapper.h"
#include "../wrapper.hpp"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
  JpegxlDecoder *dec = JpegxlDecoderCreate(NULL);

  jpegxl::ThreadParallelRunner parallel_runner;

  JpegxlDecoderStatus status = JpegxlDecoderSetParallelRunner(
      dec, &parallel_runner.Runner, (void *)&parallel_runner);
  assert(status == JPEGXL_DEC_SUCCESS);

  status = JpegxlDecoderSubscribeEvents(dec, JPEGXL_DEC_BASIC_INFO |
                                                 JPEGXL_DEC_FULL_IMAGE);
  assert(status == JPEGXL_DEC_SUCCESS);

  FILE *fp = fopen("test/sample.jxl", "rb");
  fseek(fp, 0, SEEK_END);
  size_t file_size = ftell(fp);
  fseek(fp, 0, SEEK_SET);

  uint8_t *buffer = (uint8_t *)malloc(file_size * sizeof(uint8_t));
  fread(buffer, file_size, 1, fp);

  enum JpegxlSignature signature = JpegxlSignatureCheck(buffer, 2);
  assert(signature == JPEGXL_SIG_VALID);

  const uint8_t *next_in = buffer;
  size_t avail_in = file_size;
  status = JpegxlDecoderProcessInput(dec, &next_in, &avail_in);
  assert(status == JPEGXL_DEC_BASIC_INFO);

  JpegxlBasicInfo *basic_info =
      (JpegxlBasicInfo *)malloc(sizeof(JpegxlBasicInfo));
  status = JpegxlDecoderGetBasicInfo(dec, basic_info);
  assert(status == JPEGXL_DEC_SUCCESS);
  assert(basic_info->bits_per_sample == 8);
  assert(basic_info->xsize == 2122);
  assert(basic_info->ysize == 1433);
  free(basic_info);

  size_t size = 0;
  JpegxlPixelFormat pixel_format = {3, JPEGXL_TYPE_UINT8};
  status = JpegxlDecoderImageOutBufferSize(dec, &pixel_format, &size);
  assert(status == JPEGXL_DEC_SUCCESS);

  uint8_t *image_buffer = (uint8_t *)malloc(size * sizeof(uint8_t));
  status =
      JpegxlDecoderSetImageOutBuffer(dec, &pixel_format, image_buffer, size);
  assert(status == JPEGXL_DEC_SUCCESS);

  status = JpegxlDecoderProcessInput(dec, &next_in, &avail_in);
  assert(status == JPEGXL_DEC_FULL_IMAGE);

  FILE *out = fopen("out", "wb");
  fwrite(image_buffer, size * sizeof(uint8_t), 1, out);

  free(image_buffer);
  free(buffer);
  JpegxlDecoderDestroy(dec);
  return 0;
}