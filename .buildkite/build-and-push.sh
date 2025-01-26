#!/bin/bash
set -e

PROJECT_ID="projects-showcase-449019"
IMAGE_NAME="projects-showcase-api"
IMAGE_TAG="us-central1-docker.pkg.dev/projects-showcase-449019/projects-showcase-api-repo/projects-showcase-api:latest"

echo "Building Docker image"
docker build -t $IMAGE_TAG .

echo "Pushing Docker image to GCR"
docker push $IMAGE_TAG

echo "Build & push completed"
