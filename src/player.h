#ifndef PLAYER_H
#define PLAYER_H

#include "./particle.h"
#include "./utils/camera.h"


struct PlayerPreferences {
  float sensitivity;
  float commandMagnitude;
};

class Player: public Particle {
private:
  Camera camera;
  PlayerPreferences preferences{0.1f, 500.0f};


public:
  Player() = default;

  Player(vec3 _pos, float dragFactor, float _mass): Particle(_pos, dragFactor, _mass)
  {
    camera = Camera(_pos);
  }

  vec3 front() const {
    return camera.front;
  }

  vec3 right() const {
    return camera.right;
  }

  float commandMagnitude() const {
    return preferences.commandMagnitude;
  }

  float sensitivity() const {
    return preferences.sensitivity;
  }

  glm::mat4 getViewMatrix() const {
    return camera.getViewMatrix();
  }

  float getZoom() const {
    return camera.zoom;
  }

  void rotate(float dx, float dy) {
    camera.rotate(dx, dy);
  }

  void setPosition(vec3 p)
  {
    Particle::setPosition(p);
    camera.position = p;
  }

};

#endif