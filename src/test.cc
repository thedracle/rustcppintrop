#include "rustcppinterop/include/test.h"
#include "rustcppinterop/src/main.rs.h"
#include <stdio.h>
#include <memory>

Test::Test() {
    add_to_roster("Test", 30);
    printf("Hello world: %d\n", age_for_name("Test"));
}

std::unique_ptr<Test> new_test() {
  return std::make_unique<Test>();
}
