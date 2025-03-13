use crate::components::icons::AboutIcon;
use crate::components::popup_modal::PopupModal;
use crate::components::trans::Trans;
use crate::hooks::translation::{Namespace, use_translation};
use yew::prelude::*;

#[function_component]
pub(crate) fn AboutMenu() -> Html {
    let translation = use_translation(vec!["main".to_string(), "about".to_string()]);
    let modal_open = use_state(|| false);
    let on_click = {
        let modal_open = modal_open.clone();
        move |_e: MouseEvent| {
            modal_open.set(true);
        }
    };
    let set_is_modal_open = {
        let modal_open = modal_open.clone();
        move |value: bool| {
            modal_open.set(value);
        }
    };
    html! {
      <>
        <a
          class="flex py-2 px-2 items-center gap-3 rounded-md hover:bg-gray-500/10 transition-colors duration-200 text-white cursor-pointer text-sm"
          onclick={ on_click }
        >
          <div>
            <AboutIcon />
          </div>
          { translation("about".to_string(), None) }
        </a>
        if *modal_open {
          <PopupModal
            title={ translation("about".to_string(), None) }
            set_is_modal_open={ set_is_modal_open }
            cancel_button={false}
          >
            <div class="p-6 border-b border-gray-200 dark:border-gray-600">
              <div class="min-w-fit text-gray-900 dark:text-gray-300 text-sm flex flex-col gap-3 leading-relaxed">
                <p>{translation("description".to_string(), Some(Namespace { ns: "about".to_string() }))}</p>
                <p>
                  <Trans
                  i18n_key="sourceCode"
                    ns="about"
                    components={vec![
                      html! {
                        <a
                          href="https://github.com/ztjhz/BetterChatGPT"
                          target="_blank"
                          class="link"
                        />
                      }
                    ]}
                  />
                </p>

                <p>
                  <Trans
                  i18n_key="initiative.description"
                    ns="about"
                    components={vec![
                      html! {
                        <a
                          href={translation("initiative.link".to_string(), Some(Namespace { ns: "about".to_string() }))}
                          target="_blank"
                          class="link"
                        />
                      }
                    ]}
                  />
                </p>

                <h2 class="text-lg font-bold">
                  {translation("discordServer.title".to_string(), Some(Namespace { ns: "about".to_string() }))}
                </h2>
                <p>{translation("discordServer.paragraph1".to_string(), Some(Namespace { ns: "about".to_string() }))}</p>

                <p>
                  <Trans
                  i18n_key="discordServer.paragraph2"
                    ns="about"
                    components={vec![
                      html! {
                        <a
                          class="link"
                          href="https://discord.gg/g3Qnwy4V6A"
                          target="_blank"
                        />
                      }
                    ]}
                  />
                </p>

                <>
                  <h2 class="text-lg font-bold">
                    {translation("support.title".to_string(), Some(Namespace { ns: "about".to_string() }))}
                  </h2>
                  <p>{translation("support.paragraph1".to_string(), Some(Namespace { ns: "about".to_string() }))}</p>
                  <p>
                    <Trans
                      i18n_key="support.paragraph2"
                      ns="about"
                      components={vec![
                        html! {
                          <a
                          href="https://github.com/ztjhz/BetterChatGPT"
                          target="_blank"
                          class="link"
                        />
                        }
                      ]}
                    />
                  </p>
                  <p>{translation("support.paragraph3".to_string(), Some(Namespace { ns: "about".to_string() }))}</p>

                  <div class="flex flex-col items-center gap-4 my-4">
                    <a href="https://github.com/sponsors/ztjhz" target="_blank">
                      <img
                        src="https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86"
                        width="120px"
                        alt="Support us through GitHub Sponsors"
                      />
                    </a>
                    <a href="https://ko-fi.com/betterchatgpt" target="_blank">
                      <img
                        src="./kofi.svg"
                        alt="Support us through the Ko-fi platform."
                      />
                    </a>
                    <div class="flex gap-x-10 gap-y-4 flex-wrap justify-center">
                      <div class="flex flex-col items-center justify-center gap-1">
                        <div>
                        {format!("{} {}", translation("support.alipay".to_string(), Some(Namespace { ns: "about".to_string() })), "(Ayaka)")}
                        </div>
                        <img
                          class="rounded-md w-32 h-32"
                          src="https://ayaka14732.github.io/sponsor/alipay.jpg"
                          alt="Support us through Alipay"
                        />
                      </div>
                      <div class="flex flex-col items-center justify-center gap-1">
                        <div>
                          {format!("{} {}", translation("support.wechatPay".to_string(), Some(Namespace { ns: "about".to_string() })), "(Ayaka)")}
                        </div>
                        <img
                          class="rounded-md w-32 h-32"
                          src="https://ayaka14732.github.io/sponsor/wechat.png"
                          alt="Support us through WeChat Pay"
                        />
                      </div>
                    </div>
                  </div>
                  <p>{translation("support.paragraph4".to_string(), Some(Namespace { ns: "about".to_string() }))}</p>
                </>

                <h2 class="text-lg font-bold">
                  {translation("privacyStatement.title".to_string(), Some(Namespace { ns: "about".to_string() }))}
                </h2>
                <p>{translation("privacyStatement.paragraph1".to_string(), Some(Namespace { ns: "about".to_string() }))}</p>

                <p>{translation("privacyStatement.paragraph2".to_string(), Some(Namespace { ns: "about".to_string() }))}</p>
              </div>
            </div>
          </PopupModal>
        }
      </>
    }
}
