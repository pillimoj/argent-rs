IMAGE := eu.gcr.io/grimsborn/argent
REVISION := $(shell git describe --always --dirty='-dirty')
BRANCH := $(shell git symbolic-ref --short HEAD)
TAG := $(subst /,-,$(BRANCH))-$(REVISION)
IMAGE_TAG := ${IMAGE}:${TAG}
DEV_TAG := ${IMAGE}:dev

# ================= SERVICE ==================
database:
	docker-compose up -d	

buildlocalimage:
	docker build --tag=${DEV_TAG} .

buildproductionimage:
	docker build --tag=${IMAGE_TAG} .

rundocker: buildlocalimage
	docker run -it \
	--env-file .env \
	--network argent_network \
	-p 8008:8008 \
	${DEV_TAG}

deploy: buildproductionimage
	docker push ${IMAGE_TAG}
	gcloud run deploy argent --image=${IMAGE_TAG} --platform=managed --region=europe-west1
