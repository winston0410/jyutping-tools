# Script for linking with libtorch on M1 Mac
export LIBTORCH=/opt/homebrew/Cellar/libtorch/1.11.0
export DYLD_LIBRARY_PATH=${LIBTORCH}/lib:$DYLD_LIBRARY_PATH