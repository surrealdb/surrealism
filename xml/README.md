# surrealism-xml

XML-to-JSON conversion for Surrealism, using the "xmltodict" convention.

```surql
DEFINE MODULE mod::xml AS f"bucket:/xml.surli";

RETURN mod::xml::to_json("<root attr=\"1\"><item>a</item><item>b</item></root>");
-- { root: { "@attr": "1", item: ["a", "b"] } }
```

| Function | Signature | Description |
|---|---|---|
| `to_json` | `(xml: string) -> object` | Converts an XML document to JSON using the xmltodict convention |

The root element becomes the single top-level key of the returned object.
Attributes become keys prefixed with `@`. An element with only text content
and no attributes or child elements collapses to that plain text string. An
element with attributes and/or child elements becomes an object; direct text
content on such an element goes under a `#text` key. A tag repeated as a
sibling under the same parent collects all its occurrences into an array, in
encounter order, at any nesting depth. Whitespace-only text between elements
is ignored. Malformed XML returns an error.
