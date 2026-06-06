module;
#include "def.hpp"
#include <cassert>
export module diagnostic;

import std;
import lexer.locus;
import context;

namespace qu
{
    static const int LINE_THRESHOLD = 5;
    export enum class Severity { Error, Warning, Note };

    export struct Label {
        std::string label;
        lexer::Locus locus;
        bool is_primary = false;
    };

    struct LabelGroup {
        std::size_t source_id;
        std::vector<Label> labels;
    };

    export struct Diagnostic {
        using This = Diagnostic;
        Severity severity;
        std::string code;
        std::string message;
        std::vector<Label> labels;

        Diagnostic(Severity severity, std::string message, lexer::Locus locus,
                   std::string label)
            : severity(severity), code(""), message(std::move(message)),
              labels() {
            labels.push_back({
                .label = std::move(label),
                .locus = locus,
                .is_primary = true,
                });
        }

        auto with_code(this auto&& self, std::string code) {
            self.code = std::move(code);
            return std::forward<decltype(self)>(self);
        }

        auto add_label(this auto&& self, Label label) {
            self.labels.push_back(std::move(label));
            return std::forward<decltype(self)>(self);
        }

        auto render(this This const &self, qu::Context &ctx, std::stringstream &out) -> void {
            switch (self.severity) {
            case Severity::Error: out << "error"; break;
            case Severity::Warning: out << "warning"; break;
            case Severity::Note: out << "note"; break;
            }
            if (!self.code.empty()) {
                out << std::format("[{}]", self.code);
            }
            out << std::format(": {}\n", self.message);

            if (self.labels.empty()) return;

            auto sorted_labels = self.labels;
            std::sort(sorted_labels.begin(), sorted_labels.end(),
                      [](const Label &first, const Label &second) {
                          if (first.locus.source_id != second.locus.source_id) {
                              return first.locus.source_id < second.locus.source_id;
                          }
                          if (first.locus.first_line != second.locus.first_line) {
                              return first.locus.first_line < second.locus.first_line;
                          }
                          return first.locus.first_col < second.locus.first_col;
                      });

            auto label_groups = group_labels_source_id(sorted_labels);

            for (const auto &group : label_groups) {
                if (group.labels.empty()) continue;

                const auto source_opt = ctx.get_source(group.source_id);
                if (!source_opt) {
                    out << " --> <no source info>\n";
                    continue;
                }
                auto lines = source_opt->get_lines();

                const Label* primary = nullptr;
                for (const auto &label : group.labels) {
                    if (label.is_primary) { primary = &label; break; }
                }
                if (!primary) primary = &group.labels.front();

                std::map<int, std::vector<Label>> line_to_labels;
                int max_line_num = 0;
                for (const auto &label : group.labels) {
                    line_to_labels[label.locus.first_line].push_back(label);
                    if (label.locus.first_line > max_line_num) {
                        max_line_num = label.locus.first_line;
                    }
                }

                int gutter_width = std::max(2, static_cast<int>(std::to_string(max_line_num).length()));
                std::string empty_gutter = std::string(gutter_width, ' ');

                out << std::format("{}--> {}:{}:{}\n",
                                  empty_gutter,
                                  source_opt->get_path(),
                                  primary->locus.first_line,
                                  primary->locus.first_col);
                out << std::format("{} |\n", empty_gutter);

                for (auto const& [line_num, labels_on_line] : line_to_labels) {
                    if (line_num - 1 >= lines.size()) continue;
                    const auto& line_text = lines[line_num - 1];

                    std::string line_num_str = std::to_string(line_num);
                    std::string line_gutter = std::string(gutter_width - line_num_str.length(), ' ') + line_num_str;

                    out << std::format("{} | {}\n", line_gutter, line_text);

                    out << std::format("{} | ", empty_gutter);
                    int current_col = 1;

                    auto sorted_line_labels = labels_on_line;
                    std::sort(sorted_line_labels.begin(), sorted_line_labels.end(),
                              [](const Label& a, const Label& b) { return a.locus.first_col < b.locus.first_col; });

                    for (const auto& label : sorted_line_labels) {
                        while (current_col < label.locus.first_col) {
                            out << " ";
                            current_col++;
                        }
                        char caret = label.is_primary ? '^' : '-';
                        while (current_col < label.locus.last_col) {
                            out << caret;
                            current_col++;
                        }
                    }
                    out << "\n";

                    for (size_t i = 0; i < sorted_line_labels.size(); ++i) {
                        out << std::format("{} | ", empty_gutter);
                        current_col = 1;
                        size_t active_labels_count = sorted_line_labels.size() - i;

                        for (size_t j = 0; j < active_labels_count; ++j) {
                            const auto& lbl = sorted_line_labels[j];
                            while (current_col < lbl.locus.first_col) {
                                out << " ";
                                current_col++;
                            }

                            if (j == active_labels_count - 1) {
                                out << std::format(" {}", lbl.label);
                            } else {
                                out << "|";
                                current_col++;
                            }
                        }
                        out << "\n";
                    }
                }
                out << std::format("{} |\n", empty_gutter);
            }
        }

        static auto group_labels_source_id(std::vector<Label> labels) -> std::vector<LabelGroup> {
            if (labels.empty()) return {};

            auto groups = std::vector<LabelGroup>{};

            std::size_t current_id = labels.front().locus.source_id;
            int last_line = labels.front().locus.last_line;

            std::vector<Label> current_group;
            current_group.push_back(std::move(labels.front()));

            for (std::size_t n = 1; n < labels.size(); ++n) {
                auto&& label = labels[n];

                if (label.locus.source_id == current_id &&
                    (label.locus.first_line - last_line) <= LINE_THRESHOLD) {

                    last_line = label.locus.last_line;
                    current_group.push_back(std::move(label));
                } else {
                    groups.push_back({ current_id, std::move(current_group) });

                    current_id = label.locus.source_id;
                    last_line = label.locus.last_line;
                    current_group.clear();
                    current_group.push_back(std::move(label));
                }
            }

            if (!current_group.empty()) {
                groups.push_back({ current_id, std::move(current_group) });
            }

            return groups;
        }
    };

    export struct DiagnosticPool {
        std::vector<Diagnostic> buffer;
    private:
        using This = DiagnosticPool;
        std::size_t error_count = 0;
        std::size_t warning_count = 0;
        std::size_t error_cap;

    public:
        DiagnosticPool(std::size_t error_cap)
            : buffer(), error_count(0), warning_count(0), error_cap(error_cap) {}

        auto add(this This &self, Diagnostic diag) -> void {
            switch (diag.severity) {
            case Severity::Error:
                if (self.count_errors() >= self.error_cap) return;
                self.error_count += 1;
                break;
            case Severity::Warning:
                self.warning_count += 1;
                break;
            default: break;
            }
            self.buffer.push_back(std::move(diag));
        }

        auto count_errors(this This const &self) -> std::size_t {
            return self.error_count;
        }

        auto count_warnings(this This const &self) -> std::size_t {
            return self.warning_count;
        }
    };
} // namespace qu
