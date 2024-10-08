# qr_code_reader

This is a simple QR Code Reader written in rust. I wrote it to parse QR Codes taken at Career Fairs such as AISES. https://www.aises.org/

# Usage:

1. Individual Files
```
$ cargo run -- --file qrcode1.jpeg
QR code 1 from file qrcode1.jpeg: hxxps://redactedurl1.com
```
2. An entire directory
```
$ cargo run -- --dir /directory/images/

QR code 1 from file /directory/images/qrcode1.jpeg: hxxps://redactedurl1.com
QR code 1 from file /directory/images/qrcode2.jpeg: hxxps://redactedurl2.com
QR code 1 from file /directory/images/qrcode3.jpeg: hxxps://redactedurl3.com
QR code 1 from file /directory/images/qrcode4.jpeg: hxxps://redactedurl4.com
```
