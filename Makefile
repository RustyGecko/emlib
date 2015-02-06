DEVICE=EFM32GG990F1024
TARGET=thumbv7m-none-eabi

PROJ_DIR=examples
PROJ_NAME=buttons_int

-include Makefile.user


RUSTC = $(RUSTC_PATH)rustc
FLASH = eACommander

LIB_PATH = $(SIMPLICITY_STUDIO_HOME)/developer/sdks/efm32/v2
LIB_DIR = lib
OUT_DIR = out
OUT     = $(OUT_DIR)/$(PROJ_NAME)


.PHONY: all setup proj
all:    setup proj
proj:   $(OUT).elf $(OUT).hex $(OUT).bin

setup:
	@mkdir -p lib
	@mkdir -p out

include Makefile.emlib
include Makefile.rustlib


LDFLAGS  = $(AFLAGS) -T$(LIB_PATH)/Device/SiliconLabs/EFM32GG/Source/GCC/efm32gg.ld
LDFLAGS += -Wl,--start-group -lgcc -lc -lnosys -Wl,--end-group lib/emlib.o

RUSTFLAGS  = --target $(TARGET)
RUSTFLAGS += -g -C link-args="$(LDFLAGS)"
RUSTFLAGS += -L $(LIB_DIR) --verbose

FLASHFLAGS = --verify --reset


%.elf: $(PROJ_DIR)/$(PROJ_NAME).rs $(LIB_DIR)/libcompiler-rt.a $(LIB_DIR)/libcore.rlib $(LIB_DIR)/emlib.o
	$(RUSTC) $(RUSTFLAGS) $< -o $@

%.hex: %.elf
	$(OBJCOPY) -O ihex $< $@

%.bin: %.elf
	$(OBJCOPY) -O binary $< $@


.PHONY: flash
flash: all
	$(FLASH) --flash $(OUT).bin $(FLASHFLAGS)

.PHONY:clean
clean:
	@rm -rf lib
	@rm -rf out

