## opcodes.md

# Hexa-ISA Core Opcodes v1.0 (FIXED)

**Total: 256 opcodes (8-bit)** | **Core Semantic: 0x80-0xDF (96 FIXED)**

## Opcode Partition (Hexa-ISA Compatible REQUIRED)

| Range | Category | Policy | Examples |
|-------|----------|--------|----------|
| `0x00-0x7F` | 🛡️ Legacy | **FIXED** | x86/ARM mapped (MOV/ADD/JMP) |
| `0x80-0xDF` | 🔒 Core Semantic | **FIXED v1.0** | Modal/Quantifier/Set/Quantum |
| `0xE0-0xFF` | 🟢 Extensions | **FREE** | Vendor/Custom prefixes |

## Core Opcode Map (Table I)

| Hex | Family | Canonical Operators | Notes |
|-----|--------|-------------------|-------|
| `0x00-0x7F` | **Legacy Binary** | `MOV, ADD, SUB, MUL, JMP...` | x86-64/ARM mapped [10] |
| `0x80-0x8F` | **Modal** | `NEC, POS, TAUT, CONT...` | Necessity/Possibility |
| `0x90-0x9F` | **Quantifier** | `ALL, EXIST, NEXIST...` | ∀, ∃, scoped relations |
| `0xA0-0xAF` | **Comparison** | `CMPMLT, CMPMGT, FUZ...` | Total/partial orders |
| `0xB0-0xBF` | **Set-Theoretic** | `SETSUB, SETINT, UNI...` | ⊆, ∩, ∪ operations |
| `0xC0-0xCF` | **Equivalence** | `EQID, EQSIM, NEQSIM...` | Identity vs similarity |
| `0xD0-0xDF` | **Quantum Control** | `QPREP, QSUP, QENT...` | Abstract contracts |

## Detailed Reference (Core v1.0)

### Modal Logic (0x80-0x8F)
| Hex | Mnemonic | Semantics | Emulation |
|-----|----------|-----------|-----------|
| `0x80` | `NEC` | Necessity (rs=TRUE) | Strict check |
| `0x81` | `POS` | Possibility (!FALSE) | Non-zero |
| `0x82` | `TAUT` | Tautology | `MOV 1` |
| `0x83` | `CONT` | Contradiction | `MOV 0` |
| `0x84` | `IMP` | Implication | `CMPNEZ` |
| `0x85` | `NIMP` | Non-Implication | `CMPEQZ` |
| `0x86` | `REV` | Reverse Polish | Stack swap |
| `0x87` | `OBL` | Obligation | `NEC + CONF` |

### Quantifier Relational (0x90-0x9F)
| Hex | Mnemonic | Semantics | Emulation |
|-----|----------|-----------|-----------|
| `0x90` | `ALL` | ∀ (all match) | Loop check |
| `0x91` | `EXIST` | ∃ (exists) | Early exit |
| `0x92` | `NEXIST` | ¬∃ | Full scan |
| `0x93` | `UNIQUE` | ∃! | Count=1 |
| `0x94` | `MANY` | ∃≥2 | Count≥2 |
| `0x95` | `NONE` | ∀FALSE | All zero |

### Set-Theoretic (0xB0-0xBF)
| Hex | Mnemonic | Semantics | Emulation |
|-----|----------|-----------|-----------|
| `0xB0` | `SETSUB` | ⊆ (subset) | Bitwise AND |
| `0xB1` | `SETSUP` | ⊇ (superset) | Bitwise OR |
| `0xB2` | `SETINT` | ∩ (intersection) | AND |
| `0xB3` | `SETUNI` | ∪ (union) | OR |
| `0xB4` | `SETDIFF` | \ (difference) | XOR |

**Full 256 opcode table + Meta-field examples in [Hexa_ISA-Specification.md](Hexa_ISA-Specification.md)**

## Meta-Execution Field (Byte 2 - FIXED Format)
```
Bits 7-6: Approx Mode (00=Precise, 01=Allowed, 10=Hybrid)
Bits 5-4: Fallback   (00=Abort, 01=Safe, 10=Retry)
Bits 3-0: Confidence (0000=Don't care, 1111=99.9%)
```
**Example: `0xC231` = EQSIM(0xC2) + High Conf + Approx OK**
