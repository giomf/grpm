# **G**ithub **R**elease **P**ackage **M**anager (GRPM)
GRPM is a CLI tool to install single binary releases directly from Github. 

## Install
```
$ grpm install zellij-org/zellij
 Package:      zellij 
 Version:      v0.34.4 
 Description:  A terminal workspace with batteries included 

 # | Name                                        | Size (MB) | Downloads 
---+---------------------------------------------+-----------+-----------
 0 | zellij-aarch64-apple-darwin.sha256sum       |      0.00 |         1 
 1 | zellij-aarch64-apple-darwin.tar.gz          |      5.64 |       163 
 2 | zellij-aarch64-unknown-linux-musl.sha256sum |      0.00 |         7 
 3 | zellij-aarch64-unknown-linux-musl.tar.gz    |      5.81 |       110 
 4 | zellij-x86_64-apple-darwin.sha256sum        |      0.00 |         3 
 5 | zellij-x86_64-apple-darwin.tar.gz           |      5.97 |       336 
 6 | zellij-x86_64-unknown-linux-musl.sha256sum  |      0.00 |        12 
 7 | zellij-x86_64-unknown-linux-musl.tar.gz     |      6.38 |      1754 

Choose an asset to download: 7
Downloading zellij-x86_64-unknown-linux-musl.tar.gz...
Decompressing zellij-x86_64-unknown-linux-musl.tar.gz...
Reading zellij-x86_64-unknown-linux-musl.tar.gz...
Installing zellij to /home/giom/.local/bin
Done!
```
## List
```
$ grpm list
 Package | Version | Path 
---------+---------+-----------------------
 zellij  | v0.34.4 | /home/giom/.local/bin 
```

## Uninstall
```
$ grpm uninstall zellij
```