#ifndef MODEL_H
#define MODEL_H

#include "mesh.h"
#include "shader.h"

#include <vector>
using namespace std;

class Model
{
private:
  /*  Model Data */
  vector<Mesh> meshes;
  Shader *shader;
  glm::mat4 modelMatrix;

public:
  /*  Functions   */
  // constructor, expects a filepath to a 3D model.
  Model(vector<Mesh> _meshes) : meshes(_meshes), shader(nullptr)
  {
    modelMatrix = glm::mat4(1.0f);
    modelMatrix = glm::rotate(modelMatrix, glm::radians(-90.0f), glm::vec3(1.0f, 0.0f, 0.0f));
    modelMatrix = glm::scale(modelMatrix, glm::vec3(0.3333f, 0.3333f, 0.333f));
  }
  Model() = default;

  void linkShader(Shader *_shader)
  {
    shader = _shader;
  }
  // draws the model, and thus all its meshes
  void draw(const glm::mat4 viewMatrix, const glm::mat4 projectionMatrix)
  {
    if (shader == nullptr)
    {
      throw std::invalid_argument("ResourceError shader never assigned");
    }
    shader->activate();
    shader->setMatrix4("viewMatrix", viewMatrix);
    shader->setMatrix4("projectionMatrix", projectionMatrix);
    shader->setMatrix4("modelMatrix", modelMatrix);
    for (unsigned int i = 0; i < meshes.size(); i++)
      meshes[i].draw(*shader);
  }
};

#endif