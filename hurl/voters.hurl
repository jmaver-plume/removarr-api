# Create a voter
POST {{URL}}/api/voters
Content-Type: application/json
{ "name": "Alice" }
HTTP 200
[Captures]
voter_id: jsonpath "$.id"

# List all voters
GET {{URL}}/api/voters
HTTP 200
[Asserts]
jsonpath "$" count > 0

# Get specific voter
GET {{URL}}/api/voters/{{voter_id}}
HTTP 200
[Asserts]
jsonpath "$.name" == "Alice"
jsonpath "$.id" == {{voter_id}}

# Update voter
PATCH {{URL}}/api/voters/{{voter_id}}
Content-Type: application/json
{ "name": "Alice Updated" }
HTTP 200
[Asserts]
jsonpath "$.name" == "Alice Updated"
jsonpath "$.id" == {{voter_id}}

# Verify update
GET {{URL}}/api/voters/{{voter_id}}
HTTP 200
[Asserts]
jsonpath "$.name" == "Alice Updated"

# Delete voter
DELETE {{URL}}/api/voters/{{voter_id}}
HTTP 204

# Verify deletion
GET {{URL}}/api/voters/{{voter_id}}
HTTP 404
