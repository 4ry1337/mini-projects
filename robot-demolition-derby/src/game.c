#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
  bool isRobot;  // 1 (true) a robot is here; 0 (false) the space is clear
  int direction; // 0 (west), 1 (north), 2 (east), 3 (south)
  bool alive;    // 1 (true) for running robot; (false) when crashed
} Point;

typedef struct {
  int height;
  int width;
  Point *points;
} Grid;

bool put(Grid *grid, int row, int col) {
  if (!grid->points[row * grid->height + col].isRobot) {
    grid->points[row * grid->height + col].isRobot = true;
    return true;
  }
  return false;
}

void clear(Grid *grid) {
  for (int i = 0; i < grid->height * grid->width; i++) {
    grid->points[i].isRobot = false;
  }
}

void display(Grid *grid) {
  for (int row = 0; row < grid->height; row++) {
    for (int col = 0; col < grid->width; col++) {
      Point curr = grid->points[row * grid->height + col];
      printf("%c", curr.isRobot ? (curr.alive ? 'R' : '@') : '.');
    }
    printf("\n");
  }
}

typedef struct {
  int seed;
  int robot_count;
  int action_count;
  Grid grid;
} Game;

void start(Game *game, unsigned int seed, unsigned int robot_count) {
  srand(seed);
  for (int i = 0; i < robot_count; i++) {
    int row = rand();
  }
}

void stop(Game *game) { free(game->grid.points); }

void loop(Game *game) {
  char action;
  int row, col;
  while (1) {
    printf("input action and coordinates, please:");
    scanf("%c%d%d", &action, &row, &col);
    if (action == 'F') {

    } else if (action == 'L') {

    } else if (action == 'R') {

    } else if (action == 'S') {
      printf("Thanks for playing!");
      stop(game);
    } else {
    }
  }
}
