import requests
from googleapiclient import discovery
import json

def hello(request, context):
    url = "https://hooks.slack.com/services/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"

    print("hello cloud functions")
    resp = requests.post(url, json={"text": "hello from cloudfunctions"})
    return 'Hello World!'

def test(request, context):
    print("test cloud functions")
    return 'test'

def list_compute(request):
    compute = discovery.build('compute', 'v1')
    project = "ikedaosushi"
    zone = "asia-northeast1-b"
    result = compute.instances().list(project=project, zone=zone).execute()
    url = "https://hooks.slack.com/services/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"

    print("hello cloud functions")
    resp = requests.post(url, json={"text": "hello from cloudfunctions"})
    return json.dumps(result, indent=4)