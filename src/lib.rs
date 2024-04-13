pub use makepad_widgets;
pub mod app;

pub fn get_raw_context() -> robius_authentication::RawContext {
    #[cfg(target_os = "android")] {
        use robius_authentication::jni::{JavaVM, JObject};
        use makepad_widgets::makepad_platform::os::linux::android::android_jni::{get_java_vm, get_activity};

        let jvm = unsafe { JavaVM::from_raw(get_java_vm().cast()).unwrap() };
        let activity = unsafe { JObject::from_raw(get_activity().cast()) };
        let activity_global_ref = jvm.get_env().unwrap().new_global_ref(activity).unwrap();
        let raw_context = (jvm, activity_global_ref);
        raw_context
    }
    #[cfg(not(target_os = "android"))] {
        ()
    }
}
