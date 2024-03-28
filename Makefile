# Image and repository variables
IMAGE_NAME := todo-fargate
AWS_ID := XXXXXXXXXXXX
REGION_ID := eu-west-1
REPO_URI := $(AWS_ID).dkr.ecr.$(REGION_ID).amazonaws.com/$(IMAGE_NAME)

.PHONY: build push aws-sam clean

# Build the Docker image locally
build:
	docker build -t $(IMAGE_NAME) .

# Tag and push the image to ECR
push:
	aws ecr get-login-password --region $(REGION_ID) | docker login --username AWS --password-stdin $(AWS_ID).dkr.ecr.$(REGION_ID).amazonaws.com
	docker tag $(IMAGE_NAME) $(REPO_URI):latest
	docker push $(REPO_URI):latest

# Build and Deploy the SAM Deployment (ATTENTION costs are generated, due running Tasks)
aws-sam:
	sam build
	sam validate
	sam deploy

# Remove CloudFormation Components. The Docker Images in ECR you need to remove manually.
clean:
	sam delete