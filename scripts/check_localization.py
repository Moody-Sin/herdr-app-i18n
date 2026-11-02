from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
MODS = ROOT / "defaults" / "mods"
FRAME = ROOT / "defaults" / "frame"

CHINESE_PATTERN = re.compile(r"[\u4e00-\u9fff]")

def check_commands() -> list[str]:
    errors = []
    commands_file = SRC / "commands.rs"
    if not commands_file.exists():
        return ["src/commands.rs not found"]

    content = commands_file.read_text(encoding="utf-8")
    cmd_pattern = re.compile(r'cmd\(\s*"([^"]+)",\s*&\[[^\]]*\],\s*"((?:\\.|[^"\\])*)"\)', re.DOTALL)
    for match in cmd_pattern.finditer(content):
        name = match.group(1)
        about = match.group(2)
        if not CHINESE_PATTERN.search(about):
            errors.append(f"Command '{name}' description is not in Chinese: {about[:40]}...")
    return errors

def check_mods() -> list[str]:
    errors = []
    # Check key mod UI files for localized terms
    required_translations = {
        MODS / "agents" / "main.js": ["智能体", "按优先级", "按空间分组", "等待处理", "已完成", "运行中", "空闲"],
        MODS / "menu" / "main.js": ["向右分栏", "向下分栏", "新建标签页", "关闭", "新建空间"],
        MODS / "new-menu" / "main.js": ["终端", "启动智能体", "新建空间"],
        MODS / "confirm" / "main.js": ["取消", "确认"],
        MODS / "prompt" / "main.js": ["名称", "取消", "保存"],
        MODS / "files" / "main.js": ["文件", "正在加载文件…"],
        MODS / "changes" / "main.js": ["更改", "提交", "提交信息"],
        FRAME / "settings" / "main.js": ["外观", "键盘", "机器", "设置", "浅色", "深色"],
    }

    for path, terms in required_translations.items():
        if not path.exists():
            errors.append(f"File not found: {path.relative_to(ROOT)}")
            continue
        text = path.read_text(encoding="utf-8")
        for term in terms:
            if term not in text:
                errors.append(f"{path.relative_to(ROOT)} missing expected Chinese term '{term}'")

    return errors

def main() -> int:
    errors = []
    errors.extend(check_commands())
    errors.extend(check_mods())

    if errors:
        print(f"Found {len(errors)} localization issue(s):", file=sys.stderr)
        for err in errors:
            print(f"  - {err}", file=sys.stderr)
        return 1

    print("All localization checks passed!")
    return 0

if __name__ == "__main__":
    sys.exit(main())
