#!/bin/bash
set -euo pipefail

OPTEE_PATH=/home/eric/devel/optee
TARGET=arm-unknown-linux-gnueabihf
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

UUID=`cat uuid.txt`
OPTEE_BIN="$OPTEE_PATH/toolchains/aarch32/bin"

CC="$OPTEE_BIN/arm-linux-gnueabihf-gcc"
LD="$OPTEE_BIN/arm-linux-gnueabihf-ld.bfd"
OBJCOPY="$OPTEE_BIN/arm-linux-gnueabihf-objcopy"
SIGN="$OPTEE_PATH/optee_os/out/arm/export-ta_arm32/scripts/sign.py"

# build the TA library
cargo build --target $TARGET -v --release

# generate the TA header object file
$CC -std=gnu99 -Werror -fdiagnostics-show-option -Wall -Wcast-align -Werror-implicit-function-declaration -Wextra -Wfloat-equal -Wformat-nonliteral -Wformat-security -Wformat=2 -Winit-self -Wmissing-declarations -Wmissing-format-attribute -Wmissing-include-dirs -Wmissing-noreturn -Wmissing-prototypes -Wnested-externs -Wpointer-arith -Wshadow -Wstrict-prototypes -Wswitch-default -Wwrite-strings -Wno-missing-field-initializers -Wno-format-zero-length -Waggregate-return -Wredundant-decls -Wold-style-definition -Wstrict-aliasing=2 -Wundef -pedantic -Wdeclaration-after-statement -Wno-error=cast-align -Os -g3 -fpie -MD -MF ./.user_ta_header.o.d -MT user_ta_header.o -nostdinc -isystem $OPTEE_BIN/../lib/gcc/arm-linux-gnueabihf/8.2.1/include -DCFG_TEE_TA_LOG_LEVEL=4 -DARM32=1 -D__LP32__=1 -DTRACE_LEVEL=4 -I. -I$OPTEE_PATH/optee_os/out/arm/export-ta_arm32/include -c $OPTEE_PATH/optee_os/out/arm/export-ta_arm32/src/user_ta_header.c -o target/$TARGET/release/deps/user_ta_header.o

# link the TA
$LD -pie -T ta.lds -Map=target/$TARGET/release/$UUID.map --sort-section=alignment -L $DIR/target/$TARGET/release/ target/$TARGET/release/deps/user_ta_header.o -L$OPTEE_PATH/optee_os/out/arm-plat-vexpress/export-ta_arm32/lib -rpath-link $OPTEE_PATH/out-br/target/lib --start-group -lmpa -lutils -lutee -lta --end-group -o target/$TARGET/release/$UUID.elf

# strip the TA
$OBJCOPY --strip-unneeded target/$TARGET/release/$UUID.elf target/$TARGET/release/$UUID.stripped.elf

# sign the TA
$SIGN --uuid $UUID --key $OPTEE_PATH/build/../optee_os/out/arm/export-ta_arm32/keys/default_ta.pem --in target/$TARGET/release/$UUID.stripped.elf --out target/$TARGET/release/$UUID.ta

echo "Created signed TA: target/$TARGET/release/$UUID.ta"
