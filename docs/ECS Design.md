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


# Asset Management

I started working on the code for the system abstractions, but I have an issue. I need a better way to manage assets.

## Asset Types:
+ Assets loaded from file. These files don't necessarily _need_ loaded into the GPU, but loading them from disk is slow and I should avoid doing it more than necessary.
  + Texture files
  + 3D files (.obj files)
  + Media (audio, video) - not supported right now but in the future.
  + Shaders
+ Assets that need to go to the GPU and be managed by the GPU accordingly.
  + VBO/VAO (programatically made, or from a model file).
  + Textures - need loaded into GPU to render
  + Shaders - extra complexity of pre-compiled versus source code.

For both of these aseet types their management is expensive so I don't want to do it more than necessary. I want to step them through their lifecycle _once_ with no redundancy.

I can accomplish this by handling AssetIDs instead of the assets themselves.

## Asset IDs

A few options:
+ Option 1: Human-readable Asset Ids
  + Pros:
    + Makes it easier to debug
    + Makes it easier to find common assets and prevent duplicate work
  + Cons:
    + Requires a hashmap for lookup to actual assets
    + No guaranteed uniqueness - puts the onus on the developer. That being said, I could easily make helper functions that generates a UUID if uniqueness is mandated.
+ Option 2: UUID
  + Pros:
    + Guaranteed unique always
    + Neatly handles removal of assets from the registry
  + Cons
    + Mapping to actual assets involves a HashMap or Binary search
+ Option 2: Ascending Numeric IDs
  + Pros:
    + Fast lookups - indices into a Vector.
  + Cons:
    + Issue of synchronizing a counter across threads for ID Generation
    + Monotonically increasing IDs means monotonically expanding vector for lookups.
    + Ids become inefficient if I need to remove assets from the registry.
+ Option 3: Use std::Arcs
  + Pros:
    + Built in to the system
    + Fastest lookup times without need for mapping
    + None of the drawbacks of indices either
  + Cons:
    + Locking on reference count changes
    + Underlying data becomes immutable.
+ Option 4: Use std::Arc<std::Mutext>
  + Pros:
    + Built in to the system
    + Fastest lookup times without need for mapping
    + None of the drawbacks of indices
  + Cons:
    + Locking on reference count changes
    + Locking on data mutation which is superfluous since I plan on mutation being single-threaded
+ Option 5: Write my own specialty reference counter that has the locking semantics I need
  + Pros:
    + Best of all worlds
  + Cons:
    + Added developer complexity
    + Will be reliant on unsafe code. But maybe that's OK.

I think Option 1 is favorable. There is a tradeoff in ID lookups. But if needed, I could implement some caching type system for accelerating lookups. DONT PRE OPTIMIZE

## Implementing Asset Management

There is some complexity in how to do this smoothly because:

1. Model assets are compound assets:
   1. Multiple Meshes/3D objects.
   2. Multiple textures
2. Materials are compound assets:
   1. Ambient/Diffuse textures
   2. A Uniform Buffer Object

I don't want to grab more than is absolutely necessary. So I should break assets down to their smalest parts when they are compound. While this is good from an efficiency standpoint it sucks from a developer standpoint.

Adding more complexity, the components that I want are the ultimate destination of where all these asset IDs will be handled in the system and they are potentially compound as well!

### Components that handle AssetIDs:
+ `Material` -> has multiple TextureIDs in it.
+ `Mesh` -> has ONE Mesh/3D object.
+ `Shader` -> Refers to ONE shader resource
+ `Buffer` (FUTURE) -> Refers to a GPU-backed array buffer.
+ `CompositeAsset` (FUTURE) -> A general bucket for components with multiple ResourceIDs enclosed.

While it would be tempting to make an API that returns components directly, I don't think that is maybe the wisets implementation for this part of the `SystemUtilities` API.  I definitely see advantages to methods like `load_model`, `load_material`, etc. but those APIs are higher-level and should exist on top of the bare bones AssetManagement interface.

### Interfaces - Base types

```
type K = Eq + PartialEq + Hash + Ord + PartialOrd + Sized; // I include Ord + PartialOrd to support changing to a B-Tree in the future, if needed.

trait Buildable<T> {
    fn build(self) -> V;
}

trait V {
    type Builder: Buildable<V>;
}

// Registry should have really fast inserts. Fetches may be slower.

// When it comes time to run  through draw calls, I can make it s.t. my K for the VAO registry contains the VAO ID inside of it so no access to the registry is needed.

trait Registry<K, V> {
    get_registry_id(lookup_name: &str) -> Option<K>; 
    
    fetch(registry_id: K) -> &V;

    fetch_mut(registry_id: K) -> &mut V;

    // Schedules for building and insertion later, but gives back an ID now.
    // If the same lookup_name is enqueued twice before the lazy queue has been drained
    // This must still give back a key and skip enqueuing twice.
    enqueue_builder(lookup_name: &str, builder: V::Builder) -> K; 
}
```