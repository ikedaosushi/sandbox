# synhub
synonyms-github

Search synonyms and how many the word used in GitHub


## Usage

```
$ synhub [options...] [keyword ...]
```

### Example

Output markdown format.

```
$ go run main.go find
| RANK | KEYWORD  |    TOTAL    |
|------|----------|-------------|
|    1 | search   | 173,337,669 |
|    2 | look     | 105,246,523 |
|    3 | seek     |  43,484,027 |
|    4 | research |  33,787,132 |
|    5 | look_for |     122,622 |
```