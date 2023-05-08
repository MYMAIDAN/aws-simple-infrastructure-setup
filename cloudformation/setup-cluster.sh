#!/usr/bin/bash

# Usefull links
# - https://towardsdatascience.com/kubernetes-application-deployment-with-aws-eks-and-ecr-4600e11b2d3c
# - https://docs.aws.amazon.com/eks/latest/userguide/getting-started-console.html
# - https://medium.com/@danieltse/pull-the-docker-image-from-aws-ecr-in-kubernetes-dc7280d74904

EKS_CLUSTER_NAME=my-cluster
EKS_ROLE_NAME=myAmazonEKSClusterRole
EKS_NODE_ROLE_NAME=myAmazonEKSNodeRole
# Create VPC by using CloudFormation

## Validate Network Template
aws cloudformation validate-template --template-body file://network-stack.json

## Deploy Template
aws cloudformation deploy --template-file "network-stack.json" --stack-name "simple-web-app-stack"

# Create Role for ESK Cluster
aws iam create-role \
    --role-name ${EKS_ROLE_NAME} \
    --assume-role-policy-document file://"eks-cluster-role-trust-policy.json"

aws iam attach-role-policy \
  --policy-arn arn:aws:iam::aws:policy/AmazonEKSClusterPolicy \
  --role-name ${EKS_ROLE_NAME}


# After this need to create cluster by using WebConsole


# Switch local kubectl to the config
# aws eks update-kubeconfig --region us-east-1 --name ${EKS_CLUSTER_NAME}


# Create Role for EKS Nodes
aws iam create-role \
  --role-name ${EKS_NODE_ROLE_NAME} \
  --assume-role-policy-document file://"node-role-trust-policy.json"

# Attach the required managed IAM policies to the role.
aws iam attach-role-policy \
  --policy-arn arn:aws:iam::aws:policy/AmazonEKSWorkerNodePolicy \
  --role-name ${EKS_NODE_ROLE_NAME}
aws iam attach-role-policy \
  --policy-arn arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryReadOnly \
  --role-name ${EKS_NODE_ROLE_NAME}

aws iam attach-role-policy \
  --policy-arn arn:aws:iam::aws:policy/AmazonEKS_CNI_Policy \
  --role-name ${EKS_NODE_ROLE_NAME}
