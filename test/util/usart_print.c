#include <em_usart.h>

int print_char(int ch) {
    USART_Tx(USART1, ch);
    return 0;
}
