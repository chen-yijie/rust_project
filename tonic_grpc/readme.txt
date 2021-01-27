1.设置openssl的环境变量，OPENSSL_CONF=openssl.cnf路径

2.将openssl配置文件中的 req_extensions = v3_req注释取消掉

3.去掉[req_distinguished_name]里0.xxx开头的部分

4.修改[ v3_req ]和[ v3_ca ]内容

[ v3_req ]
extendedKeyUsage=serverAuth(增强型密钥用法，服务端验证)
subjectAltName = @alt_names(使用者备用名称)

[ v3_ca ]

basicConstraints = CA:true(使用基本约束)

5.增加[ alt_names ]，里面的内容可以增加多个DNS.X，IP.X，其中IP可以不填，但是DNS一定要和你的网站域名能匹配，比如你的网站是www.my.com，那么你可以设置为DNS.1=*.my.com

[ alt_names ]
DNS.1 = 127.0.0.1
IP.1 = 127.0.0.1

6 生成根证书
-aes256代表加密算法，还可以选择des3等
1024代表加密强度
-passout pass:123456 这里直接输入密码
openssl genrsa -passout pass:123456 -aes256  -out ca_private.key 2048 

7 使用根证书私钥生成csr(证书签名请求)
openssl req -passin pass:123456 -new -key ca_private.key -out ca.csr -subj "/C=CN/ST=ST/L=CITY/O=o/OU=ou/CN=cn"

8.使用根证书私钥和csr生成根证书
-days后面是天数，尽可能填大一点
天数后面是使用的算法，不要选择sha1，谷歌浏览器会认为不安全
-extensions表示扩展属性
-extfile表示扩展属性所在的文件
openssl x509 -req -passin pass:123456 -days 18250 -sha256 -signkey ca_private.key -extensions v3_ca -extfile openssl.cnf -in ca.csr -out ca.cer

9.生成服务端证书私钥
openssl genrsa -out server_private.key 2048

10.生成服务端证书签名请求
openssl req -new -key server_private.key -out server.csr -subj "/C=CN/ST=ST/L=CITY/O=o/OU=ou/CN=cn"

11.生成服务端证书
openssl x509 -req -passin pass:123456 -days 18250 -sha256 -extensions v3_req -extfile openssl.cnf -CA ca.cer -CAkey ca_private.key -CAserial ca.srl -CAcreateserial -in server.csr -out server.cer

12.CER转PEM
openssl x509 -in server.cer

13 转成
PEM转成PKCS12文件（包含CA证书、不包含CA证书）
openssl pkcs12 -export -inkey serverprikey.pem -in server.pem -CAfile demoCA/cacert.pem -password pass:"123456" -out server.pfx
openssl pkcs12 -export -inkey server_private.key -in server.cer -password pass:"123456" -out server_nocret.pfx

2）PKCS12转成PEM文件
openssl pkcs12 -in server_nocret.pfx -out server_nocret.pem -nodes -password pass:"123456"

3）查看pkcs12信息
openssl pkcs12 -in server.pfx -password pass:"123456" -info -nocerts –nokeys











