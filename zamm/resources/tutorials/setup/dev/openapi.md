# Setting up OpenAPI for type-safety

Make sure to first follow the instructions for [setting up Java](./java.md) before continuing. Then install the CLI:

```bash
$ yarn add -W -D @openapitools/openapi-generator-cli
```

For this, note that we are currently using the generator version 6.6.0:

```bash
$ yarn openapi-generator-cli version
yarn run v1.22.19
$ /root/zamm/node_modules/.bin/openapi-generator-cli version
6.6.0
Done in 1.18s.
```

Define a file `src-python/openapi.yaml` such as this:

```yaml
openapi: 3.1.0
info:
  description: >-
    This is the OpenAPI definition of the Python API, to allow for type-safe interactions between Python and Rust.
  version: 0.0.0
  title: Python API
components:
  schemas:
    User:
      type: object
      properties:
        id:
          type: integer
        name:
          type: string
```

We are only interested in model generation for ZAMM, to provide type safety between interfaces. [This issue](https://github.com/OpenAPITools/openapi-generator/issues/12754) proposes different ways to do that. We follow one of them by running

```bash
$ yarn openapi-generator-cli generate -i src-python/openapi.yaml -g python-fastapi -o src-python/openapi-gen --global-property models
```

If you get an error such as this:

```bash
[main] WARN  o.o.codegen.DefaultCodegen - Generation using 3.1.0 specs is in development and is not officially supported yet. If you would like to expedite development, please consider woking on the open issues in the 3.1.0 project: https://github.com/orgs/OpenAPITools/projects/4/views/1 and reach out to our team on Slack at https://join.slack.com/t/openapi-generator/shared_invite/zt-12jxxd7p2-XUeQM~4pzsU9x~eGLQqX2g
[main] INFO  o.o.codegen.DefaultGenerator - Generating with dryRun=false
[main] INFO  o.o.c.ignore.CodegenIgnoreProcessor - No .openapi-generator-ignore file found.
...
Exception in thread "main" java.lang.RuntimeException: Could not generate api file for 'default'
        at org.openapitools.codegen.DefaultGenerator.generateApis(DefaultGenerator.java:717)
        ...
        at org.openapitools.codegen.OpenAPIGenerator.main(OpenAPIGenerator.java:66)
Caused by: com.github.jknack.handlebars.HandlebarsException: endpoint.handlebars:3:3: java.lang.reflect.InaccessibleObjectException: Unable to make field transient java.util.HashMap$Node[] java.util.HashMap.table accessible: module java.base does not "opens java.util" to unnamed module @edc0eb6
    endpoint.handlebars:3:3
        at java.base/java.lang.reflect.AccessibleObject.throwInaccessibleObjectException(AccessibleObject.java:391)
        ...
        at org.openapitools.codegen.DefaultGenerator.generateApis(DefaultGenerator.java:605)
        ... 4 more
Caused by: java.lang.reflect.InaccessibleObjectException: Unable to make field transient java.util.HashMap$Node[] java.util.HashMap.table accessible: module java.base does not "opens java.util" to unnamed module @edc0eb6
        ... 31 more
error Command failed with exit code 1.
info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command.
```

follow the recommendations on [this GitHub issue](https://github.com/OpenAPITools/openapi-generator/issues/13684) to either downgrade Java with `asdf` or else

```bash
$ export JAVA_OPTS="--add-opens java.base/java.util=ALL-UNNAMED --add-opens java.base/java.lang=ALL-UNNAMED"
```

before running the command again.

We now look at the generated file at `src-python/openapi-gen/openapi_client/model/user.py`:

```python
...

class User(
    schemas.AnyTypeSchema,
):
    ...
    

    def __new__(
        cls,
        *_args: typing.Union[dict, frozendict.frozendict, str, date, datetime, uuid.UUID, int, float, decimal.Decimal, bool, None, list, tuple, bytes, io.FileIO, io.BufferedReader, ],
        id: typing.Union[MetaOapg.properties.id, dict, frozendict.frozendict, str, date, datetime, uuid.UUID, int, float, decimal.Decimal, bool, None, list, tuple, bytes, io.FileIO, io.BufferedReader, schemas.Unset] = schemas.unset,
        name: typing.Union[MetaOapg.properties.name, dict, frozendict.frozendict, str, date, datetime, uuid.UUID, int, float, decimal.Decimal, bool, None, list, tuple, bytes, io.FileIO, io.BufferedReader, schemas.Unset] = schemas.unset,
        _configuration: typing.Optional[schemas.Configuration] = None,
        **kwargs: typing.Union[schemas.AnyTypeSchema, dict, frozendict.frozendict, str, date, datetime, uuid.UUID, int, float, decimal.Decimal, None, list, tuple, bytes],
    ) -> 'User':
        return super().__new__(
            cls,
            *_args,
            id=id,
            name=name,
            _configuration=_configuration,
            **kwargs,
        )

```

There does not appear to be any typing involved. We look at using the FastAPI generator:

```bash
$ yarn openapi-generator-cli generate -i src-python/openapi.yaml -g python-fastapi -o src-python/openapi-gen --global-property models
```

The path for the file generated here is different. We look at `src-python/openapi-gen/src/openapi_server/models/user.py` and see:

```python
# coding: utf-8

from __future__ import annotations
from datetime import date, datetime  # noqa: F401

import re  # noqa: F401
from typing import Any, Dict, List, Optional  # noqa: F401

from pydantic import AnyUrl, BaseModel, EmailStr, Field, validator  # noqa: F401


class User(BaseModel):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.

    User - a model defined in OpenAPI

        id: The id of this User [Optional].
        name: The name of this User [Optional].
    """

    id: Optional[object] = Field(alias="id", default=None)
    name: Optional[object] = Field(alias="name", default=None)

User.update_forward_refs()

```

While there's still no typing, this is at least much cleaner than the default Python generator. If we edit `src-python/openapi.yaml` to add a path and to downgrade the `openapi` version due to the warning

> [main] WARN  o.o.codegen.DefaultCodegen - Generation using 3.1.0 specs is in development and is not officially supported yet. If you would like to expedite development, please consider woking on the open issues in the 3.1.0 project: https://github.com/orgs/OpenAPITools/projects/4/views/1 and reach out to our team on Slack at https://join.slack.com/t/openapi-generator/shared_invite/zt-12jxxd7p2-XUeQM~4pzsU9x~eGLQqX2g

then we have:

```yaml
openapi: 3.0.0
info:
  description: >-
    This is the OpenAPI definition of the Python API, to allow for type-safe interactions between Python and Rust.
  version: 0.0.0
  title: Python API
paths:
  /users:
    get:
      summary: Get all users
      operationId: get_users
      responses:
        '200':
          description: OK
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/User'
components:
  schemas:
    User:
      type: object
      properties:
        id:
          type: integer
        name:
          type: string

```

If we rerun the command now:

```bash
$ yarn openapi-generator-cli generate -i src-python/openapi.yaml -g python-fastapi -o src-python/openapi-gen --global-property models
```

we see that the file finally has types:

```python
# coding: utf-8

from __future__ import annotations
from datetime import date, datetime  # noqa: F401

import re  # noqa: F401
from typing import Any, Dict, List, Optional  # noqa: F401

from pydantic import AnyUrl, BaseModel, EmailStr, Field, validator  # noqa: F401


class User(BaseModel):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.

    User - a model defined in OpenAPI

        id: The id of this User [Optional].
        name: The name of this User [Optional].
    """

    id: Optional[int] = Field(alias="id", default=None)
    name: Optional[str] = Field(alias="name", default=None)

User.update_forward_refs()

```

### Rust

We run this for Rust as well:

```bash
$ yarn openapi-generator-cli generate -i src-python/openapi.yaml -g rust -o src-tauri/openapi-gen --global-property models
```

The generated Rust file at `src-tauri/openapi-gen/src/models/user.rs` has types:

```rust
/*
 * Python API
 *
 * This is the OpenAPI definition of the Python API, to allow for type-safe interactions between Python and Rust.
 *
 * The version of the OpenAPI document: 0.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl User {
    pub fn new() -> User {
        User {
            id: None,
            name: None,
        }
    }
}



```

## Python package

If you already have Python set up, you can use the Python package for OpenAPI for easy setup:

```bash
$ poetry add --group dev openapi-generator-cli
```

However, as of 2023-08-22, the Python package does not support YAML inputs.

```bash
$ poetry run openapi-generator generate -i openapi.yaml -g python
[main] ERROR i.s.parser.SwaggerCompatConverter - failed to read resource listing
com.fasterxml.jackson.core.JsonParseException: Unrecognized token 'openapi': was expecting (JSON String, Number, Array, Object or token 'null', 'true' or 'false')
 at [Source: (String)"openapi: 3.1.0
info:
  description: >-
    This is the OpenAPI definition of the Python API, to allow for type-safe interactions between Python and Rust.
...
        at org.openapitools.codegen.OpenAPIGenerator.main(OpenAPIGenerator.java:61)
[main] ERROR i.s.parser.SwaggerCompatConverter - failed to read resource listing
com.fasterxml.jackson.core.JsonParseException: Unrecognized token 'openapi': was expecting (JSON String, Number, Array, Object or token 'null', 'true' or 'false')
 at [Source: (String)"openapi: 3.1.0
...
        at org.openapitools.codegen.OpenAPIGenerator.main(OpenAPIGenerator.java:61)
Exception in thread "main" java.lang.NullPointerException: Cannot invoke "java.util.Collection.size()" because "c" is null
        at java.base/java.util.HashSet.<init>(HashSet.java:120)
        at org.openapitools.codegen.config.CodegenConfigurator.toContext(CodegenConfigurator.java:464)
        at org.openapitools.codegen.config.CodegenConfigurator.toClientOptInput(CodegenConfigurator.java:507)
        at org.openapitools.codegen.cmd.Generate.execute(Generate.java:423)
        at org.openapitools.codegen.cmd.OpenApiGeneratorCommand.run(OpenApiGeneratorCommand.java:32)
        at org.openapitools.codegen.OpenAPIGenerator.main(OpenAPIGenerator.java:61)
```

This is because it is running an older version:

```bash
$ poetry run openapi-generator version
4.3.1
```
