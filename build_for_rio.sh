rustup target add arm-unknown-linux-gnueabi

export PYO3_CROSS_PYTHON_VERSION=3.10
export PYO3_CROSS_LIB_DIR="RIO_ROOT/usr/lib"

maturin build --target=arm-unknown-linux-gnueabi