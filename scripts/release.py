#!/usr/bin/env python3
"""版本发布脚本 — 同步版本号、提交、打标签并推送

用法：
    # 先生成 release notes
    python scripts/gen_release_notes.py

    # 然后执行发布
    python scripts/release.py 0.1.1
    python scripts/release.py 1.0.0-beta.1
"""

import json
import os
import re
import subprocess
import sys


def run(cmd, check=True):
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    if check and result.returncode != 0:
        print(f"错误: {cmd}\n{result.stderr.strip()}")
        sys.exit(1)
    return result.stdout.strip()


def update_version(filepath, version):
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()

    if filepath.endswith("package.json"):
        new_content = re.sub(
            r'"version"\s*:\s*"[^"]*"',
            f'"version": "{version}"',
            content,
            count=1,
        )
    elif filepath.endswith("tauri.conf.json"):
        new_content = re.sub(
            r'"version"\s*:\s*"[^"]*"',
            f'"version": "{version}"',
            content,
            count=1,
        )
    else:
        return

    if new_content == content:
        print(f"  警告: {filepath} 版本号未变更")
        return

    with open(filepath, "w", encoding="utf-8") as f:
        f.write(new_content)
    print(f"  已更新 {filepath}")


def main():
    if len(sys.argv) != 2:
        print(f"用法: python {sys.argv[0]} <version>")
        print(f"示例: python {sys.argv[0]} 0.1.1")
        print(f"      python {sys.argv[0]} 1.0.0-beta.1")
        sys.exit(1)

    version = sys.argv[1].lstrip("v")
    tag = f"v{version}"

    if not re.match(r"^\d+\.\d+\.\d+(-[\w.]+)?$", version):
        print(f"错误: 版本号格式不正确: {version}，期望 MAJOR.MINOR.PATCH 或 MAJOR.MINOR.PATCH-tag")
        sys.exit(1)

    # 检查工作区是否干净
    status = run("git status --porcelain", check=False)
    if status:
        print("存在未提交的更改，请先提交或暂存：")
        print(status)
        sys.exit(1)

    # 检查 tag 是否已存在
    tags = run("git tag -l", check=False)
    if tag in tags.splitlines():
        print(f"标签 {tag} 已存在，请先删除：git tag -d {tag} && git push origin --delete {tag}")
        sys.exit(1)

    print(f"准备发布 {tag} ...")

    # 更新版本号
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    update_version(os.path.join(root, "package.json"), version)
    update_version(os.path.join(root, "src-tauri", "tauri.conf.json"), version)

    # 读取 release notes（如果存在）
    msg_file = os.path.join(root, "msg.txt")
    commit_msg = f"chore: bump version to {version}"
    if os.path.exists(msg_file):
        commit_msg = open(msg_file, "r", encoding="utf-8").read().strip()
        os.remove(msg_file)
        print("已读取并删除 msg.txt")
    else:
        print("未找到 msg.txt，使用默认 commit message")

    # 提交版本号变更
    run("git add package.json src-tauri/tauri.conf.json")
    run(f'git commit -m "{commit_msg}"')

    # 创建 tag
    run(f"git tag {tag}")

    # 推送
    print(f"推送代码和标签 {tag} ...")
    run("git push origin master")
    run(f"git push origin {tag}")

    print(f"\n发布完成！{tag}")
    print(f"查看: https://github.com/cicbyte/ktconsole/releases/tag/{tag}")


if __name__ == "__main__":
    main()
