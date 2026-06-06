module;
#include <string_view>
#include <vector>
export module source;

import std;

namespace qu
{
    export using SourceId = std::size_t;
    export class Source {
        using This = Source;
        std::string content;
        std::string path;
        std::vector<std::string> lines;

        Source(
           const std::string &content,
           const std::string &path,
           const std::vector<std::string> &lines
               ): content(std::move(content)), path(std::move(path)), lines(std::move(lines)) {
        }
    public:
        static auto init(const std::string &path) -> std::optional<Source> {
            if (!std::filesystem::exists(path)) {
                return std::nullopt;
            }

            auto file = std::fstream(path, std::ios::openmode(std::ios::in | std::ios::binary));
            if (!file.is_open()) {
                return std::nullopt;
            }
            auto content = std::string((std::istreambuf_iterator<char>(file)),
                                           std::istreambuf_iterator<char>());
            std::vector<std::string> lines = {};
            auto current = 0u;
            auto begin   = 0u;
            while (current < content.size()) {
                if (content[current] == '\n') {
                    auto length = current - begin;
                    auto line   = content.substr(begin, length);
                    lines.push_back(line);
                    begin = current + 1;
                }
                current += 1;
            }

            if (begin < current) {
                auto length = current - begin;
                auto line   = content.substr(begin, length);
                lines.push_back(line);
            }
            return Source(content, path, lines);
        }

        auto get_content(this This &self) -> std::string_view {
            return self.content;
        }

        auto get_lines(this This &self) -> std::vector<std::string> & {
            return self.lines;
        }

        auto get_path(this This &self) -> std::string_view {
            return self.path;
        }
    };
} // namespace qu
