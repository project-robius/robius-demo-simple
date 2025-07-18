use makepad_widgets::*;

live_design!{
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

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
                    text: "Robius Simple Interactive Demo"
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
                        empty_text  : "Enter authentication prompt..."
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
                        empty_text: "Enter URI..."
                    }

                    open_button = <Button> {
                        text: "Open"
                    }

                    <LineH> { }
                }

                location_view = <View> {
                    flow: Down,
                    width: Fill, height: Fit,
                    padding: 10.0,
                    spacing: 10,
                    align: { x: 0.5, y: 0.5 },

                    location_button = <Button> {
                        text: "Start location updates"
                    }

                    location_label = <Label> {
                        align: { x: 0.5 },
                        width: Fill, height: Fit
                        draw_text: { color: #f, text_style: { font_size: 12 } }
                        text: "Waiting to request location..."
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

    #[cfg(feature = "location")]
    #[rust] location_manager: Option<robius_location::Manager>,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl App {
    #[cfg(feature = "authentication")]
    fn handle_auth_action(&mut self, cx: &mut Cx, actions: &Actions) {
        use robius_authentication::{Context, WindowsText, AndroidText, PolicyBuilder, Text};

        let label = self.ui.label(id!(auth_result));
        for action in actions {
            if let Some(AuthenticationAction::Completed(result)) = action.downcast_ref() {
                match result {
                    Ok(_) => {
                        label.set_text(cx, "Authentication successful!");
                    }
                    Err(e) => {
                        label.set_text(cx, &format!("Authentication failed: {e:?}"));
                    }
                }
            }
        }

        let auth_text_input = self.ui.text_input(id!(auth_input));
        let triggered_msg = if let Some((t, _)) = auth_text_input.returned(&actions) {
            t
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
    
        log!("Authenticating with message {triggered_msg:?}");

        let auth_policy = PolicyBuilder::new()
            .build()
            .expect("invalid policy configuration");

        let context = Context::new(());
        let auth_result = context.authenticate(
            Text {
                android: AndroidText {
                    title: message,
                    subtitle: None,
                    description: None,
                },
                apple: message,
                windows: WindowsText::new_truncated("Authentication Request", message),
            },
            &auth_policy,
            |result| {
                log!("Authentication result in callback: {result:?}");
                Cx::post_action(AuthenticationAction::Completed(result));
            }
        );

        match auth_result {
            Ok(_) => {
                label.set_text(cx, "Waiting for auth result...");
            }
            Err(e) => {
                error!("Authentication failed to start: {e:?}");
                label.set_text(cx, &format!("Authentication failed: {e:?}"));
            }
        } 
    }


    #[cfg(feature = "open")]
    fn handle_open_action(&mut self, _cx: &mut Cx, actions: &Actions) {
        let open_text_input = self.ui.text_input(id!(open_input));
        let uri = if let Some((t, _)) = open_text_input.returned(&actions) {
            t
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
        if let Err(e) = robius_open::Uri::new(&uri).open() {
            error!("Failed to open URI: {e:?}");
        }
    }

    #[cfg(feature = "location")]
    fn handle_location_action(&mut self, cx: &mut Cx, actions: &Actions) {
        let location_label = self.ui.label(id!(location_label));
        if self.ui.button(id!(location_button)).clicked(actions) {
            log!("Location requested. Waiting for updates...");
            location_label.set_text(cx, "Location requested. Waiting for updates...");
            if let Ok(mut manager) = crate::location::init(cx) {
                let _ = manager.start_updates();
                self.location_manager = Some(manager);
                log!("Location manager initialized successfully.");
            } else {
                error!("Failed to initialize location manager.");                
            }
        }

        for action in actions {
            match action.downcast_ref() {
                Some(crate::location::LocationAction::Update(update)) => {
                    let text = format!(
                        "Location: lat {:.6}, lon {:.6}, time: {:?}",
                        update.coordinates.latitude, update.coordinates.longitude, update.time
                    );
                    location_label.set_text(cx, &text);
                }
                Some(crate::location::LocationAction::Error(e)) => {
                    error!("Location error: {e:?}");
                    location_label.set_text(cx, &format!("Location error: {e:?}"));
                }
                _ => {}
            }
        }
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

        #[cfg(not(feature = "location"))] {
            warning!("The `location` feature is disabled.");
            self.ui.view(id!(location_view)).set_visible(false);
        }
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        #[cfg(feature = "authentication")]
        self.handle_auth_action(cx, actions);

        #[cfg(feature = "open")]
        self.handle_open_action(cx, actions);

        #[cfg(feature = "location")]
        self.handle_location_action(cx, actions);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
} 


/// Actions emitted from the authentication callback.
#[cfg(feature = "authentication")]
#[derive(Debug)]
enum AuthenticationAction {
    /// Authentication completed, either successfully or with an error.
    Completed(robius_authentication::Result<()>),
}



