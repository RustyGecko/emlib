#include <stdint.h>

uint32_t strlen(const uint8_t* str)
{
   uint32_t i = 0;
   while(str[i] != 0){i++;}
   return i;
}
