CXX		  := g++
CXX_FLAGS := -std=c++17 -ggdb #-Wall -Wextra  
O_FLAGS	  := -Ofast -flto

SRC	:= src

LIBRARIES	:=
EXECUTABLE	:= main


all: $(EXECUTABLE)

run: clean all
	./$(EXECUTABLE)

$(EXECUTABLE): $(SRC)/*.cpp
	$(CXX) $(O_FLAGS) -I$(INCLUDE) -L$(LIB) $^ -o $@ $(LIBRARIES)

clean:
	-rm $(EXECUTABLE)

debug: $(SRC)/*.cpp
	-rm debug
	$(CXX) $(CXX_FLAGS) -I$(INCLUDE) -L$(LIB) $^ -o $@ $(LIBRARIES)
