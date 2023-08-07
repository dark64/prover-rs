libhello.a: hello.o
	ar rcs libhello.a hello.o

hello.o: hello.c
	clang -c -o hello.o hello.c

clean:
	rm *.o *.a