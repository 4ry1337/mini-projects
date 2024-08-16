#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define SPACE '.'
#define ROBOT 'R'
#define DEAD_ROBOT '@'
#define SIZE 10
#define ROBOT_COUNT 10

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

void display_grid(Game *game) {
  printf("  ");
  for (int col = 0; col < game->width; col++) {
    printf("%d", col);
  }
  printf("\n");
  for (int row = 0; row < game->height; row++) {
    printf("%d ", row);
    for (int col = 0; col < game->width; col++) {
      GridSquare curr = game->grid[row * game->height + col];
      printf("%c", !curr.isRobot ? SPACE : curr.alive ? ROBOT : DEAD_ROBOT);
    }
    printf("\n");
  }
}

bool valid_coordinate(Game *game, int row, int col) {
  return row < 0 || col < 0 || row >= game->height || col >= game->width;
}

bool put_robot(GridSquare *grid, int pos, int direction) {
  if (!grid[pos].isRobot) {
    GridSquare *curr = &grid[pos];
    curr->isRobot = true;
    curr->alive = true;
    curr->direction = direction;
    return true;
  }
  return false;
}

void clear_robots(Game *game) {
  for (int i = 0; i < game->height * game->width; i++) {
    game->grid[i].isRobot = false;
  }
}

void move_robot_forward(Game *game, int row, int col) {
  GridSquare *curr = &game->grid[row * game->height + col];
  if (!curr->isRobot || !curr->alive) {
    return;
  }
  GridSquare *next = NULL;
  if (curr->direction == 0 && col > 0) {
    next = &game->grid[row * game->height + col - 1];
  }
  if (curr->direction == 1 && row > 0) {
    next = &game->grid[(row - 1) * game->height + col];
  }
  if (curr->direction == 2 && col < SIZE) {
    next = &game->grid[row * game->height + col + 1];
  }
  if (curr->direction == 3 && row < SIZE) {
    next = &game->grid[(row + 1) * game->height + col];
  }
  if (next != NULL) {
    if (next->isRobot) {
      curr->alive = false;
      game->robot_count--;
      if (next->alive) {
        next->alive = false;
        game->robot_count--;
      }
    } else {
      next->direction = curr->direction;
      next->isRobot = next->alive = true;
      curr->isRobot = curr->alive = false;
    }
  }
}

void rotate_robot_right(Game *game, int row, int col) {
  GridSquare *curr = &game->grid[row * game->height + col];
  curr->direction++;
  curr->direction %= 4;
}

void rotate_robot_left(Game *game, int row, int col) {
  GridSquare *curr = &game->grid[row * game->height + col];
  curr->direction--;
  curr->direction %= 4;
}

void place_robots(Game *game) {
  srand(game->seed);
  int robot_count = ROBOT_COUNT;
  do {
    int row = rand() % game->height;
    int col = rand() % game->width;
    int direction = rand() % 4;
    if (put_robot(game->grid, row * game->height + col, direction)) {
      robot_count--;
    }
  } while (robot_count != 0);
}

void stop_game(Game *game) {
  free(game->grid);
  game->width = 0;
  game->height = 0;
  game->seed = 0;
  game->robot_count = 0;
  game->action_count = 0;
}

Game start_game() {
  Game game = {
      time(NULL), ROBOT_COUNT,
      0,          SIZE,
      SIZE,       (GridSquare *)malloc(sizeof(GridSquare) * SIZE * SIZE)};

  if (game.grid == NULL) {
    printf("Memory allocation failed\n");
    exit(1);
  }

  printf("\nDemolition Robot Derby");
  printf("\nY - Play");
  printf("\nN - Exit");
  printf("\nC - Play with custom seed");
  printf("\nInput: ");

  char decision;
  scanf(" %c", &decision);

  if (decision == 'N') {
    stop_game(&game);
    exit(0);
  }
  if (decision == 'C') {
    printf("Input seed: ");
    scanf(" %d", &game.seed);
  }

  place_robots(&game);

  return game;
}

void game_loop(Game *game) {
  char action;
  int row, col;
  while (1) {
    if (game->robot_count == 0) {
      printf("\nYou won!!!");
      printf("\nWanna play again?");
      printf("\nY - Play");
      printf("\nN - Exit");
      printf("\nC - Play with custom seed");
      printf("\nInput: ");

      char decision;
      scanf(" %c", &decision);

      if (decision == 'N') {
        break;
      }

      game->seed = time(NULL);
      game->robot_count = ROBOT_COUNT;
      game->action_count = 0;

      if (decision == 'C') {
        printf("Input seed: ");
        scanf(" %d", &game->seed);
      }

      clear_robots(game);
      place_robots(game);
    }

    display_grid(game);
    printf("Robot Count: %d\nAction Count: %d\n", game->robot_count,
           game->action_count);
    printf("Input action and coordinates (e.g., 'F 2 3'): ");
    scanf(" %c", &action);

    switch (action) {
    case 'F':
    case 'L':
    case 'R':
      scanf(" %d%d", &row, &col);
      if (valid_coordinate(game, row, col)) {
        printf("Invalid coordinates! Please enter values within the grid.\n");
        continue;
      }
      game->action_count++;
      switch (action) {
      case 'F':
        move_robot_forward(game, row, col);
        break;
      case 'L':
        rotate_robot_left(game, row, col);
        break;
      case 'R':
        rotate_robot_right(game, row, col);
        break;
      }
      break;
    case 'W':
      // Too check restart buttons
      game->robot_count = 0;
      break;
    case 'S':
      printf("Game Seed: %d\nThanks for playing!\n", game->seed);
      return;
    case 'H':
      printf("Help:\n");
      printf("F (Row) (Col)\tMove Forward\n");
      printf("L (Row) (Col)\tTurn Left\n");
      printf("R (Row) (Col)\tTurn Right\n");
      printf("S\t\tStop the game\n");
      printf("H\t\tDisplay this help message\n");
      break;
    default:
      printf("Invalid input! Type 'H' for help with the available actions.\n");
      break;
    }
  }
}
