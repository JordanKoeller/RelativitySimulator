#ifndef PHYSICS_ENGINE_H
#define PHYSICS_ENGINE_H
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>

#include "./particle.h"

using glm::vec3;

class PhysicsEngine
{
public:
  void update(Particle &particle, float dt);
  PhysicsEngine() = default;

private:
  void applyFriction(Particle &particle);

  void applyGravity(Particle &particle);

  void applyCollisions(Particle &particle); // world geometry is static global.
};
#endif