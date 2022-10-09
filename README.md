# 概述

SwitchEnv是一个基于Rust并以Tauri+Yew为基础框架开发的用于服务环境切换小工具
# issue

1. 客户端加载svg，以http的方式加载不出来，但是浏览器是可以的：`https://github.com/tauri-apps/tauri/issues/3007`的
   [解决方式](https://tauri.app/zh/v1/api/js/http/)
   ```json{
    "tauri": {
       "allowlist": {
        "all": true,
        "http": {
          "all": true,
          "request": true
        }
      },
    }
   ```
