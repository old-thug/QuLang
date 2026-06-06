#include "def.hpp"
#include <cstddef>
import context;
import std;
import parser;
import diagnostic;

#include <cstdio>
#include <cstdlib>

struct CommandArgs {
    std::optional<std::string> input_path;
    std::string output_path;
    enum {
        Codegen = 0,
        Object,
        Executable,
    } target_stage;
};

auto parse_args(CommandArgs &cmdargs, std::size_t argc, char **argv) {
    auto counter           = 1u;
    const auto getArgument = [argc, argv, &counter](auto flag_name) {
        if (counter >= argc) {
            std::println(stderr, "error: '{}' expectes an argument",
                             flag_name);
            std::exit(1);
        }
        return argv[counter++];
    };
    while (counter < argc) {
        std::string arg = argv[counter++];
        if (arg.starts_with("-")) {
            std::string flag_name = arg.substr(1);
            if (flag_name.compare("o") == 0) {
                cmdargs.output_path = getArgument(arg);
            } else {
                std::println(stderr, "error: flag '{}' is unknown", arg);
                std::exit(1);
            }
        } else {
            if (cmdargs.input_path.has_value()) {
                std::println(stderr, "warning: unused argument `{}`", arg);
            } else {
                cmdargs.input_path = arg;
            }
        }
    }
}

auto main(int argc, char **argv) -> int {
    auto cmdargs = CommandArgs{};
    parse_args(cmdargs, argc, argv);

    if (!cmdargs.input_path.has_value()) {
        std::println(stderr, "error: no input specified. nothing to do...");
        return 2;
    }

    auto context = qu::Context::init();
    auto source =
        context.source(cmdargs.input_path.value())
            .or_else([cmdargs]() {
                std::println(stderr, "error: could not open file '{}'",
                                 cmdargs.input_path.value());
                std::exit(1);
                return std::optional<std::size_t>{};
            })
            .value();
    auto root_module = context.get_or_put_new_module("root");
    // auto std_module = context.get_or_put_new_module("std");
    // context.link_module_from_bin(std_module, "path/to/precompiled/module");
    auto diags = qu::DiagnosticPool(20);
    auto parser =
        qu::parser::Parser::init(&context, &diags, source)
            .or_else([]() {
                std::println(stderr, "fatal: could not initialize parser");
                std::exit(0);
                return std::optional<qu::parser::Parser>{};
            })
            .value();
    auto result = qu::parser::parse_module(parser);
    if (result.has_value()) {
        UNREACHABLE();
    } else {
        for (auto diag : diags.buffer) {
            auto buffer = std::stringstream();
            diag.render(context, buffer);
            std::println(stderr, "{}", buffer.str());
        }
        return 1;
    }
    return 0;
}
