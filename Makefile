.PHONY: build
build: build-spiderwebd build-containerd
	@cp -fR config ./target/debug

.PHONY: run
run: build
	./target/debug/spider-web

.PHONY: check-submodules
check-submodules:
	@if [ ! -f "go-tools/containerd/Makefile" ]; \
	then \
		git submodule update --init --recursive;\
	fi;

.PHONY: build-spidewebd
build-spiderwebd:
	@cargo build

.PHONY: build-containerd
build-containerd: check-submodules
	@cd go-tools/containerd && make && make install DESTDIR=../../target
	@mv ./target/usr/local/bin/* ./target/debug
	@rm -fR ./target/usr
