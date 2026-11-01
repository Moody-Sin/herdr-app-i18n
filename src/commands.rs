//! Every command the app runs, defined once. Keys, menus, mods and
//! `herdr-app do` all run these; key checks, `herdr-app commands` and the docs come
//! from this list. Mods' own commands are `<mod>.<command>`.

use serde_json::{Value, json};

pub struct Arg {
    pub name: &'static str,
    pub required: bool,
}

pub struct Command {
    pub name: &'static str,
    pub args: &'static [Arg],
    pub about: &'static str,
}

const fn req(name: &'static str) -> Arg {
    Arg { name, required: true }
}

const fn opt(name: &'static str) -> Arg {
    Arg { name, required: false }
}

const fn cmd(name: &'static str, args: &'static [Arg], about: &'static str) -> Command {
    Command { name, args, about }
}

pub const COMMANDS: &[Command] = &[
    // What exists.
    cmd("space.new", &[opt("name"), opt("folder"), opt("tab"), opt("argv"), opt("task"), opt("machine")],
        "创建空间：设置名称（默认无，跟随首个终端所在路径命名）、终端初始目录（默认当前聚焦终端目录）、首个标签页名称（默认无，按序号编号），可选指定智能体（argv）及首条消息（task），以及所在机器（`hc machine ls`；快捷键/点击默认当前聚焦机器，脚本默认本机；空字符串表示本机；除非指定目录否则在机器家目录启动）。"),
    cmd("space.close", &[opt("id"), opt("confirmed")], "关闭空间并终止其中所有终端（默认弹出确认，除非已确认）。"),
    cmd("space.rename", &[opt("id"), opt("name")], "重命名空间（未传名称时弹出提示；名称为空时恢复按首个终端所在目录命名）。"),
    cmd("space.move", &[req("id"), req("index")], "移动空间至指定位置（1 表示首位）。"),
    cmd("tab.new", &[opt("name"), opt("space"), opt("folder"), opt("argv"), opt("task"), opt("mod"), opt("props")],
        "在空间中新建标签页（默认当前选中空间；默认无名称按序号编号），可在指定目录启动终端（默认当前聚焦终端目录），可选指定智能体（argv）及首条消息（task）；或以 Mod 面板作为唯一窗格（`mod` 及其属性 `props`，如文件或看板）。无空间打开时自动创建新空间。"),
    cmd("tab.close", &[opt("id"), opt("confirmed")], "关闭标签页并终止其中所有终端。"),
    cmd("tab.rename", &[opt("id"), opt("name")], "重命名标签页（未传名称时弹出提示；名称为空时恢复按序号编号）。"),
    cmd("tab.move", &[req("id"), req("index")], "移动标签页至所在空间的指定位置（1 表示首位）。"),
    cmd("pane.split", &[req("direction"), opt("pane"), opt("argv"), opt("mod"), opt("props")], "在当前聚焦窗格的“右侧”或“下方”分栏，启动新终端或并列展示 Mod 面板。"),
    cmd("pane.close", &[opt("pane")], "关闭窗格并终止其中的终端。"),
    cmd("terminal.open", &[opt("argv"), opt("folder"), opt("place"), opt("name")], "启动终端：可作为新标签页（place 为 \"tab\"，默认），或在聚焦窗格的“右侧”/“下方”分栏启动。"),
    cmd("terminal.keep", &[req("terminal"), opt("place")], "将无窗格运行的终端（子智能体）分配至窗格：新建标签页或在“右侧”/“下方”分栏。"),
    cmd("terminal.park", &[req("terminal")], "收起终端窗格并取消常驻保护；终端保持在后台运行（子智能体会重新归入父智能体树下）。"),
    cmd("terminal.right_click", &[req("to"), opt("terminal")], "设置终端内右键点击的去向（当程序接管鼠标时）：\"program\"（发送至程序）或 \"menu\"（Herdr 右键菜单，默认）。Shift+右键始终为菜单。"),
    cmd("terminal.pin", &[req("terminal"), req("pinned")], "将智能体置顶至智能体列表顶部（pinned 为 true）或取消置顶（false）。置顶状态保存在终端上，各客户端均可见，随终端退出而结束。"),
    cmd("terminal.kill", &[req("terminal")], "终止终端及其子智能体。"),
    // What you see.
    cmd("space.select", &[opt("index"), opt("id")], "展示指定空间（按位置序号或 ID）。"),
    cmd("space.next", &[], "切换至下一个空间。"),
    cmd("space.prev", &[], "切换至上一个空间。"),
    cmd("tab.select", &[opt("index"), opt("id")], "展示指定标签页（按空间内序号或 ID）。"),
    cmd("tab.settle", &[opt("id")], "在此屏幕上将标签页收起到后台（默认当前展示的标签页）：从列表中隐藏但保持运行。再次展示或当其中智能体需要处理时自动返回。"),
    cmd("tab.unsettle", &[opt("id")], "将已收起的标签页恢复至列表。"),
    cmd("tab.next", &[], "切换至下一个标签页。"),
    cmd("tab.prev", &[], "切换至上一个标签页。"),
    cmd("pane.zoom", &[opt("pane")], "仅全屏展示当前标签页的此窗格，或恢复分栏。"),
    cmd("pane.focus", &[opt("pane"), opt("terminal")], "聚焦窗格（或显示指定终端的窗格）。"),
    cmd("pane.go", &[req("direction")], "聚焦“左侧”、“右侧”、“上方”或“下方”的邻近窗格。"),
    cmd("pane.swap", &[req("direction")], "将聚焦窗格与指定方向上的邻近窗格对调位置。"),
    cmd("terminal.peek", &[req("terminal")], "在当前标签页上方悬浮速览运行中的终端，提供常驻为标签页和关闭等操作。"),
    cmd("terminal.show", &[opt("terminal"), opt("pane")], "临时在窗格中展示指定终端以替代原有终端（无 terminal 参数时切回）。"),
    cmd("terminal.copy", &[], "复制当前聚焦终端中的选中内容。"),
    cmd("terminal.paste", &[], "粘贴到当前聚焦终端（复制的文件模拟拖拽放入；仅有图片时同 terminal.paste_image）。"),
    cmd("terminal.paste_image", &[], "Ctrl+V：若聚焦在远程终端，将剪贴板图片作为文件保存至该机器；在 Windows 上粘贴剪贴板文本；否则发送 Ctrl+V 到终端。"),
    cmd("agent.focus", &[req("terminal")], "展示智能体（支持子智能体）。"),
    cmd("agent.next", &[], "按列表顺序切换至下一个智能体。"),
    cmd("agent.prev", &[], "切换至上一个智能体。"),
    cmd("layout.switch", &[opt("name")], "切换布局（layouts/<name>.json），或切换至下一个布局。"),
    cmd("layout.clone", &[req("name"), opt("as")], "将布局复制为 <name>-copy（或指定名称），并立即展示。"),
    cmd("mod.clone", &[req("name"), opt("as")], "将 Mod 复制为 <name>-copy（或指定名称）。"),
    cmd("layout.set", &[req("layout")], "替换自定义布局树；内置布局仅接受点击/外观字段（不改变结构）。"),
    cmd("layout.toggle", &[req("id")], "显示或隐藏布局节点。"),
    cmd("layout.show", &[req("id")], "显示布局节点。"),
    cmd("layout.hide", &[req("id")], "隐藏布局节点。"),
    cmd("layout.resize", &[req("id"), opt("px"), opt("weight")], "调整布局节点大小。"),
    cmd("layout.select", &[req("stack"), req("child")], "展示堆叠中的某个子节点。"),
    cmd("layout.reset", &[opt("name")], "重置内置布局及其点击状态（旧文件将被备份）。"),
    cmd("mod.open", &[req("name")], "按 mod.json 声明的遮罩层形式打开或关闭 Mod；再次调用即关闭。"),
    cmd("overlay.close", &[opt("id")], "关闭遮罩层（默认关闭最顶层的对话框或弹窗）。"),
    // The app.
    cmd("app.help", &[], "展示快捷键帮助。"),
    cmd("app.welcome", &[], "重新打开欢迎与入门向导。"),
    cmd("app.whats_new", &[], "展示本版本更新内容（更新后首次启动会自动打开）。"),
    cmd("app.feedback", &[opt("message"), opt("from")], "起草意见反馈以供用户审阅和发送（发送者：用户或智能体）。"),
    cmd("app.palette", &[], "打开或关闭命令面板：按名称列出全部命令及其快捷键；再次调用即关闭。"),
    cmd("app.settings", &[opt("section")], "打开设置（可选分区：look 外观、alerts 提醒、updates 更新）。"),
    cmd("app.check_updates", &[], "立即检查 Herdr 与 herdr-core 的新版本。"),
    cmd("app.update_core", &[], "将 herdr-core 更新至本代最新版本；运行中的终端保持不中断。"),
    cmd("app.reset", &[], "紧急恢复：备份当前布局和快捷键，恢复至默认屏幕。"),
    cmd("app.theme", &[req("name")], "应用主题（预设或 themes/<name>.json）。"),
    cmd("app.zoom_in", &[], "全局放大界面。"),
    cmd("app.zoom_out", &[], "全局缩小界面。"),
    cmd("app.zoom_reset", &[], "恢复默认界面缩放比例。"),
    cmd("app.update", &[], "重启并应用 Herdr 已下载的更新（终端保持在后台运行）。"),
    cmd("app.quit", &[], "退出应用程序（macOS）。"),
    cmd("app.hide", &[], "隐藏应用程序（macOS）。"),
    cmd("app.hide_others", &[], "隐藏其他应用程序（macOS）。"),
    cmd("app.show_all", &[], "显示所有应用程序（macOS）。"),
    cmd("window.close", &[], "关闭窗口（Linux 上直接退出）。"),
    cmd("window.minimize", &[], "最小化窗口。"),
    cmd("window.maximize", &[], "最大化窗口或恢复。"),
];

pub fn find(name: &str) -> Option<&'static Command> {
    COMMANDS.iter().find(|c| c.name == name)
}

/// A key's `name:value` shorthand: the value fills the first argument (a
/// number fills `index` when there is one).
pub fn shorthand(c: &Command, value: &str) -> Value {
    let index = value.parse::<u64>().ok().filter(|_| c.args.iter().any(|a| a.name == "index"));
    match (index, c.args.first()) {
        (Some(i), _) => json!({ "index": i }),
        (None, Some(a)) => json!({ a.name: value }),
        (None, None) => Value::Null,
    }
}

/// Reject misspelled arguments before a command can use its current-target default.
/// Null is the argument-free form used by keys and mods.
pub fn validate_args(c: &Command, args: &Value) -> Result<(), String> {
    let valid = if c.args.is_empty() { "none".into() } else { c.args.iter().map(|a| a.name).collect::<Vec<_>>().join(", ") };
    match args {
        Value::Null => Ok(()),
        Value::Object(args) => {
            let unknown: Vec<_> = args.keys().filter(|key| !c.args.iter().any(|a| a.name == key.as_str())).map(String::as_str).collect();
            if !unknown.is_empty() {
                return Err(format!("{}: unknown argument(s): {}; valid arguments: {valid}", c.name, unknown.join(", ")));
            }
            for (key, value) in args {
                if matches!(key.as_str(), "id" | "pane" | "space" | "terminal") && !value.is_string() && !value.is_null() {
                    return Err(format!("{}: argument {key} must be a string (or null to omit it)", c.name));
                }
            }
            Ok(())
        }
        _ => Err(format!("{}: arguments must be an object; valid arguments: {valid}", c.name)),
    }
}

/// The first required argument that's missing (an empty string counts as missing).
pub fn missing(c: &Command, args: &Value) -> Option<&'static str> {
    c.args.iter().filter(|a| a.required).map(|a| a.name).find(|k| match args.get(k) {
        None | Some(Value::Null) => true,
        Some(Value::String(s)) => s.trim().is_empty(),
        _ => false,
    })
}

/// The list as data, for `herdr-app commands` and mods.
pub fn listing() -> Value {
    Value::Array(
        COMMANDS
            .iter()
            .map(|c| {
                let args: Vec<Value> = c.args.iter().map(|a| json!({ "name": a.name, "required": a.required })).collect();
                json!({ "name": c.name, "args": args, "about": c.about })
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The guide agents read (written to the home folder) names every command.
    #[test]
    fn the_guide_names_every_command() {
        let guide = include_str!("../defaults/AGENTS.md");
        let missing: Vec<&str> = COMMANDS.iter().map(|c| c.name).filter(|n| !guide.contains(&format!("`{n}"))).collect();
        assert!(missing.is_empty(), "defaults/AGENTS.md (## Commands) doesn't name: {missing:?}");
    }

    #[test]
    fn unknown_arguments_are_rejected_for_every_command() {
        for c in COMMANDS {
            let error = validate_args(c, &json!({ "typo": "target" })).unwrap_err();
            assert!(error.contains(c.name), "{error}");
            assert!(error.contains("unknown argument(s): typo"), "{error}");
            assert!(error.contains("valid arguments:"), "{error}");
            assert!(validate_args(c, &Value::Null).is_ok());
            assert!(validate_args(c, &json!({})).is_ok());
            assert!(validate_args(c, &json!(["target"])).is_err());
            let args: serde_json::Map<String, Value> = c.args.iter().map(|a| (a.name.into(), json!("target"))).collect();
            assert!(validate_args(c, &Value::Object(args)).is_ok());
        }
        let close = find("space.close").unwrap();
        assert_eq!(validate_args(close, &json!({"space": "X", "confirmed": true})),
            Err("space.close: unknown argument(s): space; valid arguments: id, confirmed".into()));
        assert!(validate_args(close, &json!({"id": "X", "confirmed": true})).is_ok());
    }

    #[test]
    fn malformed_targets_cannot_fall_back_to_the_current_work() {
        for (name, arg) in [("tab.close", "id"), ("pane.close", "pane"), ("space.close", "id"), ("tab.new", "space"), ("terminal.show", "terminal")] {
            let c = find(name).unwrap();
            for value in [json!(123), json!(false), json!([]), json!({"id": "target"})] {
                let error = validate_args(c, &json!({arg: value})).unwrap_err();
                assert!(error.contains(name) && error.contains(arg), "{error}");
            }
            // Null keeps its existing omitted-argument meaning for optional targets.
            for value in [Value::Null, json!("target")] {
                assert!(validate_args(c, &json!({arg: value})).is_ok());
            }
        }
    }

    #[test]
    fn names_are_unique_and_shorthand_fills_the_first_argument() {
        let mut names: Vec<&str> = COMMANDS.iter().map(|c| c.name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), COMMANDS.len());
        assert_eq!(shorthand(find("tab.select").unwrap(), "3"), json!({ "index": 3 }));
        assert_eq!(shorthand(find("pane.split").unwrap(), "right"), json!({ "direction": "right" }));
        let move_ = find("space.move").unwrap();
        assert_eq!(missing(move_, &json!({ "id": " ", "index": 1 })), Some("id"));
        assert_eq!(missing(move_, &json!({ "id": "sp1", "index": 1 })), None);
        // Names are optional: none is Herdr's automatic one.
        assert_eq!(missing(find("space.new").unwrap(), &json!({})), None);
    }
}
