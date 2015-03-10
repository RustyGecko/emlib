CC      = arm-none-eabi-gcc
AR      = arm-none-eabi-ar
AS      = arm-none-eabi-as
OBJCOPY = arm-none-eabi-objcopy

BINARY_NAME   = out
BINARY_FORMAT = hex

DEVICE=EFM32GG990F1024
TARGET=thumbv7m-none-eabi

EXAMPLE_DIR = examples
EXAMPLES    = $(wildcard $(EXAMPLE_DIR)/*.rs)

TEST_DIR = test

PROJ_NAME   = buttons_int

RUSTC = rustc
FLASH = eACommander

-include .emlib_hash

TARGET_DIR = target/$(TARGET)
TARGET_OUT = $(TARGET_DIR)/$(PROJ_NAME)

.PHONY: all setup proj flash test clean

all:    proj
proj:   $(PROJ_NAME).elf $(TARGET_OUT).hex $(TARGET_OUT).bin

AFLAGS   = -mthumb -mcpu=cortex-m3
LDFLAGS  = $(AFLAGS) -Tefm32-common/Device/EFM32GG/Source/GCC/efm32gg.ld
LDFLAGS += --specs=nosys.specs
LDFLAGS += -lgcc -lc -lnosys -lm
LDFLAGS += -Wl,--start-group -lnosys -lgcc -lc -lm -Wl,--start-group

KIT = stk3700
FEATURES = --features $(KIT)

-include Makefile.user

RUSTFLAGS  = --target $(TARGET) --crate-type bin
RUSTFLAGS += -g -C link-args="$(LDFLAGS)"
RUSTFLAGS += -L $(TARGET_DIR) -L $(TARGET_DIR)/deps -L $(TARGET_DIR)/build/emlib-$(HASH)/out
RUSTFLAGS += --emit=dep-info,link --verbose

FLASHFLAGS = --verify --reset

-include test/Makefile

%.elf: $(EXAMPLE_DIR)/$(@:.elf=.rs)
	BUILD_ENV=prod cargo build --target thumbv7m-none-eabi --verbose $(FEATURES)
	@$(AR) -x $(TARGET_DIR)/libemlib-$(HASH).rlib
	@mv *.o emlib-$(HASH).0.bytecode.deflate rust.metadata.bin $(TARGET_DIR)
	$(RUSTC) $<$(@:.elf=.rs) $(RUSTFLAGS) --out-dir $(TARGET_DIR) --crate-name $(@:.elf=)

%.hex: %
	$(OBJCOPY) -O ihex $< $@

%.bin: %
	$(OBJCOPY) -O binary $< $@

flash: all
	cp $(TARGET_DIR)/$(PROJ_NAME).$(BINARY_FORMAT) $(TARGET_DIR)/$(BINARY_NAME).$(BINARY_FORMAT)
	JLinkExe -commanderscript .execute.jlink || echo ""

burn: all
	$(FLASH) --flash $(TARGET_OUT).bin $(FLASHFLAGS)

test: $(notdir $(EXAMPLES:.rs=.elf))
	@echo Done

run-tests: $(TEST_DIR)/run_all_tests.rs mocks
	@mkdir -p test/mocks
	rm -rf target/build/emlib-$(HASH)
	BUILD_ENV=test cargo build --target thumbv7m-none-eabi --verbose $(FEATURES)
	@$(AR) -x $(TARGET_DIR)/libemlib-$(HASH).rlib
	@mv *.o emlib-$(HASH).0.bytecode.deflate rust.metadata.bin $(TARGET_DIR)
	$(RUSTC) $(TEST_DIR)/run_all_tests.rs $(RUSTFLAGS) --out-dir $(TARGET_DIR) --crate-name run_all_tests
	$(OBJCOPY) -O ihex $(TARGET_DIR)/run_all_tests $(TARGET_DIR)/out.hex
	JLinkExe -commanderscript .execute.jlink || echo ""
	@echo Done

clean:
	@cargo clean
