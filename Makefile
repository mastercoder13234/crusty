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
BINDINGS_HEADER = cpp/bindings.h

CPP_SOURCES = cpp/main.cpp cpp/libload.cpp
CPP_OBJECTS = $(CPP_SOURCES:.cpp=.o)
CPP_EXECUTABLE = crusty
CXX = g++
CXXFLAGS = -Wall -Wextra -fPIC

all: $(CPP_EXECUTABLE)

$(BINDINGS_HEADER): $(RUST_SRC)
	cbindgen --output $(BINDINGS_HEADER) --cpp-compat $(RUST_SRC)

$(RUST_LIB_NAME): $(RUST_SRC)
	rustc --crate-type cdylib -o $@ $<

$(CPP_EXECUTABLE): $(CPP_OBJECTS) $(RUST_LIB_NAME) $(BINDINGS_HEADER)
	$(CXX) $(CPP_OBJECTS) -o $@ -L. -lrsa -ldl

%.o: %.cpp $(BINDINGS_HEADER)
	$(CXX) $(CXXFLAGS) -c $< -o $@

cpp/%.o: cpp/%.cpp cpp/libload.h $(BINDINGS_HEADER)
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -r cpp/*.o *.a *.dll $(BINDINGS_HEADER)

run: $(CPP_EXECUTABLE)
	./$(CPP_EXECUTABLE)