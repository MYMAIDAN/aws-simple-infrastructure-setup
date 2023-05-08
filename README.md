# aws-simple-infrastucture-setup

# AWS cli cofiguration

Setup Configuration
```bash
aws configure
```

Check configuration
```bash
aws sts get-caller-identity
```


# Setup AWS ECR as a docker image registry

Login to the docker 
```bash
aws ecr get-login-password --region region | docker login --username AWS --password-stdin aws_account_ID.dkr.ecr.region.amazonaws.com
```

Create registry
```bash
asw ecr create-repository \
--repository-name name \
--image-scanning-configuration scanOnPush=true \
--region region
```

List of images
```bash
docker images
```

Tag image
```bash
docker tag image_name:version aws_acocunt_id.dkr.ecr.region.amazonaws.com/repository_name
```

Push image
```bash
docker push aws_account_id.dkr.ecr.region.amazonaws.com/repository_name
```

