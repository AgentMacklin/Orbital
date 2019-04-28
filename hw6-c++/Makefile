CXX		  := g++
CXX_FLAGS := -std=c++17 -ggdb #-Wall -Wextra  
O_FLAGS	  := -std=c++17 -Ofast -flto# -Wall -Wextra

SRC	:= src

LIBRARIES	:=
EXECUTABLE	:= main


all: $(EXECUTABLE)

run: clean all
	./$(EXECUTABLE)

$(EXECUTABLE): $(SRC)/*.cc
	$(CXX) $(O_FLAGS) -I$(INCLUDE) $^ -o $@ $(LIBRARIES)

clean:
	-rm $(EXECUTABLE)

debug: $(SRC)/*.cc
	-rm debug
	$(CXX) $(CXX_FLAGS) -I$(INCLUDE) -L$(LIB) $^ -o $@ $(LIBRARIES)
