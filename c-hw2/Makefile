CC		:= gcc
C_FLAGS := -Wall -Wextra

INCL	:= include
BIN		:= bin
SRC		:= src

LIBRARIES	:=
LINKS		:= -lm

EXECUTABLE	:= main

all: $(BIN)/$(EXECUTABLE)

clean:
	-$(RM) $(BIN)/$(EXECUTABLE)

run: all
	./$(BIN)/$(EXECUTABLE)

$(BIN)/$(EXECUTABLE): $(SRC)/*
	$(CC) $(C_FLAGS) -I$(INCL)  $^ -o $@ $(LIBRARIES) $(LINKS)
