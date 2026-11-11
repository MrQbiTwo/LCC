# Build the native compiler and standard-library-only Rust tools.
BUILD ?= build
NASM ?= nasm
LD ?= ld
RUSTC ?= rustc

.PHONY: all test interop package bench clean force
all: $(BUILD)/lcc

$(BUILD):
	mkdir -p "$(BUILD)"

runtime_tools := tools/runtime/elf.rs tools/runtime/pack.rs tools/runtime/build.rs
release_sources := $(shell find tools/release -name '*.rs')
runtime_tests := tools/runtime/tests.rs tools/runtime/build_tests.rs tools/runtime/compiler_tests.rs
runtime_sources := $(shell find runtime -type f)
compiler_sources := $(shell find src include -type f)

$(BUILD)/build_main: tools/build_main.rs $(runtime_tools) | $(BUILD)
	$(RUSTC) --edition=2021 -O -D warnings $< -o "$@"

$(BUILD)/runtime_pack.inc: force $(BUILD)/build_main tools/pack_runtime.rs $(runtime_sources)
	"$(BUILD)/build_main" --runtime "$(BUILD)" "$(NASM)" "$(LD)"

# Validate content fingerprints even when generated files have newer timestamps.
$(BUILD)/lcc: force $(BUILD)/build_main $(compiler_sources) $(runtime_sources)
	"$(BUILD)/build_main" "$(BUILD)" "$(NASM)" "$(LD)"

test_sources := tests/tests_main.rs tests/catalog.rs tests/group_template.rs $(shell find tests/support tests/groups -name '*.rs')
$(BUILD)/tests_main: $(test_sources) | $(BUILD)
	$(RUSTC) --edition=2021 -O -D warnings tests/tests_main.rs -o "$@"

$(BUILD)/suite_tests: $(test_sources) | $(BUILD)
	$(RUSTC) --edition=2021 -O -D warnings --test tests/tests_main.rs -o "$@"

$(BUILD)/release_tests: tools/release_main.rs $(release_sources) $(runtime_tools) $(runtime_tests) | $(BUILD)
	$(RUSTC) --edition=2021 -O -D warnings --test $< -o "$@"

$(BUILD)/build_tests: tools/build_main.rs $(runtime_tools) $(runtime_tests) | $(BUILD)
	$(RUSTC) --edition=2021 -O -D warnings --test $< -o "$@"

$(BUILD)/process_tests: tests/process_main.rs tests/support/process.rs | $(BUILD)
	$(RUSTC) --edition=2021 -O -D warnings --test $< -o "$@"

test: all $(BUILD)/tests_main $(BUILD)/suite_tests $(BUILD)/release_tests $(BUILD)/build_tests $(BUILD)/process_tests
	"$(BUILD)/tests_main" "$(abspath $(BUILD)/lcc)"
	"$(BUILD)/suite_tests"
	"$(BUILD)/release_tests"
	"$(BUILD)/build_tests" tests:: --skip pack:: --skip build::
	"$(BUILD)/process_tests"

# GCC and Clang are optional verification endpoints for the shared ABI.
interop: all $(BUILD)/tests_main
	"$(BUILD)/tests_main" "$(abspath $(BUILD)/lcc)" --interop

benchmark_sources := tests/bench/cases.rs $(shell find tests/bench/cases -name '*.rs') $(shell find tests/fixtures/perf -name '*.c')
$(BUILD)/compile_speed: tests/bench/compile_speed.rs $(benchmark_sources) $(shell find tests/bench/speed -name '*.rs') tests/support/process.rs | $(BUILD)
	$(RUSTC) --edition=2021 -O -D warnings $< -o "$@"

$(BUILD)/validate_cases: tests/bench/validate_cases.rs $(benchmark_sources) tests/support/process.rs | $(BUILD)
	$(RUSTC) --edition=2021 -O -D warnings $< -o "$@"

BENCH_TRIALS ?= 3
bench: all $(BUILD)/compile_speed
	$(if $(BENCH_WORK),,$(error Set BENCH_WORK to a new working directory))
	$(if $(BENCH_ARCHIVE),,$(error Set BENCH_ARCHIVE to a new persistent archive directory))
	"$(BUILD)/compile_speed" "$(abspath $(BUILD)/lcc)" "$(BENCH_WORK)" "$(BENCH_ARCHIVE)" "$(BENCH_TRIALS)" "$(BENCH_FILTER)"

$(BUILD)/release_main: tools/release_main.rs $(release_sources) $(runtime_tools) | $(BUILD)
	$(RUSTC) --edition=2021 -O -D warnings $< -o "$@"

$(BUILD)/pack_runtime: tools/pack_runtime.rs $(runtime_tools) | $(BUILD)
	$(RUSTC) --edition=2021 -O -D warnings $< -o "$@"

# Package only the implemented host/target pair into a new artifact directory.
PACKAGE ?= $(BUILD)/package/linux_x86_64
package: $(BUILD)/release_main
	"$(BUILD)/release_main" "." "$(PACKAGE)" "$(NASM)" "$(LD)"

clean: $(BUILD)/build_main
	"$(BUILD)/build_main" --clean "$(BUILD)"
