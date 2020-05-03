#include <glad/glad.h>
#include <GLFW/glfw3.h>

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>

#include "game.h"
#include "utils/text_overlay.h"

#include <iostream>

#define SCR_WIDTH 1600
#define SCR_HEIGHT 900

#define SCR_CENTER_X SCR_WIDTH / 2.0f
#define SCR_CENTERY_Y SCR_HEIGHT / 2.0f

#define DEBUG_MODE
#define BLUE glm::vec3(0.2f, 0.2f, 1.0f)

#define MSAA_MULTIPLICITY 8 // how much oversampling for anti-aliasing?

using std::cout;
using std::endl;

// Init functions for opengl and glfw
// ----------------------------------
bool window_initialize(const Game &game);
void framebuffer_size_callback(GLFWwindow *window, int width, int height);

// Callbacks for acknowledging user input
// --------------------------------------
void mouse_callback(GLFWwindow *window, double xpos, double ypos);
void scroll_callback(GLFWwindow *window, double xoffset, double yoffset);
void key_callback(GLFWwindow *window, int key, int scancode, int action, int mode);

// Static window and game object
// -----------------------------
static GLFWwindow *window;
static Game game(SCR_WIDTH, SCR_HEIGHT);

// A few variables that must be staic for input handling callbacks
// ---------------------------------------------------------------
static bool focused = true;                        // boolean saying game has focus or not.
static float _lastPosX = (float)SCR_WIDTH / 2.0f;  // Needed for mouse moved callback
static float _lastPosY = (float)SCR_HEIGHT / 2.0f; // Needed for mouse moved callback
static bool firstMouse = true;

int main()
{

  if (!window_initialize(game))
  {
    std::cout << "SESSION TERMINATED" << endl;
    return -1;
  }

#ifdef DEBUG_MODE
  TextOverlay debugOverlay(SCR_WIDTH, SCR_HEIGHT);
#endif

  game.init();

  // draw in wireframe
  //glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);

  // render loop
  // -----------

  cout << "Starting render loop" << endl;

  float dt = glfwGetTime();
  float lastFrameTime = glfwGetTime();

  while (!glfwWindowShouldClose(window))
  {
    // per-frame time logic
    // --------------------
    float currentFrame = glfwGetTime();
    dt = currentFrame - lastFrameTime;
    lastFrameTime = currentFrame;

    // process input
    // -----
    glfwPollEvents();
    game.processInput(dt);

    // Update game state
    // -----------------
    game.update(dt);

    // render
    // ------
    glClearColor(0.08f, 0.08f, 0.08f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    game.render();

// Debug mode actions
// ------------------
#ifdef DEBUG_MODE
    char debugBuff[512];
    sprintf(debugBuff, "%d FPS", (int)(1.0f / dt));
    debugOverlay.writeText(debugBuff, vec2(20, SCR_HEIGHT - 60), 1.0f, BLUE);
    glm::vec3 position = game.player.getPosition();
    glm::vec3 front = game.player.front();
    glm::vec3 velocity = game.player.velocity;
    sprintf(debugBuff, "Position <%.2f, %.2f, %.2f>", position.x, position.y, position.z);
    debugOverlay.writeText(debugBuff, glm::vec2(20, 20), 1.0f, BLUE);
    sprintf(debugBuff, "Front <%.2f, %.2f, %.2f>", front.x, front.y, front.z);
    debugOverlay.writeText(debugBuff, glm::vec2(20, 80), 1.0f, BLUE);
    sprintf(debugBuff, "Velocity <%.2f, %.2f, %.2f>", velocity.x, velocity.y, velocity.z);
    debugOverlay.writeText(debugBuff, glm::vec2(20, 140), 1.0f, BLUE);
#endif

    // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
    // -------------------------------------------------------------------------------
    glfwSwapBuffers(window);
  }

  // glfw: terminate, clearing all previously allocated GLFW resources.
  // ------------------------------------------------------------------
  glfwTerminate();
  return 0;
}

// glfw: whenever the window size changed (by OS or user resize) this callback function executes
// ---------------------------------------------------------------------------------------------
void framebuffer_size_callback(GLFWwindow *window, int width, int height)
{
  // make sure the viewport matches the new window dimensions; note that width and
  // height will be significantly larger than specified on retina displays.
  glViewport(0, 0, width, height);
}

void mouse_callback(GLFWwindow *window, double xpos, double ypos)
{
  if(firstMouse)
  {
    _lastPosX = xpos;
    _lastPosY = ypos;
    firstMouse = false;
  }

  game.mouseMoved(xpos - _lastPosX, _lastPosY - ypos);
  _lastPosX = xpos;
  _lastPosY = ypos;
}
void scroll_callback(GLFWwindow *window, double xoffset, double yoffset)
{
  game.mouseScrolled(yoffset);
}

void key_callback(GLFWwindow *window, int key, int scancode, int action, int mode)
{
#ifdef DEBUG_MODE
  if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS && focused)
    glfwSetWindowShouldClose(window, true);

#else
  // When a user presses the escape key, we set the WindowShouldClose property to true, closing the application
  if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS && focused)
  {
    glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_NORMAL);
    focused = false;
  }
  else if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS && !focused)
  {
    glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_DISABLED);
    focused = true;
  }
#endif
  if (action == GLFW_PRESS)
    game.keyPressed(key);
  if (action == GLFW_RELEASE)
    game.keyReleased(key);
}

// This function capsulates all the boilerplate of openning a window and giving openGL control of it.
bool window_initialize(const Game &game)
{
  // glfw: initialize and configure
  // ------------------------------
  glfwInit();
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
  glfwWindowHint(GLFW_SAMPLES, MSAA_MULTIPLICITY);

#ifdef __APPLE__
  glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE); // uncomment this statement to fix compilation on OS X
#endif

  // glfw window creation
  // --------------------
  window = glfwCreateWindow(SCR_WIDTH, SCR_HEIGHT, "Relativistic Motion", NULL, NULL);
  if (window == NULL)
  {
    std::cout << "Failed to create GLFW window" << std::endl;
    glfwTerminate();
    return false;
  }
  glfwMakeContextCurrent(window);
  glfwSetFramebufferSizeCallback(window, framebuffer_size_callback);
  glfwSetCursorPosCallback(window, mouse_callback);
  glfwSetScrollCallback(window, scroll_callback);
  glfwSetKeyCallback(window, key_callback);

  // tell GLFW to capture our mouse
  glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_DISABLED);

  // glad: load all OpenGL function pointers
  // ---------------------------------------
  if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress))
  {
    std::cout << "Failed to initialize GLAD" << std::endl;
    return false;
  }

  // configure global opengl state
  // -----------------------------
  glEnable(GL_DEPTH_TEST);
  glEnable(GL_MULTISAMPLE);
  glPixelStorei(GL_UNPACK_ALIGNMENT, 1); // Disable byte-alignment restriction
  // glEnable(GL_CULL_FACE);
  glEnable(GL_BLEND);
  glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

  return true;
}
