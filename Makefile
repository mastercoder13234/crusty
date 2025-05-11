RUST_SRC = rsa/lib.rs
RUST_LIB_NAME_PREFIX = lib
RUST_LIB_EXTENSION = so
ifeq ($(OS),Windows_NT)
    RUST_LIB_EXTENSION = dll
    RUST_LIB_NAME_PREFIX =
endif
ifeq ($(shell uname -s),Darwin)
    RUST_LIB_EXTENSION = dylib
endif
RUST_LIB_NAME = $(RUST_LIB_NAME_PREFIX)rsa.$(RUST_LIB_EXTENSION)
RUST_BUILD_DIR = . # Build Rust lib in the base dir

CPP_SOURCES = cpp/main.cpp cpp/bindings.cpp
CPP_EXECUTABLE = crusty
CXX = g++
CXXFLAGS = -std=c++20 -Wall -Wextra -fPIC

all: $(CPP_EXECUTABLE)

$(RUST_LIB_NAME): $(RUST_SRC)
	rustc --crate-type cdylib -o $@ $<

$(CPP_EXECUTABLE): $(CPP_SOURCES) $(RUST_LIB_NAME)
	$(CXX) $(CXXFLAGS) $^ -o $@ -L. -lrsa

clean:
	rm -f $(RUST_LIB_NAME) $(CPP_EXECUTABLE) *.o cpp/*.o

run: $(CPP_EXECUTABLE)
	./$(CPP_EXECUTABLE)

# --- Helper functions for C++ ---
cpp/%.o: cpp/%.cpp cpp/bindings.h
	$(CXX) $(CXXFLAGS) -c $< -o $@

bindings.o: cpp/bindings.cpp cpp/bindings.h
	$(CXX) $(CXXFLAGS) -c $< -o $@

main.o: cpp/main.cpp cpp/bindings.h
	$(CXX) $(CXXFLAGS) -c $< -o $@