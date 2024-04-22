use makepad_widgets::*;

live_design!{
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    LineH = <RoundedView> {
        width: Fill,
        height: 2,
        margin: 0.0
        padding: 0.0,
        spacing: 0.0
        draw_bg: {color: #f}
    }

    App = {{App}} {

        ui: <Window>{
            show_bg: true
            width: Fill,
            height: Fill
            
            draw_bg: {
                fn pixel(self) -> vec4 {
                    //return #000
                    return mix(#a, #5, self.pos.y);
                }
            }
            
            body = <ScrollXYView> {
                flow: Down,
                spacing: 20,
                padding: 15.0,
                align: { x: 0.5, y: 0.5 },

                <Label> {
                    text: "Robius Interactive Simple Demo App"
                    draw_text: { color: #e, text_style: { font_size: 16.0 } }

                }
                
                <LineH> { }

                auth_view = <View> {
                    flow: Down,
                    width: Fill, height: Fit,
                    padding: 10.0,
                    spacing: 10,
                    align: { x: 0.5, y: 0.5 },

                    auth_input = <TextInput> {
                        width: Fit, height: Fit
                        draw_text: { color: #f, text_style: { font_size: 12 } }
                        empty_message: "Enter authentication prompt..."
                    }

                    auth_button = <Button> {
                        text: "Authenticate"
                    }

                    auth_result = <Label> {
                        width: Fit, height: Fit
                        draw_text: { color: #f, text_style: { font_size: 12 } }
                        text: "Waiting to authenticate..."
                    }

                    <LineH> { }
                }


                open_view = <View> {
                    flow: Down,
                    width: Fill, height: Fit,
                    padding: 10.0,
                    spacing: 10,
                    align: { x: 0.5, y: 0.5 },

                    open_input = <TextInput> {
                        width: Fit, height: Fit
                        draw_text: { color: #f, text_style: { font_size: 12 } }
                        empty_message: "Enter URI..."
                    }

                    open_button = <Button> {
                        text: "Open"
                    }

                    <LineH> { }
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

impl App {
    #[cfg(feature = "authentication")]
    fn handle_auth_action(&mut self, cx: &mut Cx, actions: &Actions) {
        use robius_authentication::{BiometricStrength, PolicyBuilder};

        let auth_text_input = self.ui.text_input(id!(auth_input));
        let triggered_msg = if let Some(s) = auth_text_input.return_key(&actions) {
            s
        } else if self.ui.button(id!(auth_button)).clicked(&actions) {
            auth_text_input.text()
        } else {
            return
        };
        let message = if triggered_msg.is_empty() {
            // The system will preface a message with "This app wants to..."
            "authenticate with biometrics"
        } else {
            triggered_msg.as_str()
        };
    
        let label = self.ui.label(id!(auth_result));
        log!("Authenticating with message {triggered_msg:?}");

        #[cfg(not(feature = "authentication"))]
        warning!("Authentication feature is disabled.");
        let auth_policy = PolicyBuilder::new()
            .biometrics(Some(BiometricStrength::Strong))
            .password(true)
            .watch(true) // required in order to use the password option
            .build()
            .expect("invalid policy configuration");

        // label.set_text_and_redraw(cx, "Waiting to authenticate...");
        
        let context = robius_authentication::Context::new(crate::get_raw_context());
        let auth_result = context.blocking_authenticate(
            message,
            &auth_policy,
        );

        label.set_text_and_redraw(cx, &format!("Result: {auth_result:?}"));
    }


    #[cfg(feature = "open")]
    fn handle_open_action(&mut self, _cx: &mut Cx, actions: &Actions) {
        let open_text_input = self.ui.text_input(id!(open_input));
        let uri = if let Some(s) = open_text_input.return_key(&actions) {
            s
        } else if self.ui.button(id!(open_button)).clicked(&actions) {
            open_text_input.text()
        } else {
            return
        };
        if uri.is_empty() {
            log!("Ignoring attempt to open empty URI.");
            return;
        };

        log!("Opening URI: {uri:?}");
        robius_open::Uri::new(&uri).open();
    }
}
impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        #[cfg(not(feature = "authentication"))] {
            warning!("The `authentication` feature is disabled.");
            self.ui.view(id!(auth_view)).set_visible(false);
        }

        #[cfg(not(feature = "open"))] {
            warning!("The `open` feature is disabled.");
            self.ui.view(id!(open_view)).set_visible(false);
        }
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        #[cfg(feature = "authentication")]
        self.handle_auth_action(cx, actions);

        #[cfg(feature = "open")]
        self.handle_open_action(cx, actions);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
} 
