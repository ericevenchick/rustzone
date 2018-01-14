#!/bin/bash
OPTEE_PATH=/home/eric/devel/optee

UUID=`cat uuid.txt`
OPTEE_BIN="$OPTEE_PATH/toolchains/aarch64/bin"

CC="$OPTEE_BIN/aarch64-linux-gnu-gcc"
LD="$OPTEE_BIN/aarch64-linux-gnu-ld.bfd"
OBJCOPY="$OPTEE_BIN/aarch64-linux-gnu-objcopy"
SIGN="$OPTEE_PATH/optee_os/out/arm/export-ta_arm64/scripts/sign.py"

$CC -std=gnu99 -Werror -fdiagnostics-show-option -Wall -Wcast-align -Werror-implicit-function-declaration -Wextra -Wfloat-equal -Wformat-nonliteral -Wformat-security -Wformat=2 -Winit-self -Wmissing-declarations -Wmissing-format-attribute -Wmissing-include-dirs -Wmissing-noreturn -Wmissing-prototypes -Wnested-externs -Wpointer-arith -Wshadow -Wstrict-prototypes -Wswitch-default -Wwrite-strings -Wno-missing-field-initializers -Wno-format-zero-length -Waggregate-return -Wredundant-decls -Wold-style-definition -Wstrict-aliasing=2 -Wundef -pedantic -Wdeclaration-after-statement -Wno-error=cast-align -Os -g3 -fpie -mstrict-align -MD -MF ./.user_ta_header.o.d -MT user_ta_header.o -nostdinc -isystem $OPTEE_BIN/../lib/gcc/aarch64-linux-gnu/6.2.1/include -DCFG_TEE_TA_LOG_LEVEL=4 -DARM64=1 -D__LP64__=1 -DTRACE_LEVEL=4 -I. -I$OPTEE_PATH/optee_os/out/arm/export-ta_arm64/include -c $OPTEE_PATH/optee_os/out/arm/export-ta_arm64/src/user_ta_header.c -o target/aarch64-unknown-linux-gnu/release/deps/user_ta_header.o

$LD -pie -T ta.lds  -Map=target/aarch64-unknown-linux-gnu/release/$UUID.map --sort-section=alignment -L /home/eric/rustzone/ta/target/aarch64-unknown-linux-gnu/release/ target/aarch64-unknown-linux-gnu/release/deps/user_ta_header.o -L$OPTEE_PATH/optee_os/out/arm/export-ta_arm64/lib --start-group -lmpa -lutee -lutils --end-group -lta -o target/aarch64-unknown-linux-gnu/release/$UUID.elf

$OBJCOPY --strip-unneeded target/aarch64-unknown-linux-gnu/release/$UUID.elf target/aarch64-unknown-linux-gnu/release/$UUID.stripped.elf

$SIGN --key $OPTEE_PATH/build/../optee_os/out/arm/export-ta_arm64/keys/default_ta.pem --in target/aarch64-unknown-linux-gnu/release/$UUID.stripped.elf --out target/aarch64-unknown-linux-gnu/release/$UUID.ta
