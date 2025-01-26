#!/bin/bash
set -e

PROJECT_ID="projects-showcase-449019"
IMAGE_NAME="projects-showcase-api"
IMAGE_TAG="us-central1-docker.pkg.dev/projects-showcase-449019/projects-showcase-api-repo/projects-showcase-api:latest"

echo "Updating Kubernetes deployment"

kubectl set image deployment/rocket-app rocket-app=$IMAGE_TAG --record

echo "Deployment updated in GKE"
