use boa_engine::{
    Context, JsArgs, JsResult, JsString, JsValue, NativeFunction, Source, js_string,
    object::ObjectInitializer, property::Attribute,
};

use crate::analyzer::dynamic::DynamicAnalyzer;

pub trait PluginAnalyzer {
    fn has(&self, plugin_name: &str) -> bool;
    fn has_nocase(&self, plugin_name: &str) -> bool;
    fn has_loaded(&self, plugin_name: &str) -> bool;
    fn has_loaded_nocase(&self, plugin_name: &str) -> bool;
    fn version(&self, plugin_name: &str) -> Option<String>;
}

impl PluginAnalyzer for DynamicAnalyzer {
    fn has(&self, plugin_name: &str) -> bool {
        todo!()
    }

    fn has_nocase(&self, plugin_name: &str) -> bool {
        todo!()
    }

    fn version(&self, plugin_name: &str) -> Option<String> {
        todo!()
    }

    fn has_loaded(&self, plugin_name: &str) -> bool {
        todo!()
    }

    fn has_loaded_nocase(&self, plugin_name: &str) -> bool {
        todo!()
    }
}

const PLUGINS: &[(&str, &str)] = &[("WorldEdit", "7.3.0"), ("Vault", "1.7.3")];

fn find(name: &str) -> Option<&'static str> {
    PLUGINS.iter().find(|(n, _)| *n == name).map(|(_, v)| *v)
}

fn plugin_has(_this: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let name = args
        .get_or_undefined(0)
        .to_string(ctx)?
        .to_std_string_escaped();
    Ok(JsValue::from(find(&name).is_some()))
}

fn plugin_version(_this: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let name = args
        .get_or_undefined(0)
        .to_string(ctx)?
        .to_std_string_escaped();
    Ok(match find(&name) {
        Some(v) => JsValue::from(JsString::from(v)),
        None => JsValue::null(),
    })
}