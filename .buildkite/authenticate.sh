#!/bin/bash
set -e

echo "Fetching GCP SA Key for Projects Showcase API from Secret Manager"

gcloud secrets versions access latest --secret=BUILDKITE_GCP_SA_KEY_PROJECTS_SHOWCASE | \
gcloud auth activate-service-account --key-file=-

echo "Auth successful."
