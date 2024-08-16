#include "drd_game.h"
#include <time.h>

int main(int argc, char *argv[]) {
  Game game = start_game();
  game_loop(&game);
  stop_game(&game);
  return 0;
}
