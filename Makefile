RUST_ENTRY_FILE = rsa/lib.rs
RUST_SRC = rsa/*.rs
RUST_LIB_NAME_PREFIX = lib
RUST_LIB_EXTENSION = so
LD_EXTRA_FLAGS = -ldl
ifeq ($(OS),Windows_NT)
	RUST_LIB_EXTENSION = dll
	RUST_LIB_NAME_PREFIX =
	LD_EXTRA_FLAGS = 
endif
ifeq ($(shell uname -s),Darwin)
	RUST_LIB_EXTENSION = dylib
	LD_EXTRA_FLAGS = 
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

$(BINDINGS_HEADER): $(RUST_ENTRY_FILE)
	cbindgen --config rsa/cbindgen.toml --output $(BINDINGS_HEADER) --cpp-compat $(RUST_ENTRY_FILE)

$(RUST_LIB_NAME): $(RUST_ENTRY_FILE) $(RUST_SRC)
	rustc --crate-type cdylib -C incremental=./rsa/build -o $@ $<

$(CPP_EXECUTABLE): $(CPP_OBJECTS) $(RUST_LIB_NAME)
	$(CXX) $(CPP_OBJECTS) -o $@ -L. -lrsa $(LD_EXTRA_FLAGS) -Wl,-rpath,'$$ORIGIN'

%.o: %.cpp $(BINDINGS_HEADER) cpp/libload.h
	$(CXX) $(CXXFLAGS) -c $< -o $@

cpp/%.o: cpp/%.cpp cpp/libload.h $(BINDINGS_HEADER)
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -r cpp/*.o *.a *.dll $(BINDINGS_HEADER) crusty $(RUST_LIB_NAME) rsa/build

run: $(CPP_EXECUTABLE)
	./$(CPP_EXECUTABLE)