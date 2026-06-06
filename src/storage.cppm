module;
#include <cstddef>
export module storage;

import std;

namespace qu
{
    export template <typename Key, typename Value>
    class Storage {
        using This = Storage;

        std::unordered_map<Key, std::size_t> map;
        std::vector<Value> pool;
    public:
        Storage() = default;
        ~Storage() {}

        auto put(this This &self, const Key &key, const Value &value) -> std::size_t {
            auto id = self.pool.size();
            self.map[key] = id;
            self.pool.push_back(value);
            return id;
        }

        auto get_id_by_key(this This &self, const Key &key) -> std::optional<std::size_t> {
            auto it = self.map.find(key);
            if (it == self.map.end()) {
                return std::nullopt;
            }
            return it->second;
        }

        auto get_by_key(this Storage& self, const Key &key) -> std::optional<Value&> {
            auto it = self.map.find(key);
            if (it != self.map.end()) {
                return self.map[it->second];
            }
            return std::nullopt;
        }

        auto get_by_id(this Storage& self, std::size_t id) -> std::optional<Value&> {
            if (id >= self.pool.size()) {
                return std::nullopt;
            }
            return self.pool[id];
        }
    };
}
