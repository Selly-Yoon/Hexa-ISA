"""
Hexa-ISA Contract Resolution Visualizer
hexa-trace 출력(trace.jsonl)을 읽어 시각화
"""
import json
import sys
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from pathlib import Path

def load_trace(path: str) -> list[dict]:
    events = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line:
                events.append(json.loads(line))
    return events

def plot_contract_resolution(events: list[dict]):
    evaluated = [e for e in events if e.get("event") == "ContractEvaluated"]
    if not evaluated:
        print("ContractEvaluated 이벤트 없음")
        return

    df = pd.DataFrame(evaluated)

    fig, axes = plt.subplots(1, 2, figsize=(12, 5))
    fig.suptitle("Hexa-ISA Contract Resolution", fontsize=14)

    # 1. confidence 달성률
    met_counts = df["target_met"].value_counts()
    axes[0].pie(
        met_counts,
        labels=["Met", "Not Met"],
        autopct="%1.1f%%",
        colors=["#4CAF50", "#F44336"]
    )
    axes[0].set_title("Contract Target Met Rate")

    # 2. opcode별 achieved confidence 분포
    sns.boxplot(data=df, x="opcode", y="confidence_achieved", ax=axes[1])
    axes[1].set_title("Confidence Achieved by Opcode")
    axes[1].tick_params(axis="x", rotation=45)

    plt.tight_layout()
    plt.savefig("contract_resolution.png", dpi=150)
    print("contract_resolution.png 저장 완료")

def plot_fallback_distribution(events: list[dict]):
    fallbacks = [e for e in events if e.get("event") == "FallbackTriggered"]
    if not fallbacks:
        print("FallbackTriggered 이벤트 없음")
        return

    df = pd.DataFrame(fallbacks)
    plt.figure(figsize=(7, 4))
    sns.countplot(data=df, x="policy", palette="Set2")
    plt.title("Fallback Policy Distribution")
    plt.tight_layout()
    plt.savefig("fallback_distribution.png", dpi=150)
    print("fallback_distribution.png 저장 완료")

if __name__ == "__main__":
    path = sys.argv[1] if len(sys.argv) > 1 else "trace.jsonl"
    if not Path(path).exists():
        print(f"파일 없음: {path}")
        sys.exit(1)

    events = load_trace(path)
    print(f"총 {len(events)}개 이벤트 로드")
    plot_contract_resolution(events)
    plot_fallback_distribution(events)
