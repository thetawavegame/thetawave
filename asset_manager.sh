#!/usr/bin/env bash
set -euxo pipefail

if [ "$1" = "get-free" ]; then
    aws s3 cp --profile thetawavedev-p --recursive s3://assets-thetawave/free_assets/models/planets/ ./assets/models/planets/
    aws s3 cp --profile thetawavedev-p --recursive s3://assets-thetawave/free_assets/texture/backgrounds/ ./assets/texture/backgrounds/
    aws s3 cp --profile thetawavedev-p --recursive s3://assets-thetawave/free_assets/sounds ./assets/sounds/
elif [ "$1" = "get-premium" ]; then
    aws s3 cp --profile thetawavedev-p --recursive s3://assets-thetawave/premium_assets/ ./assets/
elif [ "$1" = "remove" ]; then
    rm -rf ./assets/models/planets
    rm -rf ./assets/texture/backgrounds
    rm -rf ./assets/sounds/
fi
