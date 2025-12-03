  cmake_minimum_required(VERSION 3.25)

  project(day03 VERSION 0.0.1)

  set(CMAKE_CXX_STANDARD 23)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
  set(CMAKE_CXX_EXTENSIONS OFF)
  set(CMAKE_EXPORT_COMPILE_COMMANDS on)

  file(GLOB_RECURSE SOURCES CONFIGURE_DEPENDS
      "src/*.cpp"
      "src/*.hpp"
  )

  add_executable(day03 ${SOURCES})

  set(COMMON_WARNINGS
      -Wall
      -Wextra
      -Wpedantic
      -Wshadow
  )

  if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
      target_compile_options(day03 PRIVATE
          ${COMMON_WARNINGS}
          -Werror
      )
  elseif(CMAKE_CXX_COMPILER_ID MATCHES "MSVC")
      target_compile_options(day03 PRIVATE
          /W4
          /WX
      )
  endif()
