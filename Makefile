CC      = arm-none-eabi-gcc
AR      = arm-none-eabi-ar
AS      = arm-none-eabi-as
OBJCOPY = arm-none-eabi-objcopy

DEVICE=EFM32GG990F1024
TARGET=thumbv7m-none-eabi

PROJ_DIR  = examples
PROJ_NAME = buttons_int

-include Makefile.user

RUSTC = $(RUSTC_PATH)rustc
FLASH = eACommander

HASH = 45127226698c02f0

LIB_PATH = $(SIMPLICITY_STUDIO_HOME)/developer/sdks/efm32/v2
LIB_DIR = lib

TARGET_DIR = target/$(TARGET)
TARGET_OUT = $(TARGET_DIR)/$(PROJ_NAME)

.PHONY: all setup proj flash clean clean_all

all:    proj
proj:   $(TARGET_OUT).elf $(TARGET_OUT).hex $(TARGET_OUT).bin

AFLAGS   = -mthumb -mcpu=cortex-m3
LDFLAGS  = $(AFLAGS) -T$(LIB_PATH)/Device/SiliconLabs/EFM32GG/Source/GCC/efm32gg.ld
LDFLAGS += -Wl,--start-group -lgcc -lc -lnosys -Wl,--end-group

ifeq ($(PLATFORM),Darwin)
LDFLAGS += target/thumbv7m-none-eabi/emlib-$(HASH).o
LIB_DEPENENCIES = $(TARGET_DIR)/libemlib-$(HASH).rlib $(TARGET_DIR)/emlib-$(HASH).o
else
LIB_DEPENENCIES = $(TARGET_DIR)/libemlib-$(HASH).rlib
endif

RUSTFLAGS  = --target $(TARGET) --crate-type bin
RUSTFLAGS += -g -C link-args="$(LDFLAGS)"
RUSTFLAGS += -L $(LIB_DIR) -L $(TARGET_DIR) -L $(TARGET_DIR)/deps -L $(TARGET_DIR)/build/emlib-$(HASH)/out
RUSTFLAGS += --emit=dep-info,link --verbose

FLASHFLAGS = --verify --reset

$(TARGET_DIR)/libemlib-$(HASH).rlib:
	cargo build --target thumbv7m-none-eabi

$(TARGET_DIR)/emlib-$(HASH).o: $(TARGET_DIR)/libemlib-$(HASH).rlib
	arm-none-eabi-ar -x $(TARGET_DIR)/libemlib-$(HASH).rlib
	mv emlib-$(HASH).o emlib-$(HASH).0.bytecode.deflate rust.metadata.bin target/thumbv7m-none-eabi

%.elf: $(PROJ_DIR)/$(PROJ_NAME).rs $(LIB_DEPENENCIES)
	$(RUSTC) $< $(RUSTFLAGS) --out-dir $(TARGET_DIR) --crate-name $(PROJ_NAME)

%.hex: %
	$(OBJCOPY) -O ihex $< $@

%.bin: %
	$(OBJCOPY) -O binary $< $@

flash: all
	$(FLASH) --flash $(TARGET_OUT).bin $(FLASHFLAGS)

clean:
	@cargo clean
