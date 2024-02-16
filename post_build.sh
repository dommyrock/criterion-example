#!/bin/bash

# Run the build
cargo build --release

# Check if the build was successful
if [ $? -eq 0 ]; then
   echo "Build successful, running post-build commands..."
   pwd

   # If the flamegraphs directory exists, delete its contents
   if [ -d "flamegraphs" ]; then
      rm -rf flamegraphs/*
   fi
   mkdir -p flamegraphs

   cd src/bin

   for file in *; do
      file_name="${file%.*}" # Trim the .rs extension
      # cargo flamegraph -o ../../flamegraphs/$file_name.svg --bin $file_name

      #windows is not quite finding DTrace " DTrace device not available on system"
      cargo flamegraph --bin=$file_name -o ../../flamegraphs/$file_name.svg   
   done

   # Change back to the root directory
   cd ../..

else
   echo "Build failed, not running post-build commands."
fi
