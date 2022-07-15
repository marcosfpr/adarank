#######################################################
### LTR++                                           ###
#######################################################

# Project Information
cmake_minimum_required(VERSION 3.16.0 FATAL_ERROR)

project(ltrpp VERSION 1.0.0)
set(CMAKE_CXX_STANDARD 17)

message(STATUS "Is the C++ compiler loaded? ${CMAKE_CXX_COMPILER_LOADED}")
if(CMAKE_CXX_COMPILER_LOADED)
        message(STATUS "The C++ compiler ID is: ${CMAKE_CXX_COMPILER_ID}")
        message(STATUS "Is the C++ from GNU? ${CMAKE_COMPILER_IS_GNUCXX}")
        message(STATUS "The C++ compiler version is: ${CMAKE_CXX_COMPILER_VERSION}")
endif()

# CMake dependencies for installer
include(CMakePackageConfigHelpers)
include(GNUInstallDirs)

# Finding or downloading packages
list(APPEND CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/cmake)
message("CMAKE_MODULE_PATH=${CMAKE_MODULE_PATH}")

# check if this is a master project or a subdirectory of another project
if (${CMAKE_CURRENT_SOURCE_DIR} STREQUAL ${CMAKE_SOURCE_DIR})
        set(MASTER_PROJECT ON)
else()
        set(MASTER_PROJECT OFF)
endif()

set(LTRPP_ROOT_DIR ${CMAKE_CURRENT_SOURCE_DIR})

# check if we are in debug mode
if(CMAKE_BUILD_TYPE MATCHES DEBUG)
        set(DEBUG_MODE ON)
        set(NOT_DEBUG_MODE OFF)
else()
        set(DEBUG_MODE OFF)
        set(NOT_DEBUG_MODE ON)
endif()

#######################################################
### Options                                         ###
#######################################################
# What to build
option(BUILD_EXAMPLES "Build examples" ${MASTER_PROJECT})
option(BUILD_TESTS "Build tests" ${MASTER_PROJECT})
option(BUILD_DOCS "Build documentation" ${MASTER_PROJECT})
option(BUILD_INSTALLER "Build installer target" ${MASTER_PROJECT})
option(BUILD_PACKAGE "Build package" ${MASTER_PROJECT})
option(BUILD_CONAN "Build with conan" ${MASTER_PROJECT})

# How to build
option(BUILD_SHARED_LIBS "Build shared libraries" OFF)
option(BUILD_WITH_PEDANTIC_WARNINGS "Use pedantic warnings. This is useful for developers because many of these warnings will be in continuous integration anyway." ${DEBUG_MODE})
option(BUILD_WITH_UTF8 "Accept utf-8 in MSVC by default." ON)

#######################################################
### Additional flags                                ###
#######################################################
find_program(GCOV gcov)
find_program(LCOV lcov)
find_program(GENHTML genhtml)

# Using CPM-CMake or conan
if(BUILD_CONAN)
        include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
        conan_basic_setup()
else()
        option(CPM_USE_LOCAL_PACKAGES "Try `find_package` before downloading dependencies" ON)
        include(cmake/CPM.cmake)
endif()

if(GCOV AND LCOV AND GENHTML)
        set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS} -fprofile-arcs -ftest-coverage -O0")
endif()


#######################################################
### Libraries                                       ###
#######################################################
add_subdirectory(ltr)


#######################################################
### Examples and tests                              ###
#######################################################
if (BUILD_EXAMPLES)
        add_subdirectory(examples)
endif ()

if (BUILD_TESTS)

        if(NOT GCOV)
                message("gcov not found!")
        endif()
        if(NOT LCOV)
                message("lcov not found!")
        endif()
        if(NOT GENHTML)
                message("genhtml not found!")
        endif()

        if (GCOV AND LCOV AND GENHTML)

                add_custom_target(gcov
                        COMMAND mkdir -p Coverage
                        DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/Testing
                        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
                        )

                add_custom_command(TARGET gcov POST_BUILD
                        COMMAND find ${CMAKE_BINARY_DIR} -type f -name *.cpp.o -exec ${GCOV} {} "\;"
                        COMMAND echo "Coverage reports generated."
                        COMMAND echo "You can find them on ${CMAKE_BINARY_DIR}/Coverage"
                        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/Coverage
                        COMMENT "Coverage Reports Generated"
                        )

                add_custom_target(lcov
                        COMMAND mkdir -p Coverage
                        DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/Testing
                        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
                        )

                add_custom_command(TARGET lcov POST_BUILD
                        COMMAND ${LCOV} --directory .. --capture --output-file lcov.info
                        COMMAND ${GENHTML} -o . lcov.info
                        COMMAND echo "Coverage reports generated."
                        COMMAND echo "You can find them on ${CMAKE_BINARY_DIR}/Coverage"
                        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/Coverage
                        COMMENT "Coverage Reports Generated"
                        )

                set_property(DIRECTORY APPEND PROPERTY ADDITIONAL_MAKE_CLEAN_FILES Coverage)
        endif()

                
        enable_testing()
        add_subdirectory(tests)
        
endif ()

if (BUILD_DOCS)
        add_subdirectory(docs)
endif ()

#######################################################
### Installer                                       ###
#######################################################
if (BUILD_INSTALLER)
        set(CONFIG_INSTALL_DIR ${CMAKE_INSTALL_LIBDIR}/cmake/ltr)
        message("CMAKE_INSTALL_PREFIX=${CMAKE_INSTALL_PREFIX}")
        message("CMAKE_INSTALL_LIBDIR=${CMAKE_INSTALL_LIBDIR}")

        write_basic_package_version_file(
                ltrConfigVersion.cmake
                VERSION ${PACKAGE_VERSION}
                COMPATIBILITY AnyNewerVersion
        )

        # create ltr-config.cmake from ltr-config.cmake.in
        configure_package_config_file (
                ${CMAKE_CURRENT_SOURCE_DIR}/ltr-config.cmake.in
                ${CMAKE_CURRENT_BINARY_DIR}/ltr-config.cmake
                INSTALL_DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/ltr
        )

        # install the file ltr-config.cmake
        install(
                FILES ${CMAKE_CURRENT_BINARY_DIR}/ltr-config.cmake
                DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/ltr
        )

        if (NOT BUILD_SHARED_LIBS)
                install(
                        FILES ${CMAKE_CURRENT_SOURCE_DIR}/cmake/FindFilesystem.cmake
                        DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/ltr
                )
        endif()

endif()


#######################################################
### Packages                                        ###
#######################################################

if (BUILD_INSTALLER AND BUILD_PACKAGE)
        set(CPACK_PACKAGE_VENDOR "ltrpp")
        set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "LTR++: A C++ Learning to Rank library based on Ranklib")
        set(CPACK_PACKAGE_VERSION_MAJOR ${PROJECT_VERSION_MAJOR})
        set(CPACK_PACKAGE_VERSION_MINOR ${PROJECT_VERSION_MINOR})
        set(CPACK_PACKAGE_VERSION_PATCH ${PROJECT_VERSION_PATCH})
        set(CPACK_RESOURCE_FILE_LICENSE "${LTRPP_ROOT_DIR}/LICENSE")
        set(CPACK_RESOURCE_FILE_README "${LTRPP_ROOT_DIR}/README.md")
        set(CPACK_PACKAGE_RELEASE 1)

        FILE(READ .gitignore GITIGNORE_CONTENTS)
        STRING(REGEX REPLACE ";" "\\\\;" GITIGNORE_CONTENTS "${GITIGNORE_CONTENTS}")
        STRING(REGEX REPLACE "\n" ";" GITIGNORE_CONTENTS "${GITIGNORE_CONTENTS}")
        set(CPACK_SOURCE_IGNORE_FILES ${GITIGNORE_CONTENTS})

        include(CPack)
endif()