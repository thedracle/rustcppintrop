#pragma once
#include "rust/cxx.h"
#include <memory>

class Test {
public:
    Test();
};

std::unique_ptr<Test> new_test();
