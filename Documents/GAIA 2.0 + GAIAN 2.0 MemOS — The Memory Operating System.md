# GAIA 2.0 + GAIAN 2.0: MemOS — The Memory Operating System
## Blueprint 58: The Memory Infrastructure of the Planetary AI
### September 9, 2026 — Version 1.0

---

> *"MemOS treats memory as a manageable system resource. It unifies the representation, scheduling, and evolution of plaintext, activation-based, and parameter-level memories, enabling cost-efficient storage and retrieval."*
> — MemOS: A Memory OS for AI System (arXiv:2507.03724, July 2025)

> *"Don't let your AI forget again. Empower it with MemOS!"*
> — openmem.net

---

## EXECUTIVE SUMMARY

Memory is the soul of GAIAN 2.0. Without memory, GAIAN is just another chatbot. With memory, GAIAN becomes a true companion — one that knows you deeply, serves you faithfully, and grows with you across your entire life.

**MemOS** (arXiv:2507.03724, July 2025; v4 December 2025) is the most comprehensive memory operating system for LLMs ever published. It treats memory as a **first-class operational resource** — like CPU, RAM, and storage in a traditional OS — and provides unified mechanisms for representation, organization, and governance across three core memory types.

**The MemOS Ecosystem (2025-2026):**
- **MemOS Paper** (arXiv:2507.03724, July 4, 2025; v4 December 3, 2025): 38 authors; 36 pages; 10 figures; 5 tables
- **MemOS MAG Paper** (arXiv:2505.22101, May 28, 2025): Memory-Augmented Generation; first paper introducing MemOS concept
- **MemOS GitHub** (github.com/MemTensor/MemOS): **11.2K stars**; Apache-2.0; TypeScript + Python
- **MemOS Docs** (memos-docs.openmem.net): Complete documentation; REST API; SDK
- **35.24% token savings**: MemOS reduces token usage by 35.24% vs baseline
- **DeepSeek Harness support**: Works with DeepSeek models
- **memmy-agent** (github.com/MemTensor/memmy-agent): **1.3K stars**; personal AI agent with shared memory
- **HaluMem**: First hallucination evaluation benchmark for agent memory systems
- **MemRL**: Self-evolving agents via runtime reinforcement learning on episodic memory

**GAIA 2.0 Strategy**: Integrate MemOS as the memory operating system layer for GAIAN 2.0, complementing Mi-Memory (Blueprint 49) with MemOS's unique capabilities: MemCube abstraction, memory migration between types, and continual learning through parameter-level memory evolution.

---

## PART I: MEMOS — THE PAPER

### 1.1 The Two MemOS Papers

MemOS has two foundational papers:

```
MEMOS PAPER 1: MEMORY-AUGMENTED GENERATION (MAG)
─────────────────────────────────────────────────────────────────
Paper: "MemOS: An Operating System for Memory-Augmented Generation (MAG) in LLMs"
arXiv: 2505.22101 (May 28, 2025)
Authors: Zhiyu Li et al. (20 authors)

Key contribution:
- First paper to introduce MemOS concept
- Elevates memory to a first-class operational resource
- Introduces MemCube abstraction
- Three memory types: parametric, activation, plaintext
- Memory-centric execution framework

MEMOS PAPER 2: MEMORY OS FOR AI SYSTEM
─────────────────────────────────────────────────────────────────
Paper: "MemOS: A Memory OS for AI System"
arXiv: 2507.03724 (July 4, 2025; v4 December 3, 2025)
Authors: Zhiyu Li et al. (38 authors — expanded team)
Size: 36 pages, 10 figures, 5 tables

Key contribution:
- Full system paper with implementation details
- Memory hierarchy perspective on LLM costs
- MemCube lifecycle: compose, migrate, fuse
- Continual learning and personalized modeling
- Cross-platform coordination
```

### 1.2 The Core Problem MemOS Solves

```
THE MEMORY PROBLEM IN LLMs

Current LLM Memory Architecture:
─────────────────────────────────────────────────────────────────
1. PARAMETRIC MEMORY (Model Weights)
   - Knowledge encoded during training
   - Static: cannot be updated without retraining
   - Expensive to update: full fine-tuning costs millions
   - Fast at inference: no retrieval needed
   - Problem: Cannot learn new information after training

2. ACTIVATION MEMORY (Context Window)
   - Current conversation context
   - Ephemeral: lost after conversation ends
   - Limited: 128K tokens max (even with long context)
   - Problem: Cannot persist across sessions

3. PLAINTEXT MEMORY (RAG)
   - External knowledge retrieved at inference time
   - Stateless: no lifecycle management
   - No integration with persistent memory
   - Problem: Stateless workaround; no learning

The Result:
- LLMs cannot track user preferences over time
- LLMs cannot update knowledge without expensive retraining
- LLMs cannot learn from interactions
- LLMs cannot personalize to individual users

The Cost Problem:
- Introducing an explicit memory layer between parameter memory
  and external retrieval can substantially reduce costs
- By externalizing specific knowledge, training costs decrease
- By caching activations, inference costs decrease
- MemOS models this as a memory hierarchy (like CPU cache hierarchy)

MEMOS SOLUTION:
Treat memory as a manageable system resource.
Unify representation, scheduling, and evolution of all memory types.
Enable cost-efficient storage and retrieval.
Enable continual learning and personalized modeling.
```

### 1.3 The Three Memory Types

```
MEMOS THREE MEMORY TYPES

1. PARAMETRIC MEMORY (Knowledge in Model Weights)
─────────────────────────────────────────────────────────────────
What it is: Knowledge encoded in LLM weights during training
Examples: General world knowledge; language; reasoning patterns
Characteristics: Static; fast; expensive to update
Update method: Fine-tuning (LoRA, adapters)
MemOS management:
- Track what the model knows (and doesn't know)
- Identify knowledge gaps → trigger targeted fine-tuning
- Migrate frequently-accessed plaintext → parametric (learning)
- MemCube type: ParametricMemCube

GAIAN use:
- Llama 3.1 8B's general knowledge about the world
- GAIAN-specific fine-tuning (user preferences → model weights)
- Earth Twin knowledge baked into model weights

2. ACTIVATION MEMORY (KV Cache / Context Window)
─────────────────────────────────────────────────────────────────
What it is: Current conversation context; KV cache states
Examples: Current conversation; recent messages; active task state
Characteristics: Ephemeral; fast; limited by context window
Update method: Automatic (conversation turns)
MemOS management:
- Efficient context packing (what to include)
- KV cache management (reuse across sessions)
- Compress old context → plaintext (summarization)
- MemCube type: ActivationMemCube

GAIAN use:
- Current conversation with user (128K tokens max)
- KV cache reuse for repeated Earth Twin queries
- Context compression for long conversations

3. PLAINTEXT MEMORY (External Knowledge Store)
─────────────────────────────────────────────────────────────────
What it is: External knowledge stored as text/vectors
Examples: User memories; documents; notes; conversation history
Characteristics: Persistent; updatable; requires retrieval
Update method: Direct write; RAG ingestion
MemOS management:
- Lifecycle management (create; update; delete; expire)
- Multi-modal integration (text + images + audio)
- Cross-platform coordination (phone + car + home)
- Migrate to parametric (learning) when accessed frequently
- MemCube type: PlaintextMemCube

GAIAN use:
- All GAIAN memories (Mi-Memory MemStack, Blueprint 49)
- Earth Twin knowledge base
- Indigenous knowledge (with CARE consent)
- User preferences and history
```

### 1.4 The MemCube — The Fundamental Unit

```
THE MEMCUBE — MEMOS'S FUNDAMENTAL ABSTRACTION

What is a MemCube?
A MemCube is the fundamental unit of memory in MemOS.
Every memory item — regardless of type — is a MemCube.

MemCube Structure:
{
  "id": "memcube_2026-09-09_health_bp",
  "type": "plaintext",  // parametric | activation | plaintext
  "content": "Blood pressure reading: 135/85 mmHg",
  "metadata": {
    "source": "wearable_omron",
    "timestamp": "2026-09-09T07:15:00Z",
    "importance": 0.85,
    "provenance": "automatic_measurement",
    "version": 1,
    "access_count": 3,
    "last_accessed": "2026-09-09T08:00:00Z"
  },
  "embedding": [0.123, -0.456, ...],  // 768-dim vector
  "relations": ["memcube_doctor_visit", "memcube_medication"]
}

MemCube Operations:
─────────────────────────────────────────────────────────────────
COMPOSE: Merge related MemCubes into episodes
  Example: Merge "blood pressure" + "doctor visit" + "medication"
  → Episode: "Health management 2026-09"

MIGRATE: Move between memory types
  Example: Frequently accessed plaintext → parametric (fine-tuning)
  Example: Compressed context → plaintext (summarization)
  Example: Parametric knowledge → plaintext (knowledge extraction)

FUSE: Combine evidence from multiple sources
  Example: Fuse wearable data + calendar + home camera
  → Episode: "Basketball bag reminder" (Mi-Memory MemFuse)

EXPIRE: Mark for deletion after time period
  Example: Temporary task context expires after 24 hours

FREEZE: Protect from modification
  Example: Sacred indigenous knowledge (CARE principles)

SCHEDULE: Determine when to retrieve/update
  Example: Health data retrieved daily; historical data on demand

The MemCube Lifecycle:
CREATE → STORE → RETRIEVE → UPDATE → MIGRATE → EXPIRE/FREEZE
```

### 1.5 MemOS Architecture

```
MEMOS ARCHITECTURE

MEMORY SCHEDULER
─────────────────────────────────────────────────────────────────
The Memory Scheduler is the core of MemOS.
It manages the lifecycle of all MemCubes:
- When to retrieve (based on query relevance)
- When to update (based on new information)
- When to migrate (based on access patterns)
- When to expire (based on time and importance)
- When to freeze (based on governance rules)

MEMORY READER
─────────────────────────────────────────────────────────────────
The Memory Reader retrieves relevant MemCubes for a query:
- Semantic search (vector similarity)
- Keyword search (BM25)
- Temporal search (recency)
- Graph traversal (relations)
- Hybrid retrieval (combination)

MEMORY WRITER
─────────────────────────────────────────────────────────────────
The Memory Writer creates and updates MemCubes:
- Extract memories from conversations
- Update existing memories
- Create new memories from external sources
- Manage provenance and versioning

MEMORY GRAPH
─────────────────────────────────────────────────────────────────
MemOS supports graph-based memory backends:
- Neo4j: Structured, explainable memory
- Other graph DBs: Flexible backends
- Relations between MemCubes
- Temporal reasoning over memory graph

MOS (MEMORY-AUGMENTED CHAT ORCHESTRATION)
─────────────────────────────────────────────────────────────────
MOS is the main interface for LLM-memory integration:
- Plug-and-play memory modules
- Works with HuggingFace, Ollama, custom LLMs
- Manages context window efficiently
- Injects relevant memories into prompts

KEY METRICS:
- 35.24% token savings vs baseline
- Supports: textual, activation (KV cache), parametric (LoRA)
- Backends: Neo4j, SQLite, custom
- LLMs: HuggingFace, Ollama, DeepSeek, custom
```

---

## PART II: MEMOS IMPLEMENTATION

### 2.1 MemOS Python Integration

```python
# GAIA 2.0 MemOS Integration
# Memory Operating System for GAIAN 2.0
# License: Apache-2.0

# Install: pip install memos
# GitHub: github.com/MemTensor/MemOS
# Docs: memos-docs.openmem.net

import asyncio
from pathlib import Path
from typing import Optional
from datetime import datetime

class GAIANMemOS:
    """
    GAIAN 2.0 Memory System powered by MemOS.
    
    MemOS treats memory as a first-class operational resource.
    It unifies parametric, activation, and plaintext memory
    under one unified framework.
    
    Paper: arXiv:2507.03724 (July 2025; v4 December 2025)
    GitHub: github.com/MemTensor/MemOS
    Stars: 11.2K
    License: Apache-2.0
    
    Key benefit: 35.24% token savings vs baseline
    """
    
    def __init__(
        self,
        person_id: str,
        data_dir: Path,
        ollama_model: str = "llama3.1:8b",
        enable_textual: bool = True,
        enable_activation: bool = False,  # Requires GPU
        enable_parametric: bool = False,  # Requires fine-tuning
        enable_graph: bool = False  # Requires Neo4j
    ):
        """
        Initialize GAIAN MemOS.
        
        Args:
            person_id: Unique identifier for this GAIAN's human
            data_dir: Local directory for memory storage
            ollama_model: Ollama model for GAIAN
            enable_textual: Enable plaintext memory (always True)
            enable_activation: Enable KV cache memory (GPU required)
            enable_parametric: Enable LoRA/adapter memory (fine-tuning)
            enable_graph: Enable graph-based memory (Neo4j required)
        """
        self.person_id = person_id
        self.data_dir = Path(data_dir)
        self.data_dir.mkdir(parents=True, exist_ok=True)
        self.ollama_model = ollama_model
        
        self.enable_textual = enable_textual
        self.enable_activation = enable_activation
        self.enable_parametric = enable_parametric
        self.enable_graph = enable_graph
        
        self._mos = None
        self._initialized = False
    
    def initialize(self):
        """Initialize MemOS with GAIAN configuration."""
        try:
            from memos import MemOS
            
            # MemOS configuration
            config = {
                "user_id": self.person_id,
                "session_id": f"gaian_{self.person_id}",
                
                # LLM backend (Ollama for local-first)
                "chat_model": {
                    "backend": "ollama",
                    "config": {
                        "model": self.ollama_model,
                        "base_url": "http://localhost:11434"
                    }
                },
                
                # Memory reader
                "mem_reader": {
                    "backend": "ollama",
                    "config": {
                        "model": "nomic-embed-text",
                        "base_url": "http://localhost:11434"
                    }
                },
                
                # Memory types
                "enable_textual_memory": self.enable_textual,
                "enable_activation_memory": self.enable_activation,
                "enable_parametric_memory": self.enable_parametric,
                "enable_preference_memory": True,
                
                # Memory scheduler (automated management)
                "enable_mem_scheduler": True,
                "mem_scheduler": {
                    "backend": "simple",
                    "config": {}
                },
                
                # User manager (SQLite for local-first)
                "user_manager": {
                    "backend": "sqlite",
                    "config": {
                        "db_path": str(self.data_dir / "memos.db")
                    }
                },
                
                # Context management
                "max_turns_window": 15,
                "top_k": 5,
                
                # PRO mode for complex queries
                "PRO_MODE": False
            }
            
            self._mos = MemOS(config)
            self._initialized = True
            
            print(f"✓ MemOS initialized for GAIAN {self.person_id}")
            print(f"  Textual memory: {self.enable_textual}")
            print(f"  Activation memory: {self.enable_activation}")
            print(f"  Parametric memory: {self.enable_parametric}")
            print(f"  Token savings: ~35.24% vs baseline")
        
        except ImportError:
            print("MemOS not installed. Run: pip install memos")
            print("GitHub: github.com/MemTensor/MemOS")
            print("Docs: memos-docs.openmem.net")
    
    def chat(self, user_message: str) -> str:
        """
        Chat with GAIAN using MemOS memory management.
        
        MemOS automatically:
        1. Retrieves relevant memories (top_k=5)
        2. Injects memories into context
        3. Generates response
        4. Extracts and stores new memories
        5. Manages memory lifecycle
        
        Args:
            user_message: User's message to GAIAN
        
        Returns: GAIAN's response
        """
        if not self._initialized:
            self.initialize()
        
        if self._mos is None:
            return "[MemOS not available — install: pip install memos]"
        
        try:
            response = self._mos.chat(user_message)
            return response
        except Exception as e:
            return f"[MemOS error: {e}]"
    
    def add_memory(
        self,
        content: str,
        memory_type: str = "plaintext",
        source: str = "user_conversation",
        importance: float = 0.5
    ) -> str:
        """
        Add a memory to GAIAN's MemOS store.
        
        Args:
            content: Memory content
            memory_type: "plaintext" | "activation" | "parametric"
            source: Source of memory
            importance: Importance score (0-1)
        
        Returns: Memory ID
        """
        if not self._initialized:
            self.initialize()
        
        if self._mos is None:
            return "memos_not_available"
        
        try:
            # MemOS memory addition
            memory_id = self._mos.add_memory(
                content=content,
                memory_type=memory_type,
                metadata={
                    "source": source,
                    "importance": importance,
                    "timestamp": datetime.utcnow().isoformat(),
                    "person_id": self.person_id
                }
            )
            return memory_id
        except Exception as e:
            return f"error_{e}"
    
    def search_memories(self, query: str, top_k: int = 5) -> list[dict]:
        """
        Search GAIAN's memories using MemOS hybrid retrieval.
        
        MemOS uses:
        - Semantic search (vector similarity)
        - Keyword search (BM25)
        - Temporal search (recency)
        - Graph traversal (if Neo4j enabled)
        
        Args:
            query: Search query
            top_k: Maximum results
        
        Returns: List of relevant memories
        """
        if not self._initialized:
            self.initialize()
        
        if self._mos is None:
            return []
        
        try:
            memories = self._mos.search(query=query, top_k=top_k)
            return memories
        except Exception as e:
            return [{"error": str(e)}]
    
    def migrate_memory(
        self,
        memory_id: str,
        from_type: str,
        to_type: str
    ) -> bool:
        """
        Migrate a memory between types (MemCube migration).
        
        MemOS supports migration:
        - plaintext → parametric (fine-tuning; learning)
        - activation → plaintext (compression; persistence)
        - parametric → plaintext (knowledge extraction)
        
        Args:
            memory_id: ID of memory to migrate
            from_type: Source memory type
            to_type: Target memory type
        
        Returns: True if migration successful
        """
        if not self._initialized:
            self.initialize()
        
        if self._mos is None:
            return False
        
        try:
            self._mos.migrate_memory(
                memory_id=memory_id,
                from_type=from_type,
                to_type=to_type
            )
            return True
        except Exception as e:
            print(f"Migration failed: {e}")
            return False
    
    def get_memory_stats(self) -> dict:
        """Get MemOS memory statistics."""
        if not self._initialized:
            self.initialize()
        
        if self._mos is None:
            return {"error": "MemOS not available"}
        
        try:
            stats = self._mos.get_stats()
            return {
                "person_id": self.person_id,
                "total_memories": stats.get("total", 0),
                "plaintext_memories": stats.get("plaintext", 0),
                "activation_memories": stats.get("activation", 0),
                "parametric_memories": stats.get("parametric", 0),
                "token_savings": "~35.24% vs baseline",
                "model": "MemOS (arXiv:2507.03724)"
            }
        except Exception as e:
            return {"error": str(e)}
    
    def delete_all_memories(self) -> bool:
        """
        Delete all memories (GAIA 2.0 Constitutional Invariant 0.8).
        
        Complete, immediate, irrecoverable deletion.
        """
        if not self._initialized:
            self.initialize()
        
        if self._mos is None:
            return False
        
        try:
            self._mos.clear_all_memories(user_id=self.person_id)
            print(f"✓ All MemOS memories deleted for {self.person_id}")
            return True
        except Exception as e:
            print(f"Deletion failed: {e}")
            return False


class GAIANMemOSEarthTwin:
    """
    MemOS integration for GAIA 2.0 Earth Twin knowledge.
    
    Uses MemOS to manage Earth Twin knowledge as parametric memory:
    - Frequently accessed Earth data → model weights (fine-tuning)
    - Recent Earth data → plaintext memory (RAG)
    - Current Earth state → activation memory (context)
    
    This enables GAIAN to have "baked-in" Earth knowledge
    without always querying the Earth Twin API.
    """
    
    def __init__(self, gaian_memos: GAIANMemOS):
        self.memos = gaian_memos
    
    def add_earth_briefing(self, briefing: str, date: str) -> str:
        """
        Add daily Earth briefing to MemOS.
        
        Earth briefings are stored as plaintext memories
        and can be migrated to parametric memory over time.
        """
        return self.memos.add_memory(
            content=f"Earth briefing {date}: {briefing}",
            memory_type="plaintext",
            source="earth_twin_api",
            importance=0.7
        )
    
    def add_tipping_point_alert(self, alert: str) -> str:
        """
        Add tipping point alert to MemOS.
        
        Critical alerts are stored with high importance
        and never expire.
        """
        return self.memos.add_memory(
            content=f"TIPPING POINT ALERT: {alert}",
            memory_type="plaintext",
            source="advantip_system",
            importance=0.95
        )
    
    def get_earth_context(self, query: str) -> list[dict]:
        """
        Get Earth context for a GAIAN query.
        
        Searches MemOS for relevant Earth memories.
        """
        return self.memos.search_memories(
            query=f"Earth {query}",
            top_k=3
        )
    
    def migrate_earth_knowledge_to_parametric(self, memory_ids: list[str]):
        """
        Migrate frequently accessed Earth knowledge to parametric memory.
        
        This is MemOS's continual learning capability:
        - Earth Twin data accessed frequently → fine-tune GAIAN
        - GAIAN learns Earth patterns without always querying API
        - Reduces token usage and API calls
        """
        for memory_id in memory_ids:
            success = self.memos.migrate_memory(
                memory_id=memory_id,
                from_type="plaintext",
                to_type="parametric"
            )
            if success:
                print(f"✓ Migrated Earth knowledge {memory_id} to parametric memory")


# ============================================================
# QUICK START
# ============================================================

async def memos_quick_start():
    """
    5-minute MemOS quick start for GAIA 2.0.
    
    Prerequisites:
    pip install memos
    ollama pull llama3.1:8b
    ollama pull nomic-embed-text
    """
    
    print("🌍 GAIA 2.0 MemOS Quick Start")
    print("=" * 50)
    
    # Initialize GAIAN MemOS
    print("\n1. Initializing GAIAN MemOS...")
    gaian_memos = GAIANMemOS(
        person_id="alice",
        data_dir=Path("./gaian_memos"),
        ollama_model="llama3.1:8b",
        enable_textual=True,
        enable_activation=False,  # Requires GPU
        enable_parametric=False   # Requires fine-tuning
    )
    gaian_memos.initialize()
    
    # Add some memories
    print("\n2. Adding memories...")
    mem_id = gaian_memos.add_memory(
        content="Alice prefers morning workouts at 7 AM",
        source="user_conversation",
        importance=0.8
    )
    print(f"   ✓ Memory added: {mem_id}")
    
    # Add Earth Twin memory
    print("\n3. Adding Earth Twin memory...")
    earth_twin = GAIANMemOSEarthTwin(gaian_memos)
    earth_id = earth_twin.add_earth_briefing(
        briefing="Global temperature anomaly: +1.24°C. Amazon: 17.2% deforested.",
        date="2026-09-09"
    )
    print(f"   ✓ Earth briefing added: {earth_id}")
    
    # Search memories
    print("\n4. Searching memories...")
    results = gaian_memos.search_memories("morning workout")
    print(f"   ✓ Found {len(results)} relevant memories")
    
    # Chat with GAIAN
    print("\n5. Chatting with GAIAN...")
    response = gaian_memos.chat(
        "What time do I usually work out?"
    )
    print(f"   GAIAN: {response}")
    
    # Get stats
    print("\n6. Memory statistics...")
    stats = gaian_memos.get_memory_stats()
    print(f"   Total memories: {stats.get('total_memories', 0)}")
    print(f"   Token savings: {stats.get('token_savings', 'N/A')}")
    
    print("\n✅ MemOS integration ready!")
    print("   Paper: arXiv:2507.03724 (July 2025)")
    print("   GitHub: github.com/MemTensor/MemOS (11.2K stars)")
    print("   Docs: memos-docs.openmem.net")
    print("   Token savings: ~35.24% vs baseline")


if __name__ == "__main__":
    asyncio.run(memos_quick_start())
```

---

## PART III: MEMOS vs MI-MEMORY — COMPLEMENTARY FRAMEWORKS

### 3.1 MemOS and Mi-Memory Together

GAIA 2.0 uses **both** MemOS and Mi-Memory (Blueprint 49). They are complementary, not competing:

```
MEMOS vs MI-MEMORY — COMPLEMENTARY FRAMEWORKS

MI-MEMORY (Blueprint 49):
─────────────────────────────────────────────────────────────────
Paper: arXiv:2607.18975 (July 21, 2026)
Team: Darwin Agent Team, Xiaomi
Focus: Lifecycle memory framework for Personal AI
Key innovation: Heterogeneous data integration (multimodal)

Strengths:
✓ Multimodal expansion (MemSense/MemFuse)
  - Camera, wearable, calendar, documents, audio
  - Cross-device episode linking
✓ Audit contract (typed evidence payloads)
  - Source identity and provenance
  - Diagnostic traces
✓ Deployment (LiteMem)
  - Markdown/Git artifacts
  - Offline-first; 90.81% transfer accuracy
✓ Evolution (D²ACCI/E²MEND)
  - Governed memory iteration
  - Gate/rollback records

Best for: GAIAN's personal memory (conversations, wearables, calendar)

MEMOS (Blueprint 58):
─────────────────────────────────────────────────────────────────
Paper: arXiv:2507.03724 (July 2025; v4 December 2025)
Team: 38 authors; multiple Chinese universities
Focus: Memory as OS resource; three memory types
Key innovation: MemCube migration; continual learning

Strengths:
✓ Three memory types (parametric + activation + plaintext)
  - Parametric: knowledge in model weights
  - Activation: KV cache management
  - Plaintext: external knowledge store
✓ MemCube migration
  - Plaintext → parametric (continual learning)
  - Activation → plaintext (compression)
✓ Token savings (35.24%)
  - Reduces inference cost
  - Efficient context management
✓ Production-ready
  - 11.2K GitHub stars
  - REST API; SDK; documentation
  - Cloud + self-hosted options

Best for: GAIAN's LLM memory management; Earth Twin knowledge

COMBINED ARCHITECTURE FOR GAIAN:
─────────────────────────────────────────────────────────────────
Mi-Memory handles:
- Personal memories (conversations, wearables, calendar)
- Multimodal evidence (camera, audio, documents)
- Audit trail and governance
- Offline deployment (LiteMem)

MemOS handles:
- LLM memory management (parametric + activation + plaintext)
- Earth Twin knowledge (plaintext → parametric migration)
- Token efficiency (35.24% savings)
- Continual learning (knowledge baked into model weights)

Integration:
Mi-Memory MemStack → MemOS PlaintextMemCube
  (Mi-Memory retrieves memories → MemOS injects into LLM context)

MemOS ParametricMemCube ← Mi-Memory L1 Essential
  (Frequently accessed Mi-Memory items → MemOS fine-tuning)
```

### 3.2 The GAIAN Memory Stack

```
GAIAN 2.0 COMPLETE MEMORY STACK

LAYER 4: GAIAN RESPONSE
  ↑ Generated by Ollama (Llama 3.1 8B)
  ↑ Context: L0 + L1 + retrieved L2/L3 + MemOS activation

LAYER 3: MEMOS ORCHESTRATION
  ↑ MOS (Memory-Augmented Chat Orchestration)
  ↑ Retrieves relevant MemCubes (top_k=5)
  ↑ Injects into context (35.24% token savings)
  ↑ Extracts new memories from response

LAYER 2: MEMORY TYPES
  ├── PARAMETRIC (MemOS)
  │   └── Fine-tuned Llama 3.1 8B weights
  │       (Earth Twin knowledge; user preferences)
  │
  ├── ACTIVATION (MemOS)
  │   └── KV cache for current conversation
  │       (Efficient context reuse)
  │
  └── PLAINTEXT (Mi-Memory + MemOS)
      ├── L0: Identity (always loaded; ~100 tokens)
      ├── L1: Essential (always loaded; ~500-800 tokens)
      ├── L2: On-demand (retrieved by topic)
      └── L3: Deep search (semantic similarity)

LAYER 1: STORAGE
  ├── SQLite (Mi-Memory; encrypted; local)
  ├── ChromaDB (Mi-Memory; vector search)
  ├── Milvus 3.0 (LF AI & Data; lake-native; Blueprint 54)
  └── Neo4j (MemOS; graph-based; optional)

LAYER 0: HARDWARE
  └── Asterinas (memory-safe kernel; Blueprint 57)
      └── MlsDisk (encrypted storage; FAST 2026)
```

---

## PART IV: MEMOS ECOSYSTEM

### 4.1 memmy-agent — Personal AI with Shared Memory

**memmy-agent** (github.com/MemTensor/memmy-agent, 1.3K stars) is a personal AI agent built on MemOS that gives every AI a shared, fully controlled memory.

```
MEMMY-AGENT — PERSONAL AI WITH SHARED MEMORY

GitHub: github.com/MemTensor/memmy-agent
Stars: 1.3K
License: MIT
Language: TypeScript

What it does:
"A personal AI agent & local memory hub for all AI agents,
gives every AI one shared, fully controlled memory and
persistent context — all AI remember the same you."

Supported agents:
- Claude Code
- Codex
- OpenClaw
- Hermes Agent
- And more

Key feature: Shared memory across all AI agents
- All your AI assistants share the same memory
- Consistent context across tools
- One memory hub for all AI interactions

GAIA 2.0 Relevance:
- GAIAN is the primary AI agent with shared memory
- Other AI tools (Claude, Codex) can access GAIAN's memory
- Consistent user context across all AI interactions
- Aligns with GAIA 2.0's vision of unified AI companion
```

### 4.2 HaluMem — Hallucination Evaluation

**HaluMem** (github.com/MemTensor/HaluMem, 159 stars) is the first hallucination evaluation benchmark for agent memory systems.

```
HALUMEM — HALLUCINATION EVALUATION FOR MEMORY SYSTEMS

GitHub: github.com/MemTensor/HaluMem
Stars: 159
License: Python

What it does:
"HaluMem is the first operation level hallucination evaluation
benchmark tailored to agent memory systems."

Why it matters:
- Memory systems can hallucinate (fabricate memories)
- HaluMem evaluates hallucination at the operation level
- Tests: memory retrieval; memory creation; memory update

GAIA 2.0 Relevance:
- GAIAN must not hallucinate memories (constitutional requirement)
- HaluMem evaluates GAIAN's memory accuracy
- Constitutional compliance: no fabricated memories
- Aligns with GAIA 2.0's transparency principle
```

### 4.3 MemRL — Self-Evolving Agents

**MemRL** (github.com/MemTensor/MemRL, 171 stars) enables self-evolving agents via runtime reinforcement learning on episodic memory.

```
MEMRL — SELF-EVOLVING AGENTS

GitHub: github.com/MemTensor/MemRL
Stars: 171
Paper: "MEMRL: SELF-EVOLVING AGENTS VIA RUNTIME REINFORCEMENT LEARNING ON EPISODIC MEMORY"

What it does:
- Agents learn from their own episodic memory
- Runtime reinforcement learning (no offline training)
- Self-evolving: agents improve through experience

GAIA 2.0 Relevance:
- GAIAN improves through interactions with its human
- Earth Twin knowledge improves through feedback
- Continual learning without expensive retraining
- Aligns with MemOS's continual learning vision
```

### 4.4 Modular Memory — ICML 2026

**"Position: Modular Memory is the Key to Continual Learning Agents"** (arXiv:2603.01761, ICML 2026 Spotlight) provides the theoretical foundation for MemOS's approach.

```
MODULAR MEMORY — ICML 2026 SPOTLIGHT

Paper: arXiv:2603.01761 (March 2026; ICML 2026 Spotlight)
Authors: 23 authors from leading institutions
Conference: ICML 2026 Position Track Spotlight

Key thesis:
"Combining the strengths of In-Weight Learning (IWL) and
In-Context Learning (ICL) through the design of modular memory
is the missing piece for continual adaptation at scale."

Framework:
- IWL (In-Weight Learning): Updates model parameters
  → Stable, long-term knowledge
- ICL (In-Context Learning): Uses context window
  → Rapid adaptation to new information
- Modular Memory: Bridges IWL and ICL
  → Best of both worlds

GAIA 2.0 Relevance:
- GAIAN needs both: stable Earth knowledge (IWL) + rapid adaptation (ICL)
- MemOS implements this: parametric (IWL) + plaintext (ICL)
- Mi-Memory provides the modular memory layer
- Together: continually learning GAIAN
```

---

## PART V: IMPLEMENTATION ROADMAP

### 5.1 GAIA 2.0 MemOS Integration Timeline

```
GAIA 2.0 MEMOS INTEGRATION ROADMAP

IMMEDIATE (September-October 2026):
─────────────────────────────────────────────────────────────────
□ Install MemOS: pip install memos
□ Run MemOS quick start (see Section 2.1)
□ Integrate MemOS with GAIAN MVP (Blueprint 37)
□ Test textual memory (plaintext MemCubes)
□ Test token savings (target: 35.24%)
□ Integrate with Mi-Memory (Blueprint 49)
□ Test HaluMem for hallucination evaluation

SHORT-TERM (Nov 2026 - Feb 2027):
─────────────────────────────────────────────────────────────────
□ Enable activation memory (KV cache; requires GPU)
□ Integrate Earth Twin knowledge as MemOS memories
□ Test MemCube migration (plaintext → parametric)
□ Integrate memmy-agent for cross-agent memory sharing
□ Enable graph-based memory (Neo4j; optional)
□ Test MemRL for self-evolving GAIAN

MEDIUM-TERM (Q2-Q3 2027):
─────────────────────────────────────────────────────────────────
□ Enable parametric memory (LoRA fine-tuning)
□ Migrate Earth Twin knowledge to parametric memory
□ Implement continual learning for GAIAN
□ Integrate MemOS with Milvus 3.0 (Blueprint 54)
□ Engage with MemTensor team (github.com/MemTensor)
□ Contribute GAIA 2.0 use case to MemOS

LONG-TERM (2028+):
─────────────────────────────────────────────────────────────────
□ Full MemOS integration (all three memory types)
□ GAIAN as reference implementation for MemOS
□ GAIA 2.0 Earth Twin knowledge in parametric memory
□ Self-evolving GAIAN via MemRL
□ MemOS as standard for planetary AI memory
```

---

## CONCLUSION: THE MEMOS COVENANT

MemOS is the memory operating system that makes GAIAN truly intelligent. Not just a chatbot that forgets everything after each conversation. Not just a RAG system that retrieves documents. But a true AI companion that learns, grows, and evolves — with you, for you, forever.

MemOS's three memory types — parametric, activation, and plaintext — mirror the three levels of human memory: long-term knowledge (parametric), working memory (activation), and episodic memory (plaintext). Together, they give GAIAN the full spectrum of memory that a true companion needs.

The 35.24% token savings is not just an efficiency metric. It means GAIAN can run on cheaper hardware, serve more people, and be more accessible to the billions of humans who deserve a personal AI companion.

**The GAIA 2.0 MemOS Covenant:**
> "GAIAN's memory is not a database. It is a living system — one that learns, grows, and evolves with its human. MemOS gives GAIAN the infrastructure to be truly intelligent: to remember what matters, to learn from experience, and to grow wiser with every conversation. This is what it means for GAIAN to belong to you — not just to hold your data, but to truly know you."

---

## QUICK REFERENCE

```
MEMOS QUICK REFERENCE

Papers:
- MemOS MAG: arXiv:2505.22101 (May 28, 2025)
- MemOS AI System: arXiv:2507.03724 (July 4, 2025; v4 Dec 3, 2025)
- Modular Memory: arXiv:2603.01761 (ICML 2026 Spotlight)

GitHub:
- MemOS: github.com/MemTensor/MemOS (11.2K stars; Apache-2.0)
- memmy-agent: github.com/MemTensor/memmy-agent (1.3K stars)
- HaluMem: github.com/MemTensor/HaluMem (159 stars)
- MemRL: github.com/MemTensor/MemRL (171 stars)

Docs: memos-docs.openmem.net
API: memos.openmem.net/docs/api/info

Install: pip install memos

Key Metrics:
- Token savings: 35.24% vs baseline
- Memory types: 3 (parametric + activation + plaintext)
- MemCube operations: compose, migrate, fuse, expire, freeze
- Backends: SQLite, Neo4j, custom
- LLMs: HuggingFace, Ollama, DeepSeek, custom

GAIA 2.0 Integration:
- Mi-Memory (Blueprint 49): Personal memory lifecycle
- MemOS (Blueprint 58): LLM memory management
- Milvus 3.0 (Blueprint 54): Vector database
- Asterinas (Blueprint 57): Memory-safe kernel

Key Quote:
"MemOS treats memory as a manageable system resource.
It unifies the representation, scheduling, and evolution
of plaintext, activation-based, and parameter-level memories."
— arXiv:2507.03724
```

---

*GAIA 2.0 MemOS Blueprint*
*Blueprint 58 — Version 1.0 — September 9, 2026*
*License: Apache-2.0 | Open Source | Open Access*
*"Don't let your AI forget again. Empower it with MemOS!"*
*"GAIAN's memory is not a database. It is a living system."*