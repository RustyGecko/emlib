CC      = arm-none-eabi-gcc
AR      = arm-none-eabi-ar
AS      = arm-none-eabi-as
OBJCOPY = arm-none-eabi-objcopy

DEVICE=EFM32GG990F1024
TARGET=thumbv7m-none-eabi

PROJ_DIR  = examples
PROJ_NAME = buttons_int

RUSTC = rustc
FLASH = eACommander

-include .emlib_hash

TARGET_DIR = target/$(TARGET)
TARGET_OUT = $(TARGET_DIR)/$(PROJ_NAME)

.PHONY: all setup proj flash clean

all:    proj
proj:   $(TARGET_OUT).elf $(TARGET_OUT).hex $(TARGET_OUT).bin

AFLAGS   = -mthumb -mcpu=cortex-m3
LDFLAGS  = $(AFLAGS) -Tefm32-common/Device/EFM32GG/Source/GCC/efm32gg.ld
LDFLAGS += -Wl,--start-group -lgcc -lc -lnosys -Wl,--end-group

-include Makefile.user

RUSTFLAGS  = --target $(TARGET) --crate-type bin
RUSTFLAGS += -g -C link-args="$(LDFLAGS)"
RUSTFLAGS += -L $(TARGET_DIR) -L $(TARGET_DIR)/deps -L $(TARGET_DIR)/build/emlib-$(HASH)/out
RUSTFLAGS += --emit=dep-info,link --verbose

FLASHFLAGS = --verify --reset

%.elf: $(PROJ_DIR)/$(PROJ_NAME).rs
	cargo build --target thumbv7m-none-eabi --verbose
	@$(AR) -x $(TARGET_DIR)/libemlib-$(HASH).rlib
	@mv *.o emlib-$(HASH).0.bytecode.deflate rust.metadata.bin $(TARGET_DIR)
	$(RUSTC) $< $(RUSTFLAGS) --out-dir $(TARGET_DIR) --crate-name $(PROJ_NAME)

%.hex: %
	$(OBJCOPY) -O ihex $< $@

%.bin: %
	$(OBJCOPY) -O binary $< $@

flash: all
	$(FLASH) --flash $(TARGET_OUT).bin $(FLASHFLAGS)

clean:
	@cargo clean
