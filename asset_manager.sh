#!/bin/bash

if [ "$1" = "get-free" ]; then
    aws s3 cp s3://thetawave-assets/free_assets/planets/ ./assets/models/planets/ --recursive --no-sign-request
    aws s3 cp s3://thetawave-assets/free_assets/backgrounds/ ./assets/texture/backgrounds/ --recursive --no-sign-request
elif [ "$1" = "get-premium" ]; then
    aws s3 cp s3://thetawave-assets/premium_assets/planets/ ./assets/models/planets/ --recursive
    aws s3 cp s3://thetawave-assets/premium_assets/backgrounds/ ./assets/texture/backgrounds/ --recursive
elif [ "$1" = "remove" ]; then
    rm ./assets/models/planets/*
    rm ./assets/texture/backgrounds/*
fi
