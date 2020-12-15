foo:
	cd rust && cargo build --verbose
	gcc main.c ./rust/target/debug/librust_foo.a -lpthread -o foo
	chmod +x foo
