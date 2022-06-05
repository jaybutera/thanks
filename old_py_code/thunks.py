import sys
import subprocess
from cid import make_cid
import json
from base64 import b64encode, b64decode

# Yay global
index_fname = './index.json'

def save_index(data):
    with open(index_fname, 'w') as f:
        json.dump(data, f)

def get_index():
    with open(index_fname, 'r') as f:
        data = f.read()
        if data == '':
            return {}
        else:
            return json.loads(data)

def parse_notes(filename):
    with open(filename, 'r') as f:
        thunks = f.read().split('\n\n')

    return thunks

# Make a thesis from a list of thunk hashes and saves it
def new_thesis(name, ref_hashes):
    thesis = { 'name' : name }
    ref_cids = [str(make_cid(h).encode())[2:-1] for h in ref_hashes]
    thesis['refs'] = [{ '/' : cid } for cid in ref_cids]

    return thesis

# Make a thunk out of some text and save it
def new_thunk(text, refs=[]):
    text = str(b64encode(text.encode('utf-8')))[2:-1]
    return { 'text' : text, 'refs' : refs }

def save_ipld(dag):
    thesis = json.dumps(dag)
    cmd = 'echo "' + thesis.replace('"', '\\"') + '" | ipfs dag put --pin=true'
    res = subprocess.run(cmd, text=True, capture_output=True, shell=True)

    if res.stderr:
        print(res)

    thesis_hash = res.stdout
    return thesis_hash.replace('\n', '')

def cat_thunk(fhash):
    cmd = 'ipfs dag get ' + fhash + '/text'
    res = subprocess.run(cmd, text=True, capture_output=True, shell=True)
    text = b64decode(res.stdout).decode('utf-8')
    return text

def get_thesis(fhash):
    cmd = 'ipfs dag get ' + fhash
    res = subprocess.run(cmd, text=True, capture_output=True, shell=True)
    thunk = json.loads(res.stdout)
    return thunk

def get_thunk(fhash):
    cmd = 'ipfs dag get ' + fhash
    res = subprocess.run(cmd, text=True, capture_output=True, shell=True)

    thunk = json.loads(res.stdout)
    thunk['text'] = b64decode(thunk['text'])
    return thunk


if __name__ == '__main__':
    if len(sys.argv) > 1:
        filename = sys.argv[1]
    else:
        sys.exit('specify a file')

    thesis_name  = filename.split('.')[0].split('/')[-1]
    notes        = parse_notes(filename)
    thunks       = [new_thunk(text) for text in notes]
    thunk_hashes = [save_ipld(thunk) for thunk in thunks]
    thesis       = new_thesis(thesis_name, thunk_hashes)
    thesis_hash  = save_ipld(thesis)

    try:
        open(index_fname, 'r')
    except:
        open(index_fname, 'w')
    #save_index(index_fname, ) # Make sure file exists

    index = get_index()
    index[thesis['name']] = thesis_hash
    save_index(index)

    print(thesis_hash)
