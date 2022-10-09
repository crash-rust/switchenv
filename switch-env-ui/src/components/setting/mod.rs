use crate::components::{tabgroup::Env, TabGroup, TextInput};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use stylist::yew::styled_component;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    fn alert(str: &str);
}

#[derive(Serialize, Deserialize)]
struct EnmacConfig {
    env: Env,
}

#[styled_component(SettingBox)]
pub fn setting_box() -> Html {
    let current_env = use_state(|| Env::Test);
    let cloned_current_env = current_env.clone();
    let cur_env = &*cloned_current_env.clone();

    let onchange = Callback::from(move |env| match env {
        Env::Test => {
            cloned_current_env.set(Env::Test);
            spawn_local(async move {
                invoke(
                    "change_enow_env",
                    to_value(&EnmacConfig { env: Env::Test }).unwrap(),
                )
                .await;
            });

            // 这里应该将enmac的环境改成测试环境的配置
        }
        Env::Prod => {
            cloned_current_env.set(Env::Prod);
            spawn_local(async move {
                invoke(
                    "change_enow_env",
                    to_value(&EnmacConfig { env: Env::Prod }).unwrap(),
                )
                .await;
            });
            // 这里应该将enmac的环境改成正式环境的配置
        }
    });

    html! {
        <div>
           <div class="setting-container">
            <div class="font-headline-medium enmac-setting">
             <div class="title">{"白板配置"}</div>
             <div class="setting-item enmac-env-change">
               <div class="font-title-medium setting-env-text">{"服务环境"}</div>
               <div class="tab-group-container">
                  <TabGroup default_env={cur_env.clone()} left_tab={"测试环境".to_string()} right_tab={"正式环境".to_string()} {onchange}/>
               </div>
             </div>
            </div>
            <div class="font-headline-medium system-setting">
              <div class="title">{"系统配置"}</div>
              <div class="setting-item wifi-proxy">
                <div class="proxy-control">
                  <div class="font-title-medium setting-env-text">{"无线代理"}</div>
                </div>
                <div class="proxy-wrapper">
                  <div class="proxy-item">
                     <div class="proxy-item-text">{"HTTP代理"}</div>
                     <div class="proxy-server-wrapper">
                        <div class="proxy-server">{"服务器"}</div>
                        <div class="proxy-input">
                          <TextInput max_length={"15".to_string()}/>
                        </div>
                     </div>
                     <div class="proxy-port-wrapper">
                       <div class="proxy-port">{"端口"}</div>
                        <div class="proxy-input">
                          <TextInput max_length={"4".to_string()}/>
                        </div>
                     </div>
                  </div>
                  <div class="proxy-item">
                      <div class="proxy-item-text">{"HTTPS代理"}</div>
                       <div class="proxy-server-wrapper">
                        <div class="proxy-server">{"服务器"}</div>
                         <div class="proxy-input">
                          <TextInput max_length={"15".to_string()}/>
                        </div>
                     </div>
                     <div class="proxy-port-wrapper">
                       <div class="proxy-port">{"端口"}</div>
                        <div class="proxy-input">
                          <TextInput max_length={"4".to_string()}/>
                        </div>
                      </div>
                  </div>
                </div>
              </div>
               <div class="setting-item lan-proxy">
               <div class="proxy-control">
                  <div class="font-title-medium setting-env-text">{"有线代理"}</div>
                </div>
                 <div class="proxy-wrapper">
                  <div class="proxy-item">{"HTTP代理"}</div>
                  <div class="proxy-item">{"HTTPS代理"}</div>
                </div>
              </div>
            </div>
           </div>
        </div>
    }
}
