#include "buffer.h"

#define SPACE '.'
#define ROBOT 'R'

int main(int argc, char *argv[]) {
  Buffer buf = init(10, 10, SPACE);
  draw(&buf, 5, 5, ROBOT);
  display(&buf);
  clear(&buf);
  return 0;
}
