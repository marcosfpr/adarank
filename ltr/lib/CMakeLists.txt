#######################################################
### Source                                          ###
#######################################################

set(
    SOURCES

    impl/learning/DataPoint.cpp
    impl/learning/RankList.cpp
    impl/learning/DataSet.cpp
    impl/learning/Ranker.cpp
    impl/learning/Learner.cpp
    impl/learning/ensemble/AdaRank.cpp

    impl/metric/MetricScorer.cpp
    impl/metric/MAPScorer.cpp
    impl/metric/PrecisionScorer.cpp
    impl/utils/KeyValue.cpp
    impl/utils/Logging.cpp

)

add_library(
    ltr 
    ${SOURCES}
)

if (NOT BUILD_CONAN)
    CPMFindPackage(
            NAME boost-cmake
            GITHUB_REPOSITORY marcosfpr/boost-cmake
            VERSION 1.71.0
            FIND_PACKAGE_ARGUMENTS ""
    )

    CPMFindPackage(
            NAME spdlog
            GITHUB_REPOSITORY gabime/spdlog
            VERSION 1.8.2
            FIND_PACKAGE_ARGUMENTS ""
    )
endif()

add_definitions(-DBOOST_ALL_DYN_LINK)

find_package(Threads REQUIRED)


target_include_directories(
    ltr
    PUBLIC api
)

if (BUILD_CONAN)
    target_link_libraries(ltr PRIVATE ${CONAN_LIBS} Threads::Threads)
else()
    target_link_libraries(ltr PRIVATE Boost::boost spdlog ${FILE_SYSTEM_LIB} Threads::Threads)
endif()

target_compile_features(ltr PUBLIC cxx_std_17)

include(CheckSymbolExists)

# Some hack to MSVC and boost
if(MSVC)
    add_definitions("/EHsc")
endif() 

# Some hack to not depend on FILE* internals
# https://github.com/alandefreitas/matplotplusplus/issues/4
check_symbol_exists(__fbufsize "stdio_ext.h" HAVE_FBUFSIZE)
if (HAVE_FBUFSIZE)
    target_compile_definitions(ltr PRIVATE LTR_HAS_FBUFSIZE)
endif()

# Another hack to check for min in Windows.h
# http://www.suodenjoki.dk/us/archive/2010/min-max.htm
check_symbol_exists(min "Windows.h" HAVE_WINDOWS_MINMAX)
if (HAVE_WINDOWS_MINMAX)
    target_compile_definitions(ltr PUBLIC NOMINMAX)
endif()

if (BUILD_WITH_PEDANTIC_WARNINGS)
    if (MSVC)
        target_compile_options(ltr PRIVATE /W4 /WX)
    else ()
        target_compile_options(ltr PRIVATE -Wall -Wextra -pedantic -Werror)
        # Allow the warnings related to the bundled CImg
        if (CMAKE_CXX_COMPILER_ID STREQUAL "Clang" OR CMAKE_CXX_COMPILER_ID STREQUAL "AppleClang")
            target_compile_options(ltr PRIVATE -Wno-null-pointer-arithmetic -Wno-char-subscripts)
        elseif (CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
            target_compile_options(ltr PRIVATE -Wno-error=class-memaccess -Wno-class-memaccess)
        else ()
            message(ERROR "Cannot disable the relevant warnings for ${CMAKE_CXX_COMPILER_ID}")
        endif ()
    endif ()
endif ()

# Setting public header
 set_target_properties(ltr PROPERTIES PUBLIC_HEADER "${CMAKE_CURRENT_SOURCE_DIR}/api/ltr.hpp")

# Install
if (BUILD_INSTALLER)
    # Install targets
    install(TARGETS ltr
            EXPORT ltrTargets
            LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
            ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
            PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_LIBDIR}
            )

    # Install headers
    install(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
            DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
            FILES_MATCHING PATTERN "*.hpp"
            )
endif()