import requests

payload={"text": "hello from python script"}
url = "https://hooks.slack.com/services/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"
requests.post(url, json=payload)