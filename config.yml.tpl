server:
  host: 0.0.0.0
  port: 9870

cache:
  host: 127.0.0.1
  port: 6379
  db: 0
  user: aa
  pass_word:  aa

db:
  host: 127.0.0.1
  port: 3306
  pg_port: 5432
  user: root
  pg_user: aa
  password: aa
  db: blog

qiniu:
  access_key: aa
  secret_key: aa
  bucket: aa
  host: https://cdn.xx.com/

github:
  client_id: aa,
  client_secret: aa,

mail:
  smtp_host:  smtp.gmail.com
  smtp_port:  587
  smtp_username: xxx@gmail.com
  smtp_username: xxxx
  max_client: 100

ali_pay:
  # 应用私钥
  private_key: xx
  # 支付宝公钥
  public_key: xx
  app_id: xx

wechat_pay:
   app_id:
   app_secret: