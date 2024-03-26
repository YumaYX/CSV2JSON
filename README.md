# csv2json

convert CSV into JSON

```sh
csv2json [-k] [-n] <CSV File>
```

## Example

### CSV

```csv
1,2,3
a,,true
x,false,9
z,z,z

```

### Output JSON

```json
[
  {
    "1": "a",
    "2": "",
    "3": true
  },
  {
    "1": "x",
    "2": false,
    "3": 9
  },
  {
    "1": "z",
    "2": "z",
    "3": "z"
  }
]
```

### Output JSON with k & n Option

```json
{
  "a": {
    "1": "a",
    "2": null,
    "3": true
  },
  "x": {
    "1": "x",
    "2": false,
    "3": 9
  },
  "z": {
    "1": "z",
    "2": "z",
    "3": "z"
  }
}
```

