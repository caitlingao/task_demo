## API
前置说明：

通过 headers 中的 Authorization 校验登录用户的 token 有效性，type 为 Beare

1. 注册 POST api/v1/users
```json
params: 
{
  "username": "first user",
  "email": "example008@example.com",
  "password": "123456"
}

response:
{
  "code": 0,
  "data": {
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MTExOTU2NDEsImV4cCI6MTYxMTgwMDQ0MSwidXNlciI6ImV4YW1wbGUwMDhAZXhhbXBsZS5jb20iLCJsb2dpbl9zZXNzaW9uIjoiNjgwMWJlYmI3ZGUwNDBlNTgyMzZiNGJmYmVlOWJkYjAifQ.DRYu4VXK-Nh_oA0wISCGjyijQdGLlY4B4iUDscnC02o",
    "token_type": "bearer"
  },
  "message": "Signup successfully"
}
```

2. 登录 POST api/v1/sessions
```json
params: 
{
  "username": "first user",
  "email": "example008@example.com",
  "password": "123456"
}

response:
{
  "code": 0,
  "data": {
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MTExOTU2NDEsImV4cCI6MTYxMTgwMDQ0MSwidXNlciI6ImV4YW1wbGUwMDhAZXhhbXBsZS5jb20iLCJsb2dpbl9zZXNzaW9uIjoiNjgwMWJlYmI3ZGUwNDBlNTgyMzZiNGJmYmVlOWJkYjAifQ.DRYu4VXK-Nh_oA0wISCGjyijQdGLlY4B4iUDscnC02o",
    "token_type": "bearer"
  },
  "message": "Login successfully"
}
```

3. 退出登录 DELETE api/v1/sessions
```json
response:
{
  "code": 0,
  "data": "",
  "message": "Logout successfully"
}
```
4. 获取 task 列表 GET api/v1/tasks
```json
response:
{
  "code": 0,
  "data": [
    {
      "content": "first task",
      "created_at": "2021-01-13T06:18:21.549097",
      "finished": false,
      "id": 3,
      "updated_at": "2021-01-13T06:18:21.549097",
      "user_id": 6
    }
  ],
  "message": "Logout successfully"
}
```
5. 添加 task POST api/v1/tasks
```json
params: 
{
  "content": "first task with api"
}

response:
{
  "code": 0,
  "data": "",
  "message": "ok"
}
```
6. 设置 task 为已完成 PUT api/v1/tasks/:id/finish
```json
response:
{
  "code": 0,
  "data": "",
  "message": "ok"
}
```
7. 导出 tasks POST api/v1/tasks/export
```json
response:
{
  "code": 0,
  "data": "",
  "message": "ok"
}
```