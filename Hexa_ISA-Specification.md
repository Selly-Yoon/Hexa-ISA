# Hexa-ISA Core Specification v1.0

**Hye-Eun Yoon (Selly)**  
*Independent Researcher*  
*December 2025*  

---

## License

[![License: CC BY-SA 4.0](https://img.shields.io/badge/License-CC_BY--SA_4.0-lightgrey.svg)](https://creativecommons.org/licenses/by-sa/4.0/)

**Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0)**

- Free to use, modify, distribute implementations
- Must credit "Hexa-ISA Core Specification v1.0 by Hye-Eun Yoon (Selly)"  
- Derivatives must use same license

---

## Compatibility Policy

**Hexa-ISA Compatible** implementations **MUST** conform to Core Specification v1.0.

| Category | Opcode Range | Policy | Examples |
|----------|--------------|--------|----------|
| **🛡️ Legacy** | `0x00-0x7F` | **FIXED** | x86/ARM mapped |
| **🔒 Core Semantic** | **`0x80-0xDF`** | **FIXED v1.0** | Modal, Quantifier, Set, etc. |
| **🟢 Extensions** | `0xE0-0xFF` | **FREE** | Custom prefixes allowed |
| **Meta-Field** | Byte 2 | **FIXED** | Approx/Confidence/Fallback |

**Violation** = **Not Hexa-ISA Compatible**

---

## Overview

**Total Opcodes**: 256 (8-bit)  
**Instruction Format**: 16-bit minimum (Opcode + Meta-field)  
**Binary Emulation**: Full x86-64/ARM compatibility via firmware

```
┌─────────────┬─────────────┐
│ Opcode (8b) │ Meta (8b)   │
│ 0x80-0xDF   │ Approx/Conf │
└─────────────┴─────────────┘
```

---

## Core Opcode Map (FIXED v1.0)

| Hex Range | Family | Canonical Operators | Notes |
|-----------|--------|---------------------|-------|
| `0x00-0x7F` | **Legacy** | MOV, ADD, SUB, MUL, JMP... | x86-64/ARM mapped [10] |
| `0x80-0x8F` | **Modal** | NEC, POS, TAUT, CONT... | Necessity/Possibility |
| `0x90-0x9F` | **Quantifier** | ALL, EXIST, NEXIST... | ∀, ∃, scoped relations |
| `0xA0-0xAF` | **Comparison** | CMPMLT, CMPMGT, FUZ... | Total/partial orders |
| `0xB0-0xBF` | **Set-Theoretic** | SETSUB, SETINT, UNI... | ⊆, ∩, ∪ operations |
| `0xC0-0xCF` | **Equivalence** | EQID, EQSIM, NEQSIM... | Identity vs similarity |
| `0xD0-0xDF` | **Quantum Control** | QPREP, QSUP, QENT... | Abstract contracts |
| `0xE0-0xEF` | **Reserved** | Extensions | **FREE** prefix space |
| `0xF0-0xFF` | **Escape** | Vendor/Custom | **FREE** negotiation |

**Table defines Hexa-ISA Core Specification v1.0** - stable reference for compatible implementations.

---

## Detailed Opcode Reference

### Legacy Binary (0x00-0x7F)  
**FIXED**: Standard x86-64/ARM mapping. Native execution.

### Modal Logic (0x80-0x8F)
| Hex | Mnemonic | Semantics | Emulation |
|-----|----------|-----------|-----------|
| 0x80 | `NEC` | Necessity (rs=TRUE) | Strict check |
| 0x81 | `POS` | Possibility (!FALSE) | Non-zero |
| 0x82 | `TAUT` | Tautology | MOV 1 |
| 0x83 | `CONT` | Contradiction | MOV 0 |

### Quantifier Relational (0x90-0x9F)
| Hex | Mnemonic | Semantics | Emulation |
|-----|----------|-----------|-----------|
| 0x90 | `ALL` | ∀ (all match) | Loop check |
| 0x91 | `EXIST` | ∃ (exists) | Early exit |
| 0x92 | `NEXIST` | ¬∃ | Full scan |

*(Full 256 opcode table in `opcodes.md`)*

---

## Meta-Execution Field (Byte 2) **FIXED**

| Bits | Field | Values | Description |
|------|-------|--------|-------------|
| 7-6 | **Approx Mode** | `00`: Precise<br>`01`: Allowed<br>`10`: Hybrid | Approximation policy |
| 5-4 | **Fallback** | `00`: Abort<br>`01`: Safe<br>`10`: Retry | Failure handling |
| 3-0 | **Confidence** | `0000`: Don't care<br>`1111`: 99.9% | Target level |

**Example**: `0xC231` = EQSIM (0xC2) + High Confidence + Approx OK (0x31)

---

## Extension Guidelines **FREE AREA**

**Prefix-based extensions** in `0xE0-0xFF`:

```
1. EXTPFX (0xE0): Extend meta-field width
2. ESCVND (0xF0): Vendor custom accelerators  
3. ESCLNG (0xFF): Variable-length instructions
```

**Examples**:
- AI workloads: Custom fuzzy operators
- Quantum: Extended control flow  
- Vendor: Proprietary optimizations

---

## Hexa-ISA Compatible Badge

![zmgau0gkkmemjsrl37h](https://github.com/user-attachments/assets/c8a385bc-8b65-402d-a60b-a04aa1fffed0)

**Usage**: Implementations passing Core v1.0 conformance tests.

---

## Reference Implementation

- **Emulator**: [TBD]
- **Compiler Backend**: LLVM target planned
- **Firmware**: Microcode examples forthcoming

**Conformance Tests**: Coming in v1.1

---

## Author & Citation

**Hye-Eun Yoon**  
Independent Researcher  
`selly@selly.org`

```
@misc{hexa-isa-2025,
  author = {Yoon, Hye-Eun (Selly)},
  title = {Hexa-ISA Core Specification v1.0},
  year = {2025},
  doi = {10.xxxx/hexa-isa-v1.0}
}
```

**Repository**: [[github.com/yourusername/hexa-isa-spec](https://github.com/Selly-Yoon/Hexa-ISA)]
**Paper**: [Submission to IEEE ToC / techrxiv forthcoming]

---

*Copyright © 2025 Hye-Eun Yoon (Selly).
```
