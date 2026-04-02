use zed_extension_api as zed;

struct PineScriptExtension;

impl zed::Extension for PineScriptExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(PineScriptExtension);
