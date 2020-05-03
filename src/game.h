/*******************************************************************
** This code is part of Breakout.
**
** Breakout is free software: you can redistribute it and/or modify
** it under the terms of the CC BY 4.0 license as published by
** Creative Commons, either version 4 of the License, or (at your
** option) any later version.
******************************************************************/
#ifndef GAME_H
#define GAME_H


#include <vector>
#include <iostream>

#include <glad/glad.h>
#include <GLFW/glfw3.h>

#include "resources/model.h"
#include "player.h"



#define NUM_KEYS 1024
// Game holds all game-related state and functionality.
// Combines all game-related data into a single class for
// easy access to each of the components and manageability.
class Game
{
private:
  // construction time variables
  // ---------------------------
  const unsigned int width, height;

  // init time variables
  // -------------------

  // runtime mutating variables
  // --------------------------
  bool pressedKeys[NUM_KEYS];


  // constructor/destructor
public:
  Player player;
  Game(unsigned int _width, unsigned int _height);
  ~Game();
  // initialize game state (load all shaders/textures/levels)
  void init();

  // game loop
  void processInput(float dt);
  void update(float dt);
  void render() const;

  // Input callbacks
  void mouseMoved(const double dx, const double dy);
  void mouseScrolled(const double offset);
  void keyPressed(const int key);
  void keyReleased(const int key);
};

#endif