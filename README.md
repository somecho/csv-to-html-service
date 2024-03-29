# csv-to-html-service

A web service that converts CSV to a HTML table. This wraps
[csv-to-html](https://github.com/somecho/csv-to-html) into a tool that lets you
try it out.


## Example

Try it out with curl:

```sh
curl -X POST -L https://csvhtml.soch.cc -H "Content-Type: text/csv" -d $'name,age\nmickey,99'
```

This returns the following table:

<table><thead><tr><th>name</th><th>age</th></tr></thead><tbody><tr><td>mickey</td><td>99</td></tr></tbody></table>

Which looks like this in html:
```html
<table><thead><tr><th>name</th><th>age</th></tr></thead><tbody><tr><td>mickey</td><td>99</td></tr></tbody></table>
```

## Parameters

There are two parameters you can pass as URL query parameters. 

| Param | Description | Example |
| - | - | - |
| `delimiter` | the delimiter used for parsing the csv `[default: ,]` | `?delimiter=%20` (URL encoded space ) |
| `hasHeaders` | when true, the first row is used as the table header `[default: true]` | `?hasHeaders=false` |


