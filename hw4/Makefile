CC		:= gcc
C_FLAGS := -Wall -Wextra

BIN		:= bin
SRC		:= src
INCLUDE	:= include

LIBRARIES	:= -lm

EXECUTABLE	:= main

all: $(BIN)/$(EXECUTABLE)

clean:
	-$(RM) -rf $(BIN)/*

run: all
	clear
	./$(BIN)/$(EXECUTABLE)

debug: $(BIN)/debug-main

$(BIN)/debug-main: $(SRC)/*
	$(CC) $(C_FLAGS) -g -I$(INCLUDE) $^ -o $@ $(LIBRARIES)
	

$(BIN)/$(EXECUTABLE): $(SRC)/*
	$(CC) $(C_FLAGS) -I$(INCLUDE) $^ -o $@ $(LIBRARIES)
