import os
import json
import time

from flask import Flask
from pathlib import Path
import joblib

from google.cloud import storage

GOOGLE_CLOUD_PROJECT = os.environ.get("GOOGLE_CLOUD_PROJECT", "local")
BUCKET_NAME = os.environ.get("GOOGLE_CLOUD_PROJECT", "kz-rec-sys-dev")
TMP_MODEL_DIR = Path(os.environ.get("TMP_MODEL_DIR", "/tmp"))

if GOOGLE_CLOUD_PROJECT == "local":
    service_account_json = str(Path.home() / '.gcp/kz-rec-sys-dev.json')
    storage_client = storage.Client.from_service_account_json(service_account_json)
else:
    storage_client = storage.Client()

bucket = storage_client.get_bucket(BUCKET_NAME)

app = Flask(__name__)

models = {}

organization_ids = [16655]
filenames = ["model", "uuid_cat_id_map", "show_cat_id_map", "user_items"]

for organization_id in organization_ids:
    models[organization_id] = {}
    organization_tmp_dir = TMP_MODEL_DIR / str(organization_id)
    if not organization_tmp_dir.is_dir():
        organization_tmp_dir.mkdir(parents=True)

    # ダウンロード
    for filename in filenames:
        gcs_path = f"{organization_id}/{filename}.pickle"
        local_path = organization_tmp_dir / f"{filename}.pickle"
        if not local_path.exists():
            bucket.blob(gcs_path).download_to_filename(local_path)

    # joblib load
    for filename in filenames:
        local_path = organization_tmp_dir / f"{filename}.pickle"
        print(local_path)
        for _ in range(100):
            try:
                models[organization_id][filename] = joblib.load(open(local_path), "rb")
            except:
                print("error")
                time.sleep(1)
            else:
                print("done")
                break

@app.route('/')
def hello_world():
    target = os.environ.get('TARGET', 'World')
    return 'Hello {}!\n'.format(target)

@app.route('/rec/')
def rec():
    organization_id = 16655
    uuid = '0000019a-f4cd-9a19-6445-58fc330c76db'
    model = models[organization_id]['model']
    user_items = models[organization_id]['user_items']
    uuid_cat_id_map = models[organization_id]['uuid_cat_id_map']
    show_cat_id_map = models[organization_id]['show_cat_id_map']
    recommendations = model.recommend(uuid_cat_id_map[uuid], user_items, N=30, filter_already_liked_items=True)
    rec_fmts = []

    for show_cat_code, score in recommendations:
        show_id = show_cat_id_map[show_cat_code]

        rec_fmts.append(
            {"item_id": show_id, "score": float(score)}
        )

    return json.dumps(rec_fmts)

if __name__ == "__main__":
    app.run(host='0.0.0.0', port=8080, debug=True)