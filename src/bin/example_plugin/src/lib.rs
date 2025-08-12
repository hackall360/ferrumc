use ferrumc_plugins::Plugin;
use tracing::info;

struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn name(&self) -> &'static str {
        "example-plugin"
    }

    fn on_chat_message(&self, msg: &mut String) {
        info!("chat from plugin: {msg}");
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
    Box::new(ExamplePlugin)
}
