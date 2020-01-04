import os

from flask import Flask
from pathlib import Path
import joblib
from google.cloud import storage

service_account_json = str(Path.home()/".gcp/kz-rec-sys-dev.json")
storage_client = storage.Client.from_service_account_json(service_account_json)

bucket_name = 'kz-rec-sys-dev'

bucket = storage_client.create_bucket(bucket_name)

app = Flask(__name__)

model_path = "model.pickle"
uuid_cat_id_map_path = "uuid_cat_id_map.pickle"
show_cat_id_map_path = "show_cat_id_map.pickle"
user_items_path = "user_items.pickle"

@app.route('/')
def hello_world():
    target = os.environ.get('TARGET', 'World')
    return 'Hello {}!\n'.format(target)

if __name__ == "__main__":
    app.run(debug=True, host='0.0.0.0', port=int(os.environ.get('PORT', 8080)))