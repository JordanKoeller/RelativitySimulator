#ifndef PARTICLE_H
#define PARTICLE_H

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include "config.h"

using glm::vec3;

class Particle
{
private:
  glm::vec3 position;

public:
  glm::vec3 velocity, force;
  float drag, mass;
  Particle() = default;
  Particle(vec3 _pos, float dragFactor, float _mass)
  {
    position = _pos;
    velocity = vec3(0.0f, 0.0f, 0.0f);
    force = vec3(0.0f, 0.0f, 0.0f);
    drag = dragFactor;
    mass = _mass;
  }

  void applyForce(vec3 f)
  {
    force += f;
  }

  void clearForces()
  {
    force = vec3(0.0f, 0.0f, 0.f);
  }

  vec3 getPosition() const
  {
    return position;
  }

  virtual void setPosition(vec3 p)
  {
    position = p;
  }

  float beta() {
    return glm::length(velocity) / LIGHT_SPEED;
  }
};
#endif