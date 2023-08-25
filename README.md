# HQ a tool to query data from HTML.

## LICENSE 
MIT

## How to use HQ
Query HTML content via CSS or XPATH.

```
hq [--url|u URL] <--xpath XPATH | --css CSS-SELECTOR>
   If no --url or -u specified, read from stdin.
   --xpath | -x XPATH. xpath to select the document node. By default it output the text of the selected nodes; Use '@' to select the property. e.g.: //div@name
   --css | -c SELECTOR select the document node. By default it output the text of the selected nodes; Use '@' to select the property. e.g.: div@name
```
