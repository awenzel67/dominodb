dominodb
--------

The dominodb database is characterized by the following main features:
* A single JSON document as its data structure.
* JavaScript as the query language.
* JSON Schema to describe the JSON document.

Why we think this is a good idea then where is SQL and all the other QLs like MQL, ... with there corresponding databases sqlite and mongodb?

We found that KI Agents used to talk with your data can benefit from the described features:

* JSON data: Actual llms are well trained on structurized data using json encoding. The user input may contain json data but also the assintant output may contain json data. 
* javascript: This is one of the most common programming languages. Thus it's not surprising actual llms are well trained on this programming language. Additinonaly json data are a literal representation of the basic javascript object data structure. Thus working this the object data structure can be done using javascript syntax only.
* JSON schema: The schema describes the content of a JSON data object. The data types and relation between the different parts of the JSON data object but also the sematic meaning. llms trained for tool calling are also trained for JSON schema because paramters for tool calling are described by json schema.


