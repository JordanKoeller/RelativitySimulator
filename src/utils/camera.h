#ifndef CAMERA_H
#define CAMERA_H

#include <glad/glad.h>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>

#include <vector>
#include <iostream>

// Defines several possible options for camera movement. Used as abstraction to stay away from window-system specific input methods
enum Camera_Movement
{
  FORWARD,
  BACKWARD,
  LEFT,
  RIGHT
};

// Default camera values
const float YAW = -90.0f;
const float PITCH = 0.0f;
const float SPEED = 80.0f;
const float ZOOM = 80.0f; // Field of vision in degrees

// An abstract camera class that processes input and calculates the corresponding Euler Angles, Vectors and Matrices for use in OpenGL
class Camera
{
public:
  // Camera Attributes
  glm::vec3 position;
  glm::vec3 front;
  glm::vec3 up;
  glm::vec3 right;
  glm::vec3 worldUp;
  // Euler Angles
  float yaw;
  float pitch;
  // Camera options
  float zoom;

  // Constructor with vectors
  Camera(glm::vec3 _position = glm::vec3(0.0f, 0.0f, 0.0f),
         glm::vec3 _up = glm::vec3(0.0f, 1.0f, 0.0f),
         float _yaw = YAW,
         float _pitch = PITCH) : front(glm::vec3(0.0f, 0.0f, -1.0f)), zoom(ZOOM)
  {
    position = _position;
    worldUp = _up;
    yaw = _yaw;
    pitch = _pitch;
    updateCameraVectors();
  }
  // Constructor with scalar values

  // Returns the view matrix calculated using Euler Angles and the LookAt Matrix
  glm::mat4 getViewMatrix() const
  {
    return glm::lookAt(position, position + front, up);
  }

  // Processes input received from any keyboard-like input system. Accepts input parameter in the form of camera defined ENUM (to abstract it from windowing systems)
  void moveBy(const float dx, const float dy, const float dz)
  {
    position += glm::vec3(dx, dy, dz);
  }

  // Processes input received from a mouse input system. Expects the offset value in both the x and y direction.
  void rotate(float dx_theta, float dy_theta)
  {
    yaw += dx_theta;
    pitch += dy_theta;

    // Make sure that when pitch is out of bounds, screen doesn't get flipped
    if (pitch > 89.0f)
      pitch = 89.0f;
    if (pitch < -89.0f)
      pitch = -89.0f;

    // Update Front, Right and Up Vectors using the updated Euler angles
    updateCameraVectors();
  }

  // Processes input received from a mouse scroll-wheel event. Only requires input on the vertical wheel-axis
  void ProcessMouseScroll(float yoffset)
  {
    if (zoom >= 1.0f && zoom <= 45.0f)
      zoom -= yoffset;
    if (zoom <= 1.0f)
      zoom = 1.0f;
    if (zoom >= 45.0f)
      zoom = 45.0f;
  }

private:
  // Calculates the front vector from the Camera's (updated) Euler Angles
  void updateCameraVectors()
  {
    // Calculate the new Front vector
    glm::vec3 _front;
    _front.x = cos(glm::radians(yaw)) * cos(glm::radians(pitch));
    _front.y = sin(glm::radians(pitch));
    _front.z = sin(glm::radians(yaw)) * cos(glm::radians(pitch));
    front = glm::normalize(_front);
    // Also re-calculate the Right and Up vector
    right = glm::normalize(glm::cross(front, worldUp)); // Normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
    up = glm::normalize(glm::cross(right, front));
  }
};
#endif
