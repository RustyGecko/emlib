MOCK = ruby test/lib/cmock/lib/cmock.rb
MOCK_FLAGS = -otest/cmock.yml

EM_LIB_DIR = efm32-common/emlib/inc
SRC_DIR = src

SRCS = $(SRC_DIR)/timer/timer.h

LIBS = $(EM_LIB_DIR)/em_timer.h \
    $(EM_LIB_DIR)/em_adc.h

.PHONY: mocks

mocks: $(SRCS) $(LIBS)
	$(MOCK) $(MOCK_FLAGS) $^
