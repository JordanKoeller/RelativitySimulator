
#include <iostream>
#include <vector>
#include <string>

#include "game.h"
#include "resources/model.h"
#include "resources/resource_manager.h"
#include "resources/cubemap.h"
#include "kinematics_engine.h"
#include "player.h"

#define BRAKING_FORCE 10.0f

using std::cout;
using std::endl;
using std::vector;

CubeMap skybox;
PhysicsEngine engine;

Game::Game(unsigned int _width, unsigned int _height)
    : width(_width), height(_height) {}

Game::~Game() {}

void Game::init()
{
  auto shader = ResourceManager::loadShader("shaders/models.vs", "shaders/models.fs", nullptr, "cityshader");
  ResourceManager::loadModel("resources/Camellia City/3DS/Camellia City.3ds", "city");
  ResourceManager::linkModelShader("city", "cityshader");
  player = Player(glm::vec3(81.0f, 66.0f, -1800.0f), 0.01f, 1.0f);
  skybox = CubeMap(std::vector<std::string>{
      "resources/Skybox/right.jpg",
      "resources/Skybox/left.jpg",
      "resources/Skybox/top.jpg",
      "resources/Skybox/bottom.jpg",
      "resources/Skybox/front.jpg",
      "resources/Skybox/back.jpg"});
}

void Game::update(float dt)
{
  engine.update(player, dt);
}

void Game::processInput(float dt)
{
  if (pressedKeys[GLFW_KEY_LEFT_SHIFT]) // hit the brakes!
  {
    //vf = vi + at
    // 0.999 = vi + at
    // -vi = at
    // a = -vi/dt
    if (glm::length(player.velocity) > 1.0f)
    {
      float velMag = glm::length(player.velocity);
      vec3 velDir = glm::normalize(player.velocity);
      float stoppingMag = velMag / dt;
      float brakingForce = BRAKING_FORCE * velMag;
      stoppingMag > brakingForce ? player.applyForce(-brakingForce * velDir)
                                 : player.applyForce(-stoppingMag * velDir);
    }
  }
  glm::vec3 ret(0.0f, 0.0f, 0.0f);
  if (pressedKeys[GLFW_KEY_W])
  {
    ret += glm::normalize(player.front());
  }
  if (pressedKeys[GLFW_KEY_S])
  {
    ret -= glm::normalize(player.front());
  }
  if (pressedKeys[GLFW_KEY_D])
  {
    ret += glm::normalize(player.right());
  }
  if (pressedKeys[GLFW_KEY_A])
  {
    ret -= glm::normalize(player.right());
  }
  if (ret == glm::vec3(0.0f, 0.0f, 0.0f))
    return; // No buttons pressed so no work to do.
  ret = player.commandMagnitude() * glm::normalize(ret);
  player.applyForce(glm::vec3(ret.x, 0.0f, ret.z)); // Lock y value.
}

void Game::render() const
{
  glm::mat4 perspective = glm::perspective(glm::radians(player.getZoom()), (float)width / (float)height, 0.1f, 100000.0f);
  glm::mat4 viewMatrix = player.getViewMatrix();
  vec3 pos = player.getPosition();
  auto shader = ResourceManager::getShader("cityshader");
  shader.activate();
  shader.setVector3f("viewPos", pos);
  shader.setVector3f("directionlight.direction", vec3(0.2f, -1.0f, 0.2f));
  shader.setVector3f("directionlight.ambient", vec3(0.4f, 0.4f, 0.4f));
  shader.setVector3f("directionlight.diffuse", vec3(0.6f, 0.6f, 0.6f));
  shader.setVector3f("directionlight.specular", vec3(1.0f, 1.0f, 1.0f));
  ResourceManager::getModel("city").draw(viewMatrix, perspective);
  skybox.draw(viewMatrix, perspective);
}

void Game::mouseMoved(const double dx, const double dy)
{
  // cout << "MOUSE MOVED " << dx << ", " << dy << endl;
  player.rotate(dx * player.sensitivity(), dy * player.sensitivity());
}
void Game::mouseScrolled(const double yoffset)
{
  // cout << "scroll callback" << endl;
}

void Game::keyPressed(const int key)
{
  pressedKeys[key] = true;
}

void Game::keyReleased(const int key)
{
  pressedKeys[key] = false;
}