
## run openapi generator

```shell
docker run --rm -it -v "${PWD}:/local" ghcr.io/beryju/openapi-generator generate -i /local/openapi.json -g typescript-fetch -o /local/openapi --additional-properties=ngVersion=6.1.7,npmName=api-client,supportsES6=true,npmVersion=1.0.0,withInterfaces=true
```
