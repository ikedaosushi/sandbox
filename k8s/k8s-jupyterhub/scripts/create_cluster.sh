eksctl create cluster \
--profile kaizen-trial \ 
--name ikedayu-k8s-jupyterhub \
--version 1.13 \
--nodegroup-name standard-workers \
--node-type t3.medium \
--nodes 3 \
--nodes-min 1 \
--nodes-max 4 \
--node-ami auto