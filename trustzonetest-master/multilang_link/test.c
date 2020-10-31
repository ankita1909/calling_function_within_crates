extern char run(); 

extern volatile unsigned int * const UART_DR = (unsigned int *)0x4000c000;
extern volatile unsigned int * const SOME_ADDR = (unsigned int *)0x40626364;

extern volatile unsigned int shift = 0;

static void uart_print(const char *s) {
	while (*s != '\0') {
		*UART_DR = *s;
		s++;
	}
}
void main(void) {
	char x = 0;
	uart_print("Hello, World!\n");

	for(unsigned int i = 0; i<4; i++) {
		shift = (shift + 8);
		if (x == 0) {
			uart_print("Rust says <nothing>\n");
		} else {
			char string[14] = {'R','u','s','t',' ','s','a','y','s',':',' ', x, '\n', '\0'};
			uart_print(string);
		}
	}
	
	uart_print("done.------------------------------------\n");
}
