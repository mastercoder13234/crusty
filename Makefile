UNAME_S := $(shell uname -s)

ifeq ($(OS),Windows_NT)
    export BIN_NAME := crusty.exe
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
        export BIN_NAME := crusty
    else ifeq ($(UNAME_S),Darwin)
        export BIN_NAME := crusty
    else
        $(error Unsupported OS: $(UNAME_S))
    endif
endif


all: rsa/librsa.so 
	tup

rsa/librsa.so: rsa/*.rs
	rustc --crate-type cdylib -o rsa/librsa.so -C incremental=rsa/build rsa/lib.rs

run: $(BIN_NAME)
	./$(BIN_NAME)