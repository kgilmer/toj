# Trees of JSON

Model projections from hierarchical parents in JSON

`toj` is a command-line tool that merges JSON documents in an assumed directory structure, starting from a parent document down to any number of specializations in subdirectories.  

Combining directory traversal with JSON merge logic, this simple tool allows for patterns like object-orientated inheritance, but for data. This may be useful in reducing redundancy in cases where a large number of model instances can be realized from a base general model in addition to one or more successive layers of specialization.

The merge strategy is to create the model from the top-most parent.  Children may Create (add new key/values), Update (specify new values for existing keys), and Delete (set a key's value to `null`) from parent data.

## Usage

```console
Usage: toj [OPTIONS] <LEAF_FILE_PATH>

Arguments:
  <LEAF_FILE_PATH>  

Options:
  -v, --verbose     See details
  -s, --skip-empty  Traverse to root of file system
  -h, --help        Print help
  -V, --version     Print version
```

## Example

### `animal-kingdom`

Given the following model hierarchy that describes some animals as JSON objects, in directories that specify habitats:

```text
j$ tree examples/animal-kingdom/
examples/animal-kingdom/
├── animal-model.json
├── forest
│   ├── alpine
│   │   └── animal-model.json
│   └── animal-model.json
├── ocean
│   └── animal-model.json
└── zoo
    └── animal-model.json
```

The base model that describes the superset of animals is in the top-most parent. 

`examples/animal-kingdom/animal-model.json`:

```json
{
    "animals" : {
        "deer" : {
            "avg-weight-kg" : 56,
            "leg-count": 4,
            "diet" : "herbivore"
        },
        "monkey" : {
            "avg-weight-kg" : 16,
            "leg-count": 2,
            "diet" : "herbivore"
        },
        "piranha" : {
            "avg-weight-kg" : 1,
            "leg-count": 0,
            "diet" : "carnivore"
        }      
    }
}
```


We can determine the concrete model projection of animals in any given habitat by merging the subset of models from the top-most parent (abstract) to the desired leaf (concrete):

```text
$ toj examples/animal-kingdom/forest/animal-model.json
{
  "animals": {
    "deer": {
      "avg-weight-kg": 56,
      "diet": "herbivore",
      "leg-count": 4
    },
    "monkey": {
      "avg-weight-kg": 16,
      "diet": "herbivore",
      "leg-count": 2
    }
  }
}
```

```text
$ toj examples/animal-kingdom/zoo/animal-model.json 
{
  "animals": {
    "monkey": {
      "avg-weight-kg": 20,
      "diet": "herbivore",
      "leg-count": 2
    },
    "piranha": {
      "avg-weight-kg": 2,
      "diet": "carnivore",
      "leg-count": 0
    }
  }
}
```

## Alternatives

### `jq`

The json tool `jq` can perform the JSON [object merging](https://stackoverflow.com/questions/19529688/how-to-merge-2-json-objects-from-2-files-using-jq), combined with shell scripts to perform directory traversal.
