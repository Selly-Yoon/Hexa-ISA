"""
Contract 충족률 통계 출력
"""
import json
import sys
from collections import Counter

def main(path: str):
    events = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line:
                events.append(json.loads(line))

    evaluated = [e for e in events if e.get("event") == "ContractEvaluated"]
    fallbacks  = [e for e in events if e.get("event") == "FallbackTriggered"]
    violations = [e for e in events if e.get("event") == "ContractViolation"]

    total = len(evaluated)
    met   = sum(1 for e in evaluated if e.get("target_met"))

    print("=== Hexa-ISA Contract Resolution Stats ===")
    print(f"Total instructions evaluated : {total}")
    print(f"Contract met                 : {met} ({met/total*100:.1f}%)" if total else "")
    print(f"Fallback triggered           : {len(fallbacks)}")
    print(f"Contract violations          : {len(violations)}")

    if fallbacks:
        dist = Counter(e["policy"] for e in fallbacks)
        print("\nFallback distribution:")
        for k, v in dist.items():
            print(f"  {k}: {v}")

if __name__ == "__main__":
    path = sys.argv[1] if len(sys.argv) > 1 else "trace.jsonl"
    main(path)
