# Project 2: Making minecraft


## Goals:

 1. Have a block-based procedural world.
 2. Be able to place and destroy blocks.
 3. Have 3 or 4 block types that can be placed and are placed by default.
 4. Day/Night cycle and shadow casting

## Design

 1. Chunk components - 16x16x256 columns of the world.
    1. Chunks are 3D arrays of Blocks/Non-blocks
    2. They have a VAO where they click together external blocks to form single drawables for an entire chunk.
    3. Textures are combined into sprite-sheets? (Would be a 128 x 2048 sheet per side)
 2. If needed, chunks are optimized in a separate thread by a ChunkLoader api.

## Development Plan

 1. Render a floor I can walk on as 16x16x1 individual blocks
 2. Write an api for taking a chunk and producing a minimized VAO 
 3. Add a Debuggable ChunkManager system that generates chunks with controllable Perling noise parameters.