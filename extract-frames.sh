#!/bin/bash

set -ex

FRAMES_DIR=./frames
FRAMES_SMALL=./frames-small

echo "Downloading video file..."
FILENAME=$(yt-dlp -f bv tPYUdjptMpk -o 'bad-apple.%(ext)s' -O filename)
yt-dlp -f bv tPYUdjptMpk -o 'bad-apple.%(ext)s'
mkdir -p $FRAMES_DIR $FRAMES_SMALL
echo "Extracting frames..."
ffmpeg -i "$FILENAME" -start_number 0 $FRAMES_DIR/%08d.qoi

pushd $FRAMES_DIR
echo "Cropping frames..."
fd | xargs -n1 -I{} convert {} -crop 500x500+110+76 ../$FRAMES_SMALL/{}
popd

echo "Frames reside in $FRAMES_SMALL"
