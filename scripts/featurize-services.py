# SPDX-License-Identifier: CC0-1.0

"""Make every service an opt-in Cargo feature of the router and the embedded facade.

EmulatorCore re-renders core/router/Cargo.toml and core/router/src/lib.rs on every service add,
which drops the features; run this again afterwards. It rewrites those two files and
core/embedded/Cargo.toml from `cargo metadata`, and is idempotent.

Usage (from the repository root): python3 scripts/featurize-services.py
"""
import json, re, subprocess
from pathlib import Path

root = Path(__file__).resolve().parent.parent
meta = json.loads(subprocess.run(['cargo', 'metadata', '--format-version', '1', '--no-deps'],
                                 cwd=root, check=True, capture_output=True).stdout)
pk = {p['name']: p for p in meta['packages']}
PREFIX = 'service-com-amazonaws-'
svcs = sorted(n for n in pk if n.startswith(PREFIX))
feat = {n: n[len(PREFIX):].split('-')[0] for n in svcs}
assert len(set(feat.values())) == len(svcs), 'feature names collide'
crate_feat = {n.replace('-', '_'): f for n, f in feat.items()}

def hard_deps(n):  # normal (non-dev, non-build) deps on other service crates
    return sorted({d['name'] for d in pk[n]['dependencies']
                   if d['kind'] is None and d['name'] in feat and d['name'] != n})

# ---- router/Cargo.toml
rt = root / 'core/router/Cargo.toml'
txt = rt.read_text()
txt = re.sub(r'^(service-com-amazonaws-[\w-]+) = \{ workspace = true \}$',
             r'\1 = { workspace = true, optional = true }', txt, flags=re.M)
lines = ['', '# One feature per service, named for its Smithy namespace (the `aws-sdk-<name>` name).',
         '# Enabling a service also enables every service its crate links (e.g. `s3` pulls',
         '# `kms`, `sts`, `cloudwatch`), so what is compiled is exactly what is mounted.',
         '[features]', 'default = ["all"]',
         'all = [' + ', '.join(f'"{feat[n]}"' for n in svcs) + ']']
for n in svcs:
    items = [f'"dep:{n}"'] + [f'"{feat[d]}"' for d in hard_deps(n)]
    lines.append(f'{feat[n]} = [{", ".join(items)}]')
tests = ['', '# Cross-service suites and the model sweep name services directly, so they need the full set.',
         '[[test]]', 'name = "all"', 'path = "tests/all.rs"', 'required-features = ["all"]', '',
         '[[test]]', 'name = "emulator_core"', 'path = "tests/emulator_core/main.rs"', 'required-features = ["all"]']
txt = re.sub(r'\n\n# One feature per service.*?(?=\n\n\[build-dependencies\])', '', txt, flags=re.S)  # idempotence
txt = txt.replace('\n[build-dependencies]', '\n'.join(lines + tests) + '\n\n[build-dependencies]', 1)
rt.write_text(txt)

# ---- router/src/lib.rs: cfg-gate each registration
lr = root / 'core/router/src/lib.rs'
src = lr.read_text()
def gate(m):
    indent, crate = m.group(1), m.group(2)
    return f'{indent}#[cfg(feature = "{crate_feat[crate]}")]\n{m.group(0)}'
src = re.sub(r'^\s*#\[cfg\(feature = "[\w]+"\)\]\n', '', src, flags=re.M)
src = re.sub(r'^( +)router\.register_service\((service_com_amazonaws_\w+)::', gate, src, flags=re.M)
# A chunk whose services are all disabled leaves `router` unmutated and `engine` unused.
src = src.replace('#[inline(never)]\nfn mount_chunk', '#[inline(never)]\n#[allow(unused_mut, unused_variables)]\nfn mount_chunk')
src = src.replace('#[allow(unused_mut, unused_variables)]\n#[allow(unused_mut, unused_variables)]\n', '#[allow(unused_mut, unused_variables)]\n')
lr.write_text(src)

# ---- embedded/Cargo.toml: forward every feature
et = root / 'core/embedded/Cargo.toml'
e = et.read_text()
e = e.replace('aws-emulator-router = { path = "../router" }',
              'aws-emulator-router = { path = "../router", default-features = false }')
e = re.sub(r'\n\n# Forwarded to the router.*?(?=\n\n\[lints\])', '', e, flags=re.S)
fl = ['', '# Forwarded to the router: pick services with `default-features = false, features = ["s3", ...]`.',
      '[features]', 'default = ["all"]', 'all = ["aws-emulator-router/all"]']
fl += [f'{feat[n]} = ["aws-emulator-router/{feat[n]}"]' for n in svcs]
e = e.replace('\n[lints]', '\n'.join(fl) + '\n\n[lints]', 1)
et.write_text(e)
print(f'{len(svcs)} service features written')
