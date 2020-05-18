#include <stdio.h>

char* hello_rust();

int c_sayhello( unsigned int value ) {
    printf( "%s\n", hello_rust() );
    printf( "hello, this is c function %d!\n", value );

    return 80;
}

