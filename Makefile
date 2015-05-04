OBJCOPY = arm-none-eabi-objcopy

TARGET = thumbv7m-none-eabi
KIT    = stk3700

# emlib examples
DIR = examples
OUT = buttons_int

# cargo directories
TARGET_DIR   = target/$(TARGET)/debug
TARGET_OUT   = $(TARGET_DIR)/$(OUT)
EXAMPLES_DIR = $(TARGET_DIR)/examples
EXAMPLES_OUT = $(EXAMPLES_DIR)/$(OUT)

# cargo config
LINKARGS  = -mthumb -mcpu=cortex-m3 -Tefm32-common/Device/EFM32GG/Source/GCC/efm32gg.ld
LINKARGS += --specs=nosys.specs -lgcc -lc -lnosys -lm

RSFLAGS   = --target $(TARGET) --verbose

# flash config
BINARY_NAME   = out
BINARY_FORMAT = hex

FLASH      = eACommander
FLASHFLAGS = --verify --reset

.PHONY: all example test build-tests run-tests flash burn clean clean-emlib mock-dir

all: example

lib:
	cargo linkargs "$(LINKARGS)" $(RSFLAGS) --lib

example: $(OUT).elf $(EXAMPLES_OUT).hex $(EXAMPLES_OUT).bin $(EXAMPLES_OUT).axf

%.elf: $(DIR)/$(@:.elf=.rs)
	cargo linkargs "$(LINKARGS)" $(RSFLAGS) --example $(@:.elf=)

%.hex: %
	$(OBJCOPY) -O ihex $< $@

%.bin: %
	$(OBJCOPY) -O binary $< $@

%.axf: %
	$(OBJCOPY) $< $@

test:
	cargo linkargs "$(LINKARGS)" $(RSFLAGS) --build-examples

-include test/Makefile
build-tests: mock-dir mocks
	BUILD_ENV=test cargo linkargs "$(LINKARGS)" $(RSFLAGS)

run-tests: build-tests
	$(OBJCOPY) -O ihex $(TARGET_DIR)/run_all_tests $(TARGET_DIR)/$(BINARY_NAME).$(BINARY_FORMAT)
	JLinkExe -commanderscript .execute.jlink || echo ""

flash: all
	cp $(EXAMPLES_DIR)/$(OUT).$(BINARY_FORMAT) $(TARGET_DIR)/$(BINARY_NAME).$(BINARY_FORMAT)
	JLinkExe -commanderscript .execute.jlink || echo ""

burn: all
	$(FLASH) --flash $(TARGET_OUT).bin $(FLASHFLAGS)
	@echo Done

clean:
	@cargo clean

clean-emlib:
	@find target/ -iname "*emlib-*" -type d -exec rm -r {} +
	@rm -f $(TARGET_DIR)/run_all_tests;

mock-dir:
	@mkdir -p test/mocks
