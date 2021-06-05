PKG_NAME = LoaderPkg
TARGET = x86_64-unknown-uefi
CONFIG = release
RUN_QEMU_DIR = ../tools/devenv/run_qemu.sh

CARGO = cargo
OPTION = +nightly build -Zbuild-std=core

EFI_DIR = target/$(TARGET)/$(CONFIG)/$(PKG_NAME).efi
BUILD = $(CARGO) $(OPTION) --target $(TARGET) --$(CONFIG)
RUN = ./$(RUN_QEMU_DIR) $(EFI_DIR)

TARGET3 = ./../tools/devenv/run_qemu.sh target/x86_64-unknown-uefi/release/rust-uefi.efi

.PHONY: all
all: clean build run

.PHONY: build
build:
	$(BUILD)

.PHONY: run
run: 
	$(RUN)

.PHONY: clean
clean:
	-rm -rf mnt
	-rm -rf target
	-rm -rf disk.img 

