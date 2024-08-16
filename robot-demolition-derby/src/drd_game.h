#include <stdbool.h>

#ifndef drd_game
#define drd_game

typedef struct {
  bool isRobot;  // 1 (true) a robot is here; 0 (false) the space is clear
  int direction; // 0 (west), 1 (north), 2 (east), 3 (south)
  bool alive;    // 1 (true) for running robot; (false) when crashed
} GridSquare;

typedef struct {
  int seed;
  int robot_count;
  int action_count;
  int height;
  int width;
  GridSquare *grid;
} Game;

Game start_game();
void game_loop(Game *game);
void stop_game(Game *game);

#endif
