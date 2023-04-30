import json
import os

# list files on folder target/sqlx
def list_files():
    files = []
    for file in os.listdir("target/sqlx"):
        if file.endswith(".json"):
            files.append(file)
    return files

# read file
def read_file(file):
    with open("target/sqlx/" + file) as json_file:
        data = json.load(json_file)
        return data

records = {}
for file in list_files():
    data = read_file(file)
    _hash = data.pop('hash')
    records[_hash] = data


with open("sqlx-data.json") as json_file:
    sqlx_data = json.load(json_file)
    sqlx_data.update(records)

with open("sqlx-data.json", "w") as json_file:
    json.dump(sqlx_data, json_file, indent=4)


