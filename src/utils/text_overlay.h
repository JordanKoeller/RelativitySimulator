# ifndef TEXT_OVERLAY_H
# define TEXT_OVERLAY_H

#include <string>

#include <glm/glm.hpp>


using glm::vec2;
using glm::vec3;

class TextOverlay {
public:
  TextOverlay(float w, float h);

  void writeText(const std::string &text, vec2 position, GLfloat scale, vec3 color);
};

#endif