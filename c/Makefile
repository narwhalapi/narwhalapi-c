# C library over rust/narwhalapi-ffi. `make test` runs the ASan/UBSan tests and the
# quickstart against the loopback mock; `make leaks` repeats the tests without sanitizers
# under macOS `leaks --atExit` (Linux uses LeakSanitizer through `make test`).
ROOT := $(abspath ..)
CARGO ?= cargo
PYTHON ?= python3
PKG_CONFIG ?= pkg-config
# clang unless the caller chose a compiler (make's built-in default is cc).
ifeq ($(origin CC),default)
CC := clang
endif
BUILD := build
WARNINGS := -std=c99 -Wall -Wextra -Werror
SANITIZE := -fsanitize=address,undefined -fno-sanitize-recover=all -fno-omit-frame-pointer
VERSION := $(shell sed -n 's/^version = "\(.*\)"/\1/p' $(ROOT)/rust/narwhalapi-ffi/Cargo.toml)
RELEASE_DIR := $(ROOT)/rust/target/release
# The test library adds the panic hook (cfg nw_c_tests); it never ships.
TEST_TARGET := $(ROOT)/rust/target/c-tests
TEST_LIB := $(TEST_TARGET)/debug/libnarwhalapi.a
TEST_SOURCES := tests/test_main.c tests/exports.c
ifeq ($(shell uname -s),Darwin)
# From `--print native-static-libs`; clang already links libSystem (libc, libm).
NATIVE_LIBS := -liconv
# Build the Rust objects and the C code for one deployment target.
export MACOSX_DEPLOYMENT_TARGET ?= 11.0
else
NATIVE_LIBS := -lgcc_s -lutil -lrt -lpthread -lm -ldl -lc
endif
PKG_ENV := PKG_CONFIG_PATH=$(abspath $(BUILD))/pkgconfig
RUN := env PYTHON=$(PYTHON) ASAN_OPTIONS=abort_on_error=1 sh tests/with_mock.sh

.PHONY: lib test-lib pc test leaks clean

lib:
	cd $(ROOT)/rust && $(CARGO) build --locked -p narwhalapi-ffi --release

test-lib:
	cd $(ROOT)/rust && RUSTFLAGS="--cfg nw_c_tests" CARGO_TARGET_DIR=$(TEST_TARGET) \
	  $(CARGO) build --locked -p narwhalapi-ffi

pc: lib
	mkdir -p $(BUILD)/pkgconfig
	@if [ "$$(uname -s)" = Linux ]; then ln -sf libnarwhalapi.so $(RELEASE_DIR)/libnarwhalapi.so.0; fi
	sed -e 's|@LIBDIR@|$(RELEASE_DIR)|' -e 's|@INCLUDEDIR@|$(abspath include)|' \
	  -e 's|@VERSION@|$(VERSION)|' -e 's|@NATIVE_LIBS@|$(NATIVE_LIBS)|' \
	  narwhalapi.pc.in > $(BUILD)/pkgconfig/narwhalapi.pc

# Tests are not part of this distribution; release snapshots carry no tests/.
ifeq ($(wildcard tests/test_main.c),)
test leaks:
	@echo "$@: tests are not part of this distribution" >&2; exit 1

else
test: test-lib pc
	mkdir -p $(BUILD)
	$(CC) $(WARNINGS) $(SANITIZE) -g -Iinclude -DNW_UNSTABLE $(TEST_SOURCES) $(TEST_LIB) \
	  $(NATIVE_LIBS) -lpthread -o $(BUILD)/test_sanitized
	$(CC) $(WARNINGS) $(SANITIZE) examples/quickstart.c \
	  $$($(PKG_ENV) $(PKG_CONFIG) --cflags --libs narwhalapi) -o $(BUILD)/quickstart
	$(RUN) $(BUILD)/test_sanitized
	$(RUN) sh -c 'NARWHAL_API_KEY=test-key NARWHAL_BASE_URL=$$NW_TEST_BASE_URL $(BUILD)/quickstart'

ifeq ($(shell uname -s),Darwin)
leaks: test-lib
	mkdir -p $(BUILD)
	$(CC) $(WARNINGS) -g -Iinclude -DNW_UNSTABLE $(TEST_SOURCES) $(TEST_LIB) \
	  $(NATIVE_LIBS) -lpthread -o $(BUILD)/test_plain
	$(RUN) leaks --atExit -- $(BUILD)/test_plain
else
leaks:
	@echo "$@: LeakSanitizer already ran in make test"
endif
endif

clean:
	rm -rf $(BUILD)
