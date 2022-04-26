.PHONY: build
build: build-containerd

.PHONY: check-submodules
check-submodules:
	@if [ ! -f "go-tools/containerd/Makefile" ]; \
	then \
		git submodule update --init --recursive;\
	fi;

.PHONY: build-containerd
build-containerd: check-submodules
	@cd go-tools/containerd && make && make install DESTDIR=../../target
	@mv ./target/usr/local/bin ./target/bin
	@rm -fR ./target/usr
