use dioxus::prelude::*;

use crate::components::{button::*, card::*, label::*, select::*, switch::*};

#[component]
pub fn SettingsCard() -> Element {
    // State for bedtime settings
    let mut bedtime_enabled = use_signal(|| false);

    // State for notification settings
    let mut notification_message = use_signal(|| true);
    let mut notification_vibration = use_signal(|| false);
    let mut notification_sound = use_signal(|| false);

    rsx! {
        Card {
            CardHeader {
                CardTitle { "基本設定" }
                CardDescription { "就寝時刻と通知方法を設定します" }
            }
            CardContent {
                div { style: "display: flex; flex-direction: column; gap: 1.5rem;",

                    // Bedtime settings section
                    div { style: "display: flex; flex-direction: column; gap: 1rem;",
                        div { style: "display: flex; align-items: center; justify-content: space-between;",
                            div {
                                Label { html_for: "bedtime-enabled", "就寝時刻の設定" }
                                p { style: "font-size: 0.875rem; color: var(--secondary-color-5); margin: 0.25rem 0 0 0;",
                                    "設定した時刻を過ぎると催促を開始します"
                                }
                            }
                            Switch {
                                id: "bedtime-enabled",
                                checked: bedtime_enabled(),
                                on_checked_change: move |new_checked| {
                                    bedtime_enabled.set(new_checked);
                                },
                                SwitchThumb {}
                            }
                        }

                        if *bedtime_enabled.read() {
                            div { style: "display: flex; gap: 1rem; align-items: center; padding-left: 1rem;",
                                div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
                                    Label { html_for: "bedtime-hour", "時" }
                                    Select::<String> {
                                        placeholder: "時",
                                        name: "bedtime-hour",
                                        SelectTrigger { SelectValue {} }
                                        SelectList {
                                            {
                                                (20..24)
                                                    .map(|h| {
                                                        let h_str = format!("{:02}", h);
                                                        rsx! {
                                                            SelectOption::<String> {
                                                                index: h as usize,
                                                                key: "{h_str}",
                                                                value: h_str.clone(),
                                                                text_value: h_str.clone(),
                                                                "{h_str}"
                                                                SelectItemIndicator {}
                                                            }
                                                        }
                                                    })
                                            }
                                        }
                                    }
                                }
                                span { style: "padding-top: 1.5rem; font-size: 1.25rem;",
                                    ":"
                                }
                                div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
                                    Label { html_for: "bedtime-minute", "分" }
                                    Select::<String> {
                                        placeholder: "分",
                                        name: "bedtime-minute",
                                        SelectTrigger { SelectValue {} }
                                        SelectList {
                                            {
                                                ["00", "15", "30", "45"]
                                                    .into_iter()
                                                    .enumerate()
                                                    .map(|(i, m)| {
                                                        let m_str = m.to_string();
                                                        rsx! {
                                                            SelectOption::<String> {
                                                                index: i,
                                                                key: "{m_str}",
                                                                value: m_str.clone(),
                                                                text_value: m_str.clone(),
                                                                "{m_str}"
                                                                SelectItemIndicator {}
                                                            }
                                                        }
                                                    })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Divider
                    hr { style: "border: none; border-top: 1px solid var(--primary-color-6); margin: 0;" }

                    // Notification settings section
                    div { style: "display: flex; flex-direction: column; gap: 1rem;",
                        div {
                            Label { html_for: "notification-section", "通知方法" }
                            p { style: "font-size: 0.875rem; color: var(--secondary-color-5); margin: 0.25rem 0 0 0;",
                                "催促に使用する通知方法を選択します"
                            }
                        }

                        div { style: "display: flex; flex-direction: column; gap: 0.75rem; padding-left: 1rem;",
                            div { style: "display: flex; align-items: center; justify-content: space-between;",
                                Label { html_for: "notification-message", "メッセージ通知" }
                                Switch {
                                    id: "notification-message",
                                    checked: notification_message(),
                                    on_checked_change: move |new_checked| {
                                        notification_message.set(new_checked);
                                    },
                                    SwitchThumb {}
                                }
                            }

                            div { style: "display: flex; align-items: center; justify-content: space-between;",
                                div {
                                    Label { html_for: "notification-vibration",
                                        "バイブレーション"
                                    }
                                    p { style: "font-size: 0.75rem; color: var(--secondary-color-5); margin: 0.25rem 0 0 0;",
                                        "Android のみ"
                                    }
                                }
                                Switch {
                                    id: "notification-vibration",
                                    checked: notification_vibration(),
                                    on_checked_change: move |new_checked| {
                                        notification_vibration.set(new_checked);
                                    },
                                    SwitchThumb {}
                                }
                            }

                            div { style: "display: flex; align-items: center; justify-content: space-between;",
                                Label { html_for: "notification-sound", "音" }
                                Switch {
                                    id: "notification-sound",
                                    checked: notification_sound(),
                                    on_checked_change: move |new_checked| {
                                        notification_sound.set(new_checked);
                                    },
                                    SwitchThumb {}
                                }
                            }
                        }
                    }
                }
            }
            CardFooter { style: "display: flex; justify-content: flex-end;",
                Button { onclick: move |_| {}, "保存" }
            }
        }
    }
}
