## Run Project

Use `smdk` to build project

```
smdk build
```

Rust `test` to convert an xml file into json

```
smdk test --file data/uscourts-gov.xml --raw | tail -n +3 | jq
```