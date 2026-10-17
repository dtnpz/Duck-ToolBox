"""Read-only fork metadata and diagnostic tests (no Android device or network needed)."""
import json
from pathlib import Path
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[1]


def props():
    return dict(
        line.split("=", 1)
        for line in (ROOT / "module" / "module.prop").read_text().splitlines()
        if "=" in line
    )


class ForkMetadataTest(unittest.TestCase):
    def test_fork_version_and_updates_agree(self):
        module = props()
        update = json.loads((ROOT / "update.json").read_text())
        self.assertEqual(module["id"], "duck-toolbox")
        self.assertEqual(module["version"], update["version"])
        self.assertEqual(int(module["versionCode"]), update["versionCode"])
        self.assertIn("dtnpz/Duck-ToolBox", module["updateJson"])
        self.assertIn("dtnpz/Duck-ToolBox", update["zipUrl"])
        self.assertIn(update["version"], update["zipUrl"])

    def test_shell_parses(self):
        script = ROOT / "module" / "bin" / "joyeuse-diag.sh"
        subprocess.run(["sh", "-n", str(script)], check=True)
        content = script.read_text()
        self.assertIn("rkp-device status", content)
        self.assertNotIn("rkp-device provision", content)
        self.assertNotIn("--acknowledge-limitations", content)

    def test_upstream_attribution_remains(self):
        license_text = (ROOT / "LICENSE").read_text()
        self.assertIn("Copyright (c) 2026 Eltavine", license_text)
        self.assertIn("MIT License", license_text)


if __name__ == "__main__":
    unittest.main()
