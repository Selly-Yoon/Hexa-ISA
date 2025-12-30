## ISA_Table.md

# Hexa-ISA Full Table v1.0 (FIXED)

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

## Complete Reference Table

| Hex Code | Mnemonic | Operands | Semantic Definition | Binary Emulation / Fallback Logic | Family |
|----------|----------|----------|---------------------|-----------------------------------|--------|
| `0x00-0x7F` | `(LEGACY)` | - | Legacy Binary Region (x86-64/ARM Mapped) | Native Execution | 0. Legacy |
| `0x80` | `NEC` | `rd, rs` | Necessity (□): Assert rs is necessarily true | Check `rs == TRUE` (Strict) | 1. Modal (0x8) |
| `0x81` | `POS` | `rd, rs` | Possibility (◇): Assert rs is possibly true | Check `rs != FALSE` (Non-zero) | 1. Modal (0x8) |
| `0x82` | `TAUT` | `rd` | Tautology (⊤): Load absolute logical Truth | `MOV rd, 1` (Max Confidence) | 1. Modal (0x8) |
| `0x83` | `CONT` | `rd` | Contradiction (⊥): Load absolute logical False | `MOV rd, 0` (Max Confidence) | 1. Modal (0x8) |
| `0x84` | `MOD_AND` | `rd, rs1, rs2` | Modal AND: True iff rs1 AND rs2 are necessary | Bitwise AND (Strict prop) | 1. Modal (0x8) |
| `0x85` | `MOD_OR` | `rd, rs1, rs2` | Modal OR: True iff rs1 OR rs2 is possible | Bitwise OR (Relaxed prop) | 1. Modal (0x8) |
| `0x86` | `MOD_IMP` | `rd, rs1, rs2` | Strict Implication (⊸): rs1 necessarily implies rs2 | `(!rs1) OR rs2` check | 1. Modal (0x8) |
| `0x90` | `ALL` | `rd, ptr, len` | For All (∀): True if all elements match | Loop; `FALSE` on mismatch | 2. Quantifier (0x9) |
| `0x91` | `EXIST` | `rd, ptr, len` | Exists (∃): True if >= 1 element matches | Loop; `TRUE` on match | 2. Quantifier (0x9) |
| `0x92` | `NEXIST` | `rd, ptr, len` | Not Exists (∄): True if 0 elements match | Loop; `FALSE` on match | 2. Quantifier (0x9) |
| `0x93` | `EXIST_U` | `rd, ptr, len` | Unique (∃!): True if exactly 1 element matches | Counter loop; `count == 1` | 2. Quantifier (0x9) |
| `0x94` | `REL_MAP` | `rd, r1, r2` | Relation Map: Check relation R(r1, r2) | Function pointer call | 2. Quantifier (0x9) |
| `0xA0` | `CMP_MLT` | `rd, r1, r2` | Much Less (≪): r1 << r2 (Order of mag.) | `(r2 - r1) > Threshold` | 3. Comparison (0xA) |
| `0xA1` | `CMP_MGT` | `rd, r1, r2` | Much Greater (≫): r1 >> r2 | `(r1 - r2) > Threshold` | 3. Comparison (0xA) |
| `0xA2` | `CMP_INC` | `rd, r1, r2` | Incomparable (≹): Undefined order | Check NaN or disjoint | 3. Comparison (0xA) |
| `0xA3` | `CMP_FUZ` | `rd, r1, r2` | Fuzzy Compare: Probabilistic comparison | Return probability score | 3. Comparison (0xA) |
| `0xB0` | `SET_SUB` | `rd, r1, r2` | Subset (⊆): r1 is subset of r2 | `(r1 & r2) == r1` | 4. Set-Theory (0xB) |
| `0xB1` | `SET_SUP` | `rd, r1, r2` | Superset (⊇): r1 is superset of r2 | `(r1 & r2) == r2` | 4. Set-Theory (0xB) |
| `0xB2` | `SET_INT` | `rd, r1, r2` | Intersection (∩): r1 AND r2 | Bitwise AND | 4. Set-Theory (0xB) |
| `0xB3` | `SET_UNI` | `rd, r1, r2` | Union (∪): r1 OR r2 | Bitwise OR | 4. Set-Theory (0xB) |
| `0xB4` | `SET_DIF` | `rd, r1, r2` | Difference (∖): r1 MINUS r2 | `r1 & (~r2)` | 4. Set-Theory (0xB) |
| `0xB5` | `SET_SYM` | `rd, r1, r2` | Sym. Diff (△): XOR set | Bitwise XOR | 4. Set-Theory (0xB) |
| `0xB6` | `SET_MEM` | `rd, el, set` | Membership (∈): element in set | Bit test / Hash lookup | 4. Set-Theory (0xB) |
| `0xC0` | `EQ_ID` | `rd, r1, r2` | Identical (≡): Strict identity (Value+Type) | Standard `==` check | 5. Equiv (0xC) |
| `0xC1` | `EQ_ISO` | `rd, r1, r2` | Isomorphic (≅): Structural equivalence | Graph/Structure compare | 5. Equiv (0xC) |
| `0xC2` | `EQ_SIM` | `rd, r1, r2` | Similar (∼): Metric distance < epsilon | `dist < Meta.Threshold` | 5. Equiv (0xC) |
| `0xC3` | `NEQ_SIM` | `rd, r1, r2` | Not Similar (≁): Metric distance >= epsilon | `dist >= Meta.Threshold` | 5. Equiv (0xC) |
| `0xD0` | `Q_PREP` | `rd, st` | Prepare: Init qubit/state to 0 or 1 | Init prob vector | 6. Quantum (0xD) |
| `0xD1` | `Q_SUP` | `rd` | Superpose (H): Apply Hadamard | Set prob to 0.5 (Uniform) | 6. Quantum (0xD) |
| `0xD2` | `Q_ENT` | `r1, r2` | Entangle (CNOT): Correlate states | Bind vars in prob graph | 6. Quantum (0xD) |
| `0xD3` | `Q_MEAS` | `rd, r1` | Measure: Collapse state to binary | Random sample | 6. Quantum (0xD) |
| `0xE0` | `EXT_PFX` | `imm` | Prefix: Extend Meta-field width | Firmware decode state | 7. Extension (0xE) |
| `0xF0` | `ESC_VND` | `imm` | Vendor Escape: Custom accelerator func | Driver passthrough | 7. Extension (0xF) |
| `0xFF` | `ESC_LNG` | `len` | Long Instruction: Variable length follows | Multi-byte fetch | 7. Extension (0xF) |

## Meta-Field Structure (Byte 2 - FIXED)

| Bit Range | Field Name | Description | Values (Example) |
|-----------|------------|-------------|------------------|
| `[7:6]` | Approx Mode | Approximation operation permission | `00`: Precise Only<br>`01`: Approx Allowed<br>`10`: Hybrid<br>`11`: Reserved |
| `[5:4]` | Fallback Policy | Response when targets are not met | `00`: Abort (Error)<br>`01`: Reduce (Safe Val)<br>`10`: Retry<br>`11`: Escalate (Slow) |
| `[3:0]` | Confidence Target | Target Confidence Level (4-bit) | `0000`: Don't Care<br>`1000`: >50%<br>`1100`: >90%<br>`1111`: >99.9% (Strict) |

**Example: `0xC231` = EQ_SIM(0xC2) + High Conf + Approx OK**
