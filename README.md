# Hexa-ISA

## **A Multi-Valued Instruction Set Architecture for Confidence-Aware and Approximate Computation**  
[![CC BY-SA 4.0](https://img.shields.io/badge/License-CC_BY--SA_4.0-lightgrey.svg)](LICENSE)  
![Compatible](docs/Hexa_ISA-compatible_badge.svg)

### Abstract

Contemporary binary instruction set architectures (ISAs) encode operations under a precision-centric assumption, leaving approximation tolerance, confidence, and fallback behavior unrepresentable at the instruction level. As a result, uncertainty and execution validity are reconstructed post hoc in software, obscuring intent and preventing explicit coordination across the ISA–firmware–hardware boundary—an increasingly acute limitation for relational reasoning systems and complex multi-scale simulations.
This paper introduces Hexa ISA, a hexadecimal opcode–based ISA framework that restores a missing semantic layer by decoupling logical intent from execution constraints. Hexa defines each instruction as a composite of an opcode-derived semantic domain and a meta-execution field specifying approximation allowance, confidence targets, and fallback preferences. By leveraging nibble-aligned radix-16 encoding, the framework enables orthogonal partitioning of semantic operator families (e.g., modal, relational/quantifier, set-theoretic, and equivalence domains) while remaining explicitly emulatable on existing binary hardware via firmware-level interpretation.
Hexa reframes ISA extension as a minimal representational upgrade: it introduces explicit contract-based execution semantics without requiring new hardware paradigms or claiming immediate performance gains. The proposed semantic map and contract model provide a stable interface for heterogeneous execution substrates and support formal reasoning at the opcode level independent of physical realization.

---

## 📄 Full Paper
[[Hexa ISA.pdf](docs/paper/Hexa%20ISA.pdf)]

**DOI:** [10.36227/techrxiv.1374820](10.36227/techrxiv.1374820)

## 📖 Full Specification
[[Hexa_ISA-Specification.md](docs/Hexa_ISA-Specification.md)]

## 🛠️ Core Opcodes (FIXED v1.0)
[[ISA_Table.md](docs/spec/ISA_Table.md)]

## 🏆 Compatibility
**Hexa-ISA Compatible** implementations MUST conform to Core v1.0  
[[Use this badge → Hexa_ISA-compatible_badge.svg](docs/Hexa_ISA-compatible_badge.svg)]
