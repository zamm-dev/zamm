# PyPI

## Errors

If you get

```
HTTP Error 401: User amosjyng has two factor auth enabled, an API Token or Trusted Publisher must be used to upload in place of password. | b'<html>\n <head>\n  <title>401 User amosjyng has two factor auth enabled, an API Token or Trusted Publisher must be used to upload in place of password.\n \n <body>\n  <h1>401 User amosjyng has two factor auth enabled, an API Token or Trusted Publisher must be used to upload in place of password.\n  This server could not verify that you are authorized to access the document you requested.  Either you supplied the wrong credentials (e.g., bad password), or your browser does not understand how to supply the credentials required.<br/><br/>\nUser amosjyng has two factor auth enabled, an API Token or Trusted Publisher must be used to upload in place of password.\n\n\n \n'
```

it's because you need to set the PYPI password as an API key instead of your own regular password.

If you get

```
Publishing langchain-visualizer (0.0.30) to PyPI
 - Uploading langchain_visualizer-0.0.30-py3-none-any.whl FAILED

HTTP Error 403: Invalid or non-existent authentication information. See https://pypi.org/help/#invalid-auth for more information. | b'<html>\n <head>\n  <title>403 Invalid or non-existent authentication information. See https://pypi.org/help/#invalid-auth for more information.\n \n <body>\n  <h1>403 Invalid or non-existent authentication information. See https://pypi.org/help/#invalid-auth for more information.\n  Access was denied to this resource.<br/><br/>\n\n\n\n \n'
```

It's because you need to set the PYPI username as `__token__` instead of your own regular username.
