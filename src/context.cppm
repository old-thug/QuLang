export module context;

import std;
import storage;
import source;
import mod;


namespace qu
{
    export using SourceStorage = Storage<std::string, Source>;
    export using ModuleStorage = Storage<std::string, Module>;

    export class Context
    {
        using This = Context;

        SourceStorage sources;
        ModuleStorage modules;
    public:
        static auto init() -> This {
            return This();
        }

        auto get_or_put_new_module(this This &self, const std::string &name)
            -> ModuleId {
            auto mod = self.modules.get_id_by_key(name);
            if (mod.has_value()) {
                return mod.value();
            }

            auto new_mod = Module::init();
            return self.modules.put(name, new_mod);
        }

        auto source(this This &self, const std::string& path) -> std::optional<SourceId> {
            if (!std::filesystem::exists(path)) return std::nullopt;
            auto canon_path = std::filesystem::canonical(path);
            auto source = self.sources.get_id_by_key(canon_path);
            if (!source.has_value()) {
                auto new_source = Source::init(path);
                if (new_source.has_value()) {
                    return self.sources.put(path, new_source.value());
                } else return std::nullopt;
            }
            return source.value();
        }

        auto get_source(this This &self, SourceId id)
            -> std::optional<Source &> {
            return self.sources.get_by_id(id);
        }
    };
}
