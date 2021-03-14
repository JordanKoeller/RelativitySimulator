# Relativistic game engine.

This is a game engine that has special shaders for simulating relativistic motion. It's also an opportunity for me to learn about OpenGL and Rust.

A lot of the "starter code" comes from bwasty's repo about converting Joey DeVries OpenGL tutorials to Rust. https://github.com/bwasty/learn-opengl-rs


## TODO LIST:

  * Collisions
  * Add Input widgets for menus and debugging
  * Blinn Phong Lighting
  * Shadows
  * Better Tessellation control
  * Scene selection/swapping in a menu.
  * A city model scene


## Goal:

I need to have a concrete goal for where I'm going so that I can accomplish something

So what am I trying to do?

I want to make a procedurally generated 9-block city

The first iteration will be:

### Requirements:
  Buildings are textured rectangles:
    One rectangle on top of another.
    Each rectangle is uniquely textured.
    Parameters:
      1. height
      2. blocks ratio
      3. A button to re-choose textures
  
  There are two different "street" models.
    A straightaway piece (rectangle with middle street line)
    An intersection piece (A square with crosswalk walls)
    Parameters:
      1. width
      2. length

  "City" generation:
    The street and building pieces will be laid out in a grid. They will snap together like lego bricks
    The building will be a NxN piece and street pieces are 1x1. So you can use streets to border a building.
    A "City" is initialized with a 2D array of numbers, where a "#" is a building and a "@" is a street. If there are multiple adjacent building pieces, they get merged into one actual building model
    Ex:
    @@@@@@@@@
    ##@#@###@
    ##@@@###@
    @@@#@###@
    @#@#@@@@@
    @@@@@##@#
    @#@#@##@@
    @@@@@@@@#
    @######@#
  

  
