#ifndef RESOURCE_MANAGER_H
#define RESOURCE_MANAGER_H

#include <map>
#include <string>

#include <glad/glad.h>

// #include "texture.h"
#include "shader.h"
#include "model.h"

// A static singleton ResourceManager class that hosts several
// functions to load Textures and Shaders. Each loaded texture
// and/or shader is also stored for future reference by string
// handles. All functions and resources are static and no
// public constructor is defined.
class ResourceManager
{
public:
  // resource storage
  static std::map<std::string, Shader> shaders;
  static std::map<std::string, Model> models;
  // loads (and generates) a shader program from file loading vertex, fragment (and geometry) shader's source code. If gShaderFile is not nullptr, it also loads a geometry shader
  static Shader loadShader(const char *vShaderFile, const char *fShaderFile, const char *gShaderFile, std::string name);
  // retrieves a stored sader
  static Shader getShader(const std::string name);
  // loads (and generates) a texture from file
  static Model loadModel(const string &file, const string name);
  // static Texture2D loadTexture(const char *file, bool alpha, std::string name);
  // // retrieves a stored texture
  static Model getModel(const std::string name);

  static void linkModelShader(const string modelName, const string shaderName);
  // static Texture2D getTexture(std::string name);
  // // properly de-allocates all loaded resources
  static void clear();

private:
  static std::map<std::string, std::string> modelShaderLinks;
  // private constructor, that is we do not want any actual resource manager objects. Its members and functions should be publicly available (static).
  ResourceManager() {}
  // loads and generates a shader from file
  static Shader loadShaderFromFile(const char *vShaderFile, const char *fShaderFile, const char *gShaderFile = nullptr);
  // loads a single texture from file
  static Model loadModelFromFile(const char *file);
  // static Texture2D loadTextureFromFile(const char *file, bool alpha);
};

#endif