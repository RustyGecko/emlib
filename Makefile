DEVICE=EFM32GG990F1024
TARGET=thumbv7m-none-eabi

PROJ_DIR  = examples
PROJ_NAME = buttons_int

-include Makefile.user

RUSTC = $(RUSTC_PATH)rustc
FLASH = eACommander

LIB_PATH = $(SIMPLICITY_STUDIO_HOME)/developer/sdks/efm32/v2
LIB_DIR = lib

TARGET_DIR = target/$(TARGET)
TARGET_OUT = $(TARGET_DIR)/$(PROJ_NAME)

.PHONY: all setup proj flash clean clean_all

all:    setup proj
proj:   $(TARGET_OUT).elf $(TARGET_OUT).hex $(TARGET_OUT).bin

setup:
	@mkdir -p lib

include Makefile.emlib

LDFLAGS  = $(AFLAGS) -T$(LIB_PATH)/Device/SiliconLabs/EFM32GG/Source/GCC/efm32gg.ld
LDFLAGS += -Wl,--start-group -lgcc -lc -lnosys -Wl,--end-group

RUSTFLAGS  = --target $(TARGET) --crate-type bin
RUSTFLAGS += -g -C link-args="$(LDFLAGS)"
RUSTFLAGS += -L dependency=$(TARGET_DIR)
RUSTFLAGS += -L dependency=$(TARGET_DIR)/deps
RUSTFLAGS += -L $(LIB_DIR)
RUSTFLAGS += --extern core=$(TARGET_DIR)/deps/libcore-9bcb639479e20b80.rlib
RUSTFLAGS += --extern emlib=$(TARGET_DIR)/libemlib-9b44da1ad5dde3c6.rlib
RUSTFLAGS += --emit=dep-info,link --verbose

FLASHFLAGS = --verify --reset

%.elf: $(PROJ_DIR)/$(PROJ_NAME).rs $(LIB_DIR)/libcompiler-rt.a
	cargo build --target thumbv7m-none-eabi
	$(RUSTC) $< $(RUSTFLAGS) --out-dir $(TARGET_DIR) --crate-name $(PROJ_NAME)

%.hex: %
	$(OBJCOPY) -O ihex $< $@

%.bin: %
	$(OBJCOPY) -O binary $< $@

flash: all
	$(FLASH) --flash $(TARGET_OUT).bin $(FLASHFLAGS)

clean:
	@rm -rf lib

clean_all: clean
	@find . -iname "*.o" -exec rm {} \;
	@cargo clean
