PKG_NAME = LoaderPkg
TARGET = x86_64-unknown-uefi
CONFIG = release
RUN_QEMU_DIR = ../tools/devenv/run_qemu.sh
DISK_IMG = disk.img
MOUNT_DIR = mnt

CARGO = cargo
OPTION = +nightly build -Zbuild-std=core

EFI_DIR = target/$(TARGET)/$(CONFIG)/$(PKG_NAME).efi
BUILD = $(CARGO) $(OPTION) --target $(TARGET) --$(CONFIG)
RUN = ./$(RUN_QEMU_DIR) $(EFI_DIR)

.PHONY: all
all: clean build run

.PHONY: build
build:
	$(BUILD)

.PHONY: run
run: 
	$(RUN)

.PHONY: mem
mem: 
	$(BUILD)
	-sudo mount -o loop $(DISK_IMG) $(MOUNT_DIR)
	-ls mnt

.PHONY: clean
clean:
	-sudo umount mnt
	-rm -rf $(MOUNT_DIR)
	-rm -rf target
	-rm -rf $(DISK_IMG)

