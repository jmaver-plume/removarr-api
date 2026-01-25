PUT {{URL}}/removarr/api/settings
{ "sonarr": { "url": "http://localhost:5000/api", "api_key": "123" }, "radarr": { "url": "http://localhost:5000/api", "api_key": "123" }, "credentials": { "username": "john", "password": "cookie" }, "voters": ["John", "Jake"] }
HTTP 200


GET {{URL}}/removarr/api/settings
HTTP 200

