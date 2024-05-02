# Geektime Rust 语言训练营 第一周作业

## 作业1 chacha20加解密

加密：
```ps
cargo run encrypt -k AHpveN84mAGJshihuviO8wffk5rqUIYU -i fixtures/text_to_encrypt.txt
```
将加密的字符串复制保存在`fixtures/text_to_decrypt.txt`，然后运行解密：
```ps
cargo run text decrypt -k AHpveN84mAGJshihuviO8wffk5rqUIYU -i fixtures/text_to_decrypt.txt
```

## 作业2 jwt签名验签

作业2 jwt签名验签运行示例：

生成签名：
```ps
cargo run jwt sign -s Goo -a www.google.com -e 1814593571436
```
验证签名：
```
cargo run jwt verify -t eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJHb28iLCJhdWQiOiJ3d3cuZ29vZ2xlLmNvbSIsImV4cCI6MTcxNDU5MzU3MTQzNn0.CRIoZuLnQ6JUJ9nwHfATCd6PQxVxp4ZWoy32fdoct3VNx9CpIUpKHQIVb8EGfYn0FREy6axobVbmFzC7uBMvRA -a www.google.com
```

## 作业3 http访问文件夹

启动http服务器
```ps
$env:RUST_LOG='info'; cargo run http serve
```
浏览器访问：http://localhost:8080/fixtures，然后继续点击链接