EXEC := "spider-web"
EXC_PATH := "/etc/spiderweb"
LIB_PATH := "/lib/spiderweb"
VAR_PATH := "/var/lib/spiderweb"

.PHONY: build
build: build-spiderwebd prep-dir-struct build-containerd
	@cp -fR config ./target/debug${EXC_PATH}

.PHONY: prep-dir-struct
prep-dir-struct:
	@mkdir -p ./target/debug${EXC_PATH}
	@mkdir -p ./target/debug${VAR_PATH}/containerd/opt

.PHONY: run
run: build
	@./target/debug/${EXEC}

.PHONY: check-submodules
check-submodules:
	@if [ ! -f "go-tools/containerd/Makefile" ]; \
	then \
		git submodule update --init --recursive;\
	fi;

.PHONY: build-spidewebd check-submodules
build-spiderwebd:
	@cargo build

.PHONY: build-containerd
build-containerd: check-submodules
	@cd go-tools/containerd && make && make install DESTDIR=../../target
	@mv ./target/usr/local/bin/* ./target/debug
	@rm -fR ./target/usr
