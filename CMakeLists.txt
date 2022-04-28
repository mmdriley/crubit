cmake_minimum_required(VERSION 3.5)
project(crubit)

set(CMAKE_CXX_STANDARD 17)


# https://llvm.org/docs/CMake.html#embedding-llvm-in-your-project
# need to make sure only newest versions of clang and libclang-dev are installed
find_package(LLVM REQUIRED CONFIG)
find_package(Clang REQUIRED CONFIG)  # requires libclang-dev

message(STATUS "LLVM ${LLVM_PACKAGE_VERSION}")
# doesn't seem to be a corresponding CLANG_PACKAGE_VERSION

include_directories(${LLVM_INCLUDE_DIRS})
include_directories(${CLANG_INCLUDE_DIRS})

separate_arguments(LLVM_DEFINITIONS_LIST NATIVE_COMMAND ${LLVM_DEFINITIONS})
add_definitions(${LLVM_DEFINITIONS_LIST})


# abseil
# for now assume it's in a peer directory

add_subdirectory(../abseil-cpp/ abseil-cpp)
include_directories(../abseil-cpp/)


# this project

include_directories(./)
add_subdirectory(lifetime_annotations)
add_subdirectory(rs_bindings_from_cc)
