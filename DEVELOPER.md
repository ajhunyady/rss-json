## Run Project

Build & test. The code will convert an .xml file into a json represetation.

```
smdk build
```

```
smdk test --file data/uscourts-gov.xml --raw | tail -n +3 | jq
```