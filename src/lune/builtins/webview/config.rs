use mlua::prelude::*;

pub struct LuaWebview {
    pub send_message: tokio::sync::watch::Sender<String>,
}

impl LuaUserData for LuaWebview {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("exit", |_lua, webview, _: ()| {
            if webview.send_message.send("^Exit".into()).is_err() {
                return Err(LuaError::RuntimeError(
                    "Failed to send exit message to webview".into(),
                ));
            };

            Ok(())
        });

        methods.add_method("open_devtools", |_lua, webview, _: ()| {
            if webview.send_message.send("^OpenDevtools".into()).is_err() {
                return Err(LuaError::RuntimeError(
                    "Failed to send exit message to webview".into(),
                ));
            };

            Ok(())
        });

        methods.add_method("close_devtools", |_lua, webview, _: ()| {
            if webview.send_message.send("^CloseDevtools".into()).is_err() {
                return Err(LuaError::RuntimeError(
                    "Failed to send exit message to webview".into(),
                ));
            };

            Ok(())
        });

        methods.add_method("load_url", |_lua, webview, url: LuaString| {
            if webview
                .send_message
                .send("^LoadUrl:".to_owned() + url.to_string_lossy().to_string().as_str())
                .is_err()
            {
                return Err(LuaError::RuntimeError(
                    "Failed to send exit message to webview".into(),
                ));
            };

            Ok(())
        });
    }
}

pub struct WebviewConfig<'a> {
    pub exit: Option<LuaFunction<'a>>,
    pub url: LuaString<'a>,
}

impl<'lua> FromLua<'lua> for WebviewConfig<'lua> {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        if let LuaValue::Table(tab) = &value {
            let exit: Option<LuaFunction> = tab.get("exit")?;
            let url: Option<LuaString> = tab.get("url")?;

            if url.is_none() {
                return Err(LuaError::RuntimeError(
                    "Invalid webview config - missing 'url'".to_owned(),
                ));
            }

            Ok(Self {
                exit,
                url: url.unwrap(),
            })
        } else {
            // Anything else is invalid
            Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "WebviewConfig",
                message: Some(format!(
                    "Invalid webview config - expected table, got {}",
                    value.type_name()
                )),
            })
        }
    }
}