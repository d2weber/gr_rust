#include "gnuradio/basic_block.h"
#include "gnuradio/blocks/file_sink.h"
#include "gnuradio/blocks/vector_source.h"
#include "gnuradio/hier_block2.h"
#include "gnuradio/top_block.h"

#pragma once

namespace gr {
namespace blocks {

inline vector_source_s::sptr
make_vector_source_s(const std::vector<std::int16_t> &data,
                     bool repeat = false) {
  return vector_source_s::make(data, repeat, 1, std::vector<tag_t>());
}
inline std::shared_ptr<gr::basic_block>
cast_vector_source_s(std::shared_ptr<vector_source_s> arg) {
  return arg;
}

inline file_sink::sptr make_file_sink(const char *filename) {
  return file_sink::make(sizeof(short) * 1, filename, false);
}
inline std::shared_ptr<gr::basic_block>
cast_file_sink(std::shared_ptr<file_sink> arg) {
  return arg;
}

} // namespace blocks
} // namespace gr
