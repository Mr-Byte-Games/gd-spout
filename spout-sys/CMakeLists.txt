cmake_minimum_required(VERSION 3.30)
project(spout_wrapper)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 14)
set(SPOUT_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/deps/Spout2/BUILD")
set(SPOUT_DX12_INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/deps/Spout2/SPOUTSDK/SpoutDirectX/SpoutDX/SpoutDX12")

add_library(spout_wrapper STATIC src/spout.cpp)

target_include_directories(
        spout_wrapper
            PRIVATE
                "include"
                ${SPOUT_DX12_INCLUDE_DIR}
)

set(SPOUTDX12_LIB "${SPOUT_LIB_DIR}/SpoutDX12.lib")
set(SPOUTDX_LIB "${SPOUT_LIB_DIR}/SpoutDX.lib")

target_link_libraries(
        spout_wrapper
        PRIVATE
            ${SPOUTDX_LIB}
            ${SPOUTDX12_LIB}
)

install(
        TARGETS
        spout_wrapper
        EXPORT
        spout_wrapper-targets
        LIBRARY
        DESTINATION ${CMAKE_INSTALL_LIBDIR}
        ARCHIVE
        DESTINATION ${CMAKE_INSTALL_LIBDIR}
        RUNTIME
        DESTINATION ${CMAKE_INSTALL_BINDIR}
        INCLUDES
        DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
)