CXX		  := g++
CXX_FLAGS := -std=c++17 -Ofast #-ggdb #-Wall -Wextra  

SRC		:= src

LIBRARIES	:=
EXECUTABLE	:= main


all: $(EXECUTABLE)

run: clean all
	./$(EXECUTABLE)

$(EXECUTABLE): $(SRC)/*.cpp
	$(CXX) $(CXX_FLAGS) -I$(INCLUDE) -L$(LIB) $^ -o $@ $(LIBRARIES)

clean:
	-rm $(EXECUTABLE)
