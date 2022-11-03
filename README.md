# rust-blog

rust 练手

```
stay hungary Stay foolish
migrate https://github.com/x931890193/blog-backend
```
- [x] ORM  Rbatis ✅ 
- [x] redis  ✅
- [x] protobuf ✅


# pgconfig

```bash
edit /avr/lib/postgresql/data
# "local" is for Unix domain socket connections only
local   all             all                                     trust
# IPv4 local connections:
host    all             all             127.0.0.1/32            trust
# IPv6 local connections:
host    all             all             ::1/128                 trust

#host all all all scram-sha-256
host all all 0.0.0.0/0 password
```