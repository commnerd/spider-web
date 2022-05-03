EXEC := "spider-web"
TGT_PATH := "$${PWD}/target/debug"
ETC_PATH := "${TGT_PATH}/etc/spiderweb"
ETC_REGX := $(shell echo ${ETC_PATH} | sed "s#\/#\\\/#g")
LIB_PATH := "${TGT_PATH}/lib/spiderweb"
VAR_PATH := "${TGT_PATH}/var/lib/spiderweb"

.PHONY: build
build: build-spiderwebd prep-dir-struct build-containerd
	@cp -fR config/* ${ETC_PATH}
	@sed -i ${ETC_PATH}/containerd.toml -e "s/ROOT/${ETC_REGX}/g"

.PHONY: prep-dir-struct
prep-dir-struct:
	@mkdir -p ${ETC_PATH}
	@mkdir -p ${VAR_PATH}/containerd/opt

.PHONY: run
run: build
	@sudo ${TGT_PATH}/${EXEC}

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
	@mv ./target/usr/local/bin/* ${TGT_PATH}
	@rm -fR ./target/usr
