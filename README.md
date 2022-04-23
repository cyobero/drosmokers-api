# drosmokers-api
Drosmokers is an app that allows users to keep track of the different strains they've smoked.

## Resources

### Strains
------------------------------------------------------------------------
- `GET /strains/`         Retrieve a list of all `Strain`s
- `GET /strains/{id}`     Return a `Strain` with `{id``}`
- `POST /strains`         Create a new `Strain` by passing
                        a JSON object. Ex:
                        `curl -X POST\
                        -H "Content-Type: application/json"
                        -d '{"name": "Foo", "species": "Indica"}'`

### Batches
