# ECS Abstractions

Specs provides the bare bones to meet the requirements of an ECS, but it's extremely barebones.

I want to provide some more sophisticated tools and utilities that will make it easier to do common game dev things.

# Supported Features

## Game Lifecycle things

+ Game Initialization Logic.
+ Game "running" Logic.
+ Game teardown logic.
+ Frame-stepping logic for debug purposes.
+ Saving/Loading game resources, as well as user progress.

## Dev Tools

+ A panel with performance measures
  + FPS counter (avg and instantaneous)
  + Camera coordinates, facing, velocity
  + Number of draw calls
  + Consumed RAM
  + CPU Utilization
  + Consumed GPU RAM
  + Render Time (Avg and Instantaneous)
  + Frame Time (Avg and Instantaneous)
+ Posibility to step through frames
+ Widgets for fiddling parameters on entities
+ Widget for fiddling with shader parameters
+ Logging of any error messages from resource loading, OpenGL, Shader engine, etc. to both the terminal and a log file.

Far down the road I will add the ability to spawn entities, move them around, set properties, etc. and build a scene visually

## Handy Utilities/abstractions

+ Prefab support (prebuilt bundles of components for a particular entity)
+ Event passing system with a Pub/Sub API
+ Command pattern abstraction on top of window events
+ Spawning/destroying prefabs easily
+ Delta-based render queue (where it is a NOOP to draw something between two successive frames from the CPU side)
+ Entity-based materials
+ Environment-based shader parameters
+ GUI tools for creating interractive HUD.
+ Ability to swap out scenes.
+ Compute Shaders in the scene setup phase (maybe other times as well? I need to research compute shaders more).

## Premade systems

+ Kinematics/physics engine
+ Collision handling
+ Simple pub/sub based networking abstraction
+ Particle system
+ Fast rendering system that's GPU-accelerated (can use a LRU-type system to create/destroy particles really fast)
+ Audio system

## Rendering Functionality

+ Drawing objects as a three-tuple of (Mesh, Material, ShaderId)
+ Instanced Rendering
+ Basic lighting effects
+ Ambient occlusion
+ Bloom
+ Shadows
+ Deferred rendering (will require research)

# `System` Abstraction

## API Descriptions
+ Prefab creation with an Entity returned
+ Prefab destruction with automatic GPU cleanup
+ Logger Functionality
+ Gui Panel creation
+ Getting data out of GUI panel
+ Submitting GPU commands
+ System Runtime performance reporting

## Implementation Details
+ I want to give the game dev as much possibility to control parallelism as possible. So under the hood I will use `EventChannel`s for system APIs and interior mutability.
+ I want to make an API that hides these details. I'm going to make a `trait` that provies some methods like:
  +  `create_prefab<P: PrefabBuilder>(&self, prefab_state: P) -> Entity`
  +  `destroy_prefab(&self, prefab_id: Entity)`
  +  `log(&self, message: &str, log_level: LogLevel)`
  +  `init_panel<S: PanelState>(&self, panel: GuiPanel<S>)`
  +  `refresh_panel(&self) -> Box<dyn S>`