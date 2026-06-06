export module mod;

import std;

namespace qu
{
    export using ModuleId = std::size_t;
    export class Module {
    public:
        static auto init() -> Module {
            return {};
        }
    };
}
