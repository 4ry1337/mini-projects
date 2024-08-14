#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

struct Buffer {
  int height;
  int width;
  char *pixels;
};

typedef struct Buffer Buffer;

Buffer init(int height, int width, char c) {
  Buffer buf;

  buf.height = height;
  buf.width = width;

  buf.pixels = (char *)malloc(height * width * sizeof(char));

  if (buf.pixels == NULL) {
    printf("Memory allocation failed\n");
    exit(1);
  }

  for (int i = 0; i < height * width; i++) {
    buf.pixels[i] = c;
  };

  return buf;
}

bool draw(Buffer *buffer, int row, int col, char c) {
  if (0 > row || row > buffer->height || 0 > col || col > buffer->width) {
    printf("Invalid input\n");
    return false;
  }
  buffer->pixels[row * buffer->height + col] = c;
  return true;
}

void display(Buffer *buffer) {
  for (int row = 0; row < buffer->height; row++) {
    for (int col = 0; col < buffer->width; col++) {
      printf("%c", buffer->pixels[row * buffer->height + col]);
    }
    printf("\n");
  }
}

void clear(Buffer *buffer) {
  free(buffer->pixels);
  buffer->pixels = NULL;
  buffer->width = 0;
  buffer->height = 0;
}
