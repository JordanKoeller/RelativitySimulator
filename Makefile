# CC specifies which compiler we're using

# COMPILER_FLAGS specifies the additional compilation options we're using
# -w suppresses all warnings
LINKER_FLAGS = -lglfw3 -lGL -lX11 -lpthread -lXrandr -lXi -ldl -lassimp -lfreetype -I include

CXX = g++
CXXFLAGS = -Wall -std=c++17 --pedantic

# LINKER_FLAGS specifies the libraries we're linking against

# This is the target that compiles our executable
main : bin/main.o bin/glad.o
	$(CXX) $(CXXFLAGS) bin/glad.o bin/game.o bin/text_overlay.o bin/shader.o bin/resource_manager.o bin/main.o bin/kinematics_engine.o -o relsim $(LINKER_FLAGS)

bin/glad.o : src/glad.c
	$(CXX) $(CXXFLAGS) -c -o bin/glad.o src/glad.c $(LINKER_FLAGS)

bin/main.o : src/main.cpp bin/game.o bin/text_overlay.o 
	$(CXX) $(CXXFLAGS) -c -o bin/main.o  src/main.cpp $(LINKER_FLAGS)

bin/game.o : src/game.cpp bin/resource_manager.o src/resources/cubemap.h bin/kinematics_engine.o src/particle.h src/player.h
	$(CXX) $(CXXFLAGS) -c  -o bin/game.o src/game.cpp $(LINKER_FLAGS)

bin/text_overlay.o : src/utils/text_overlay.cpp
	$(CXX) $(CXXFLAGS) -c -o bin/text_overlay.o src/utils/text_overlay.cpp $(LINKER_FLAGS)

bin/shader.o : src/resources/shader.cpp
	$(CXX) $(CXXFLAGS) -c -o bin/shader.o src/resources/shader.cpp $(LINKER_FLAGS)

bin/resource_manager.o : src/resources/resource_manager.cpp bin/shader.o src/resources/mesh.h src/resources/model.h
	$(CXX) $(CXXFLAGS) -c -o bin/resource_manager.o src/resources/resource_manager.cpp $(LINKER_FLAGS)

bin/kinematics_engine.o : src/kinematics_engine.cpp
	$(CXX) $(CXXFLAGS) -c -o bin/kinematics_engine.o src/kinematics_engine.cpp $(LINKER_FLAGS)

clean :
	rm -rf bin
	mkdir bin
