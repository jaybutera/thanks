import sys
import subprocess
import json
#from thunks import get_thesis, get_thunk, new_thunk, save_ipld, cat_thunk, get_index, new_thesis
from thunks import *

def parse_ref_range(ref_range):
    refs = ref_range.split(',')
    final_refs = []

    for i, r in enumerate(refs):
        ends = r.split('-')
        if len(ends) == 2:
            final_refs.extend([x for x in range(int(ends[0]), int(ends[1])+1)])
        else:
            final_refs.append(int(r))

    return final_refs

if __name__ == "__main__":
    if len(sys.argv) > 1:
        thesis_name = sys.argv[1]
    else:
        #sys.exit('specify an ipfs file')
        sys.exit('specify the name of a thesis')

    index = get_index()

    print("[j] Next")
    print("[k] Previous")
    print("[e] Edit")
    print("[c] Comment")
    print("[n] New Thesis")
    print("[l] List Theses")
    print("[x] Quit")

    fhash = index[thesis_name]
    thesis = get_thesis(fhash)
    thunk_hashes = [t['/'] for t in thesis['refs']]

    # Just print the content if no links
    if len(thunk_hashes) == 0:
        sys.exit(thesis['text'])

    cur_idx = 0

    print('')
    print('[' + str(cur_idx) + ']')
    print(cat_thunk(thunk_hashes[cur_idx]))

    while action := input("> "):
        match action:
            case 'j':
                cur_idx += 1
            case 'k':
                cur_idx -= 1
            case 'l':
                index = get_index()
                print(', '.join(index.keys()))
                continue
            case 'n':
                name = input('title: ')
                thesis = new_thesis(name, [])
                thesis_hash = save_ipld(thesis)
                index[name] = thesis_hash
                save_index(index)
            case 'c':
                ref_idxs     = parse_ref_range(input('on: '))
                thunk_refs   = [thunk_hashes[i] for i in ref_idxs]

                text         = input('comment: ')
                comment      = new_thunk(text, thunk_refs)
                comment_hash = save_ipld(comment)
                #print(comment_hash)

                thesis_names = input('which theses to add this to?\n').split(',')
                for name in thesis_names:
                    fhash   = index[name]
                    thesis  = get_thesis(fhash)
                    print(comment_hash)
                    updated = new_thesis(
                            name,
                            thunk_hashes + [comment_hash])
                    thesis_hash = save_ipld(updated)
                    # Point index to updated thesis
                    index[name] = thesis_hash

                save_index(index)

        print('')
        print('[' + str(cur_idx) + ']')
        print(cat_thunk(thunk_hashes[cur_idx]))
