use dioxus::prelude::*;

use crate::components::{button::*, card::*, input::*, label::*, switch::*};

#[component]
pub fn SettingsCard() -> Element {
    // State for bedtime settings
    let mut bedtime_enabled = use_signal(|| false);
    let mut bedtime_hour = use_signal(|| String::from("22"));
    let mut bedtime_minute = use_signal(|| String::from("00"));

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
                                div { style: "display: flex; flex-direction: column; gap: 0.5rem; flex: 1;",
                                    Label { html_for: "bedtime-hour", "時" }
                                    Input {
                                        id: "bedtime-hour",
                                        r#type: "number",
                                        value: bedtime_hour,
                                        oninput: move |e: FormEvent| bedtime_hour.set(e.value()),
                                        min: "0",
                                        max: "23",
                                        step: "1",
                                    }
                                }
                                span { style: "padding-top: 1.5rem; font-size: 1.25rem;",
                                    ":"
                                }
                                div { style: "display: flex; flex-direction: column; gap: 0.5rem; flex: 1;",
                                    Label { html_for: "bedtime-minute", "分" }
                                    Input {
                                        id: "bedtime-minute",
                                        r#type: "number",
                                        value: bedtime_minute,
                                        oninput: move |e: FormEvent| bedtime_minute.set(e.value()),
                                        min: "0",
                                        max: "59",
                                        step: "1",
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
