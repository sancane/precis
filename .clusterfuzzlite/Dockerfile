FROM gcr.io/oss-fuzz-base/base-builder-rust

# Copy the project source code
COPY . $SRC/precis
WORKDIR $SRC/precis

# Build script will be executed by ClusterFuzzLite
COPY .clusterfuzzlite/build.sh $SRC/
