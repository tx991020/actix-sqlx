

curl -v --data '{"name": "Bob", "email": "Bob@google.com", "password": "Bobpass"}' -H "Content-Type: application/json" -X POST localhost:9090/user/register

curl -v --data '{"name": "Bob", "email": "Bob@google.com", "password": "Bobpass"}' -H "Content-Type: application/json" -X POST localhost:9090/user/login

curl -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJCb2IiLCJleHAiOjE1OTEyNDYwOTR9.O1dbYu3tqiIi6I8OUlixLuj9dp-1tLl4mjmXZ0ve6uo' localhost:9090/user/userInfo