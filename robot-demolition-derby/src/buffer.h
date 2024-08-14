#include <stdbool.h>

#ifndef buffer
#define buffer

typedef struct Buffer {
  int height;
  int width;
  char *pixels;
} Buffer;

Buffer init(int height, int width, char c);

bool draw(Buffer *buffer, int col, int row, char c);

void display(Buffer *buffer);

void clear(Buffer *buffer);

#endif
