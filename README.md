# Fast search engine
###### (like elastic/open/meili/typesense)
## Dictionary
* Field - key-value pair with type
* Document - Entry/Item/Row with fields
* Collection - Array of documents
## Supported Field Types
* Int,
* Bool,
* String
## Ranking
* BM25(for string fields)
### Overview
###### http is httpie
#### Info about collection
`http GET 'localhost:1551/collection/1'`
#### Create collection
`http PUT 'localhost:1551/collection/1' field_name=string`
#### Update collection
`http POST 'localhost:1551/collection/1' field_name=int`
#### Info about document
`http GET 'localhost:1551/collection/1/documents/0'`
#### Create document
`http PUT 'localhost:1551/collection/1/documents' field_name=123`
#### Update document
`http POST 'localhost:1551/collection/1/documents/0' field_name=12345`
#### Remove document
`http DELTE 'localhost:1551/collection/1/documents/0'`
#### Search
`http 'localhost:1551/collection/1/search?q=test&strict=false'`
#### Commit changes
`http GET 'localhost:1551/collection/1/commit'`

## Running
`cargo run --package sightnet_core_server --bin server`

## LICENSE
AGPL 3.0