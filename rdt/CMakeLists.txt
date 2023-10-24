cmake_minimum_required(VERSION 3.20)
project(rdt)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_C_COMPILER clang)
set(CMAKE_CXX_COMPILER clang++)

include_directories(${PROJECT_SOURCE_DIR}/include)
FIND_LIBRARY(NETSIM_LIB libnetsim.a ${PROJECT_SOURCE_DIR}/lib)

add_executable(gbn
  src/main.cpp
  src/GBNRdtReceiver.cpp
  src/GBNRdtSender.cpp
  include/DataStructure.h
  include/Global.h
  include/NetworkService.h
  include/RandomEventEnum.h
  include/RdtReceiver.h
  include/RdtSender.h
  include/Tool.h
  include/GBNRdtReceiver.h
  include/GBNRdtSender.h
)
target_link_libraries(gbn ${NETSIM_LIB})
target_compile_definitions(gbn PUBLIC GBN WINDOW_LENGTH=16)
# target_compile_options(gbn PUBLIC -std=c++17 -Wall -Wshadow -fsanitize=address -fsanitize=undefined)
# target_link_options(gbn PUBLIC -fsanitize=address -fsanitize=undefined)

add_executable(sr
  src/main.cpp
  src/SRRdtReceiver.cpp
  src/SRRdtSender.cpp
  include/DataStructure.h
  include/Global.h
  include/NetworkService.h
  include/RandomEventEnum.h
  include/RdtReceiver.h
  include/RdtSender.h
  include/Tool.h
  include/SRRdtReceiver.h
  include/SRRdtSender.h
)
target_link_libraries(sr ${NETSIM_LIB})
target_compile_definitions(sr PUBLIC SR WINDOW_LENGTH=16)
# target_compile_options(sr PUBLIC -std=c++17 -Wall -Wshadow -fsanitize=address -fsanitize=undefined)
# target_link_options(sr PUBLIC -fsanitize=address -fsanitize=undefined)

add_executable(tcp
  src/main.cpp
  src/TCPRdtReceiver.cpp
  src/TCPRdtSender.cpp
  include/DataStructure.h
  include/Global.h
  include/NetworkService.h
  include/RandomEventEnum.h
  include/RdtReceiver.h
  include/RdtSender.h
  include/Tool.h
  include/TCPRdtReceiver.h
  include/TCPRdtSender.h
)
target_link_libraries(tcp ${NETSIM_LIB})
target_compile_definitions(tcp PUBLIC TCP WINDOW_LENGTH=16)
# target_compile_options(tcp PUBLIC -std=c++17 -Wall -Wshadow -fsanitize=address -fsanitize=undefined)
# target_link_options(tcp PUBLIC -fsanitize=address -fsanitize=undefined)