# drosmokers-api
Drosmokers is an app that allows users to keep track of the different strains they've smoked.

## Resources

### Strains
------------------------------------------------------------------------
- `GET /strains/`   Returns a list of all `Strain`s.
  Example:
    request: `curl localhost:8008/strains`
    response: `[{"name":"OG Kush", "species": "Indica"},
                {"name": "Reggie Kush", "species": "Sativa"}]`
