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

2. 使用trunk进行打包应用的时候，在`tauri.config.json`里的`beforeDevCommand`写上`cargo -p switch-env-cli && trunk serve switch-env-ui/index.html`并没有让trunk服务正常启动，会一直循环报Warning：`Waiting for your frontend dev server to start on http://localhost:1420`,经过github好心人帮助，解决了问题，解决方式是改成`cargo -p switch-env-cli && cd switch-env-ui && trunk serve`,附上相关issue:(https://github.com/tauri-apps/tauri/issues/5375)
