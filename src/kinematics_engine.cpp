
#include "./kinematics_engine.h"
#include "./config.h"

void PhysicsEngine::update(Particle &particle, float dt)
{
  applyFriction(particle);
  // applyGravity(particle);
  // applyCollisions(particle);
  // TODO: Apply collisions.
  //vf = vi + at
  // x = x0 + vit + 1/2at^2
  // I remember talking with Mark a long time ago that integrators can get better performance
  // if they add velocity before updating position. I might want to refresh that conversation
  // with him. For now I won't
  vec3 acceleration = particle.force / particle.mass;
  // swap next two lines based on convo with Mark
  particle.setPosition(particle.getPosition() + particle.velocity * dt + acceleration * dt * dt / 2.0f);
  particle.velocity += dt * acceleration;
  if (glm::length(particle.velocity) < 1.0f) {
    particle.velocity = vec3(0.0f, 0.0f, 0.0f);
  }
  particle.clearForces();
}

void PhysicsEngine::applyFriction(Particle &particle)
{
  // friction is in opposite direction to velocity
  // magnitude proportional to velocity squared.
  if (glm::length(particle.velocity) > 10.0f)
  {
    vec3 frictionDirection = -glm::normalize(particle.velocity);
    float frictionMagnitude = particle.drag * glm::dot(particle.velocity, particle.velocity);
    particle.applyForce(frictionMagnitude * frictionDirection);
  }
}

void PhysicsEngine::applyGravity(Particle &particle)
{
  particle.applyForce(GRAVITY);
}