import json
import sys
from pathlib import Path

usage = "Usage: generate_packets.py <protocol.json> <out_file> [old_packets.json]"

if not (3 <= len(sys.argv) <= 4):
    print(usage)
    sys.exit(1)

protocol_path = Path(sys.argv[1])
out_path = Path(sys.argv[2])
old_path = Path(sys.argv[3]) if len(sys.argv) == 4 else None

with protocol_path.open() as f:
    protocol = json.load(f)

old_index = {}
if old_path and old_path.exists():
    with old_path.open() as f:
        old = json.load(f)
    for state, dirs in old.items():
        old_index[state] = {}
        for bound, pkts in dirs.items():
            old_index[state][bound] = {v["protocol_id"]: k.split(":", 1)[1] for k, v in pkts.items()}

STATE_MAP = {
    "handshake": "handshaking",
    "login": "login",
    "play": "play",
    "status": "status",
}

BOUND_MAP = {
    "clientbound": "toClient",
    "serverbound": "toServer",
}

out = {
    "configuration": {"clientbound": {}, "serverbound": {}},
}

def extract(state_key):
    proto_state = protocol.get(STATE_MAP[state_key], {})
    out_state = {}
    for bound_out, bound_in in BOUND_MAP.items():
        try:
            fields = proto_state[bound_in]["types"]["packet"][1]
            name_field = next(f for f in fields if f.get("name") == "name")
            mappings = name_field["type"][1]["mappings"]
            result = {}
            for pid_hex, default_name in mappings.items():
                pid = int(pid_hex, 16)
                name = old_index.get(state_key, {}).get(bound_out, {}).get(pid, default_name)
                result[f"minecraft:{name}"] = {"protocol_id": pid}
            out_state[bound_out] = result
        except Exception:
            out_state[bound_out] = {}
    out[state_key] = out_state

for s in ["handshake", "login", "play", "status"]:
    extract(s)

with out_path.open('w') as f:
    json.dump(out, f, indent=2)
    f.write('\n')
