use crate::components::toast::manager::ToastManager;
use crate::components::toast::Toast;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastProviderProps {
    pub children: Element,
}

#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    #[allow(unused_mut)]
    let mut manager = use_signal(|| ToastManager::default());

    client! {
        let mut eval = document::eval(
            r#"
            setInterval(() => {
                dioxus.send("");
            }, 1000)
            "#,
        );

        use_hook(|| {
            spawn(async move {
                loop {
                    let _ = eval.recv::<String>().await;
                    manager.write().cleanup_expired();
                }
            })
        });
    }
    use_context_provider(|| manager);

    rsx! {
        div {
            class: "relative",
            {props.children},
            if manager().toasts.len() > 0 {
                div {
                    class: "absolute bottom-4 right-4 space-y-4",
                    for (_id, toast) in manager().toasts.iter() {
                        Toast {
                            key: "{toast.id}",
                            toast: toast.clone(),
                            // onclose: move |e: Event<MouseData>| manager.write().remove_toast(toast.id)
                        }
                    }
                }
            }
        }
    }
}
