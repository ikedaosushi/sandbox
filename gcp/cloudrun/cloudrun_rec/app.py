import requests
import time

elapsed = 0

url = "https://hooks.slack.com/services/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"
resp = requests.post(url, json={"text": "hello from cloudrun"})
print("hello cloudrun")

for m in range(60): 
    print("now:", elapsed) 
    time.sleep(60) 
    elapsed += 60 

resp = requests.post(url, json={"text": "60分たった"})