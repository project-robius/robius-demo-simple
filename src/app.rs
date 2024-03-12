use makepad_widgets::*;

#[cfg(target_os = "android")]
use makepad_widgets::makepad_platform::os::linux::android::android_jni::{get_java_vm, get_activity};

use robius_authentication::{BiometricStrength, PolicyBuilder};

live_design!{
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    App = {{App}} {

        ui: <Window>{
            show_bg: true
            width: Fill,
            height: Fill
            
            draw_bg: {
                fn pixel(self) -> vec4 {
                    //return #000
                    return mix(#7, #3, self.pos.y);
                }
            }
            
            body = <View>{
                flow: Down,
                spacing: 20,
                align: {
                    x: 0.5,
                    y: 0.5
                },
                
                auth_button = <Button> {
                    text: "Authenticate"
                }
                auth_label = <Label> {
                    draw_text: {
                        color: #f
                    },
                    text: "Click the above button."
                }

                
            }
        }
    }
} 
     
app_main!(App); 
 
#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){
        if self.ui.button(id!(auth_button)).clicked(&actions) {
            let label = self.ui.label(id!(auth_label));
            log!("CLICKED auth button.");

            let auth_policy = PolicyBuilder::new()
                .biometrics(Some(BiometricStrength::Strong))
                .password(true)
                .watch(true) // required in order to use the password option
                .build()
                .expect("invalid policy configuration");

            // The system will preface the message with "This app wants to..."
            let message = "authenticate with biometrics";

            label.set_text("Waiting to authenticate...");
            label.redraw(cx);

            let raw_context = {
                #[cfg(target_os = "android")] {
                    use robius_authentication::{ActivityObject, JavaVM, JObject};
                    let raw_context = unsafe { (
                        JavaVM::from_raw(get_java_vm().cast()).unwrap(),
                        ActivityObject::JObject(JObject::from_raw(get_activity().cast())),
                    ) };
                    raw_context
                }
                #[cfg(not(target_os = "android"))] {
                    ()
                }
            };
            let context = robius_authentication::Context::new(raw_context);
            let auth_result = context.blocking_authenticate(
                message,
                &auth_policy,
            );

            label.set_text_and_redraw(cx, &format!("Result: {auth_result:?}"));
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
} 
