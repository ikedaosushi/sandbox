helm upgrade --install jhub jupyterhub/jupyterhub \
  --namespace jhub \
  --version=0.8.2 \
  --values config.yaml