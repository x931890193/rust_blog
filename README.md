# rust-blog

rust 练手

```
stay hungary Stay foolish
migrate https://github.com/x931890193/blog-backend
```
- [x] ORM  Rbatis ✅ 
- [x] redis  ✅
- [x] protobuf ✅
- [x] crontab ✅
- [x] websocket ✅
- [x] middleware add user ✅
- [x] read configuration from toml file ✅

- 写不动了 基本所有的功能都有模板了、 接下来就是CRUD了, 没意思了 
- Wed Nov 23 00:09:50 CST 2022


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
