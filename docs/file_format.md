## What is a thesis file

Thesis files are a way of building knowledge graphs from a file spec. Knowledge
graphs are built from a thesis file in a declarative way. Meaning if you change
remove/change a thunk it will rebuild all objects that reference it too.

The thanks compiler parses a thesis file and loads all knowledge graph
objects to ipfs. It also produces a json file called `index.json` which maps
titles of theses to their ipfs address.

The knowledge graph is a data structure which is independent of the operating
system notion of files. Thesis files are just a descriptor to submit "patches"
of updates to a knowledge graph. Two thesis files may both reference the same
thesis, or theses in eachother's files, or theses that just exist on ipfs and
not in a file. Thesis files are just a convenient way to build knowledge graphs
in your regular text editor, and to share knowledge graph "patches" as files.
But the actual knowledge graphs exist on the ipfs abstraction, not the unix
file abstraction.

## Thesis File Format

### Thesis References

At the top of a file you can list the theses you reference within the file
(like a list of dependencies). Each reference is assigned an alias with the `@`
symbol. Theses are referenced by ipfs address.

Aliases are often best as single characters for ease of use. If you want to
annotate what a reference points to, use a comment with the `#`.

```
# Sovereign Individual Notes
@ s
! bafyreidn4r2qo26ykzeuxqfnaqcdzvfyx7ssuivogb5apduq6q7ryj3smm

# Ocalan Notes
@ o
! bafyreidn4r2qo26ykzeuxqfnaqcdzvfyx7ssuivogb5apduq6q7ryj3saa
```

### Theses

Any number of theses (except 0) can be in a file. Theses are delimited by a
title, where inside are the thunks of that thesis.

The title is of the form:
```
my_thesis
---
```

### Thunks

A thunk is a block of text followed by an empty line or end of file. Thunks
can reference other thunks. To do this you can add a references line above
the text block, starting with `*`.

The reference format is the index number of a thunk in a thesis, followed by an
alias of the thesis referenced. The alias must be defined at the top of the
file in the thesis references.

```
* 21s 3o
The printing press democratized novel thought. Not just for those that could
read and write but also by enabling ideas to spread easier.

The printing press was crucial for the new class of merchants by providing
access to information about foreign market opportunities.
```
