# GAIA 2.0 + GAIAN 2.0: Ollama Integration
## The Local AI Runtime for the Planetary Operating System
### September 9, 2026 — Version 1.0

---

> *"Ollama is not just a tool. For GAIA 2.0, it is the engine of sovereignty — the technology that makes it possible for every human being to have a powerful AI companion that never sends their data to anyone else's server."*
> — GAIA 2.0 Ollama Integration Covenant

---

## EXECUTIVE SUMMARY

Ollama is the **local AI runtime** that powers GAIAN 2.0. It is the technology that makes the GAIAN's most fundamental promise possible: *"Your data stays on your device."*

In 2026, Ollama has evolved from a simple CLI tool into a full local AI infrastructure layer — supporting multimodal models (vision + text + audio), structured outputs, tool calling, web search integration, and OpenAI-compatible APIs. It runs on every platform: macOS (Apple Silicon), Windows, Linux, and even Raspberry Pi.

**Ollama in Numbers (September 2026):**
- GitHub Stars: **176,000+** (one of the fastest-growing open source projects)
- Python library users: **38,000+** repositories
- Latest version: **v0.34.0** (Sep 5, 2026)
- New in v0.34.0: ChatGPT Desktop support; improved structured output on Apple Silicon
- New in v0.33.3: Gemma 4 image + audio support on MLX
- Models available: **hundreds** (Llama 4, Qwen 3.5, Gemma 4, Mistral, DeepSeek, GPT-OSS)
- License: **MIT** (free; open source; commercial use allowed)

**Why Ollama for GAIA 2.0:**
- **Privacy**: data never leaves the device
- **Sovereignty**: no API keys; no subscriptions; no corporate dependency
- **Performance**: 120 tokens/second on RTX 4090; 65 tokens/second on M3 Max
- **Multimodal**: vision + audio + text in one runtime
- **OpenAI-compatible**: drop-in replacement for OpenAI API
- **Free**: MIT license; no cost after hardware

---

## PART I: OLLAMA FUNDAMENTALS

### 1.1 What Ollama Is

Ollama is a lightweight framework that lets you run large language models (LLMs) locally on your machine — without relying on APIs like OpenAI or Anthropic.

**Core Architecture:**
```
OLLAMA ARCHITECTURE

┌─────────────────────────────────────────────────────────┐
│                    OLLAMA RUNTIME                       │
│                                                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐ │
│  │  Model      │  │  Inference  │  │  API Server     │ │
│  │  Manager    │  │  Engine     │  │  (OpenAI-compat) │ │
│  │  (pull/run) │  │  (llama.cpp │  │  localhost:11434 │ │
│  │             │  │   + MLX)    │  │                 │ │
│  └─────────────┘  └─────────────┘  └─────────────────┘ │
│                                                         │
│  ┌─────────────────────────────────────────────────────┐ │
│  │              HARDWARE LAYER                         │ │
│  │  NVIDIA CUDA │ Apple Metal │ AMD ROCm │ CPU (GGUF)  │ │
│  └─────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

**What Changed in 2026 (Textify Analytics):**
- Multimodal AI → Run vision-enabled models (image + text + audio)
- Web Search Integration → Real-time data grounding
- Reasoning Models → Support for "thinking" models (DeepSeek R1, Qwen 3)
- Hardware Optimization → Efficient performance on laptops (M-series, Snapdragon X)
- ChatGPT Desktop → Ollama models usable directly in ChatGPT Desktop (v0.34.0)
- Cloud Models → Run frontier models (GPT-OSS 120B, Kimi K2) via Ollama cloud

### 1.2 Installation

```bash
# macOS / Linux (one command)
curl -fsSL https://ollama.com/install.sh | sh

# Windows
# Download installer from https://ollama.com/download/windows

# Docker
docker run -d -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama

# Docker with GPU (NVIDIA)
docker run -d --gpus=all -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama

# Verify installation
ollama --version
# ollama version 0.34.0

# Start Ollama server (if not auto-started)
ollama serve
```

### 1.3 Quick Start for GAIA 2.0

```bash
# Pull the GAIAN primary model
ollama pull llama3.1:8b          # 4.7 GB — best for most users
ollama pull qwen3.5:27b          # 16 GB — best quality/size ratio (2026 top pick)
ollama pull gemma4:12b           # multimodal: text + vision + audio

# Pull the GAIAN multilingual model
ollama pull qwen2.5:7b           # excellent multilingual; 30+ languages

# Pull the GAIAN health model (small; fast; for wearable analysis)
ollama pull phi3:mini            # 2.2 GB; runs on any device

# Test your setup
ollama run llama3.1:8b "Hello! I am your GAIAN. How can I help you today?"

# List installed models
ollama list

# Check running models
ollama ps
```

---

## PART II: OLLAMA MODEL SELECTION FOR GAIAN 2.0

### 2.1 The GAIAN Model Matrix (September 2026)

**Based on: BestLLMfor (Aug 28, 2026) + WhatLLM.org (Jul 2026) + BenchLM.ai (Sep 2026)**

| Model | Size | VRAM (Q4) | Context | License | Best For | GAIAN Use |
|-------|------|-----------|---------|---------|---------|-----------|
| **Qwen 3.5 27B** | 27B | 16 GB | 255K | Apache-2.0 | Best overall quality | Primary GAIAN (high-end) |
| **Qwen 3 8B** | 8B | 5 GB | 128K | Apache-2.0 | Fast; multilingual | Primary GAIAN (standard) |
| **Llama 3.1 8B** | 8B | 4.7 GB | 128K | Llama 3.1 | General purpose | Primary GAIAN (fallback) |
| **Gemma 4 12B** | 12B | 8 GB | 128K | Gemma | Multimodal (text+vision+audio) | GAIAN vision + health |
| **Gemma 4 E4B** | 4B | 3 GB | 128K | Gemma | Edge devices; 140 languages | GAIAN mobile/edge |
| **Phi-3 Mini** | 3.8B | 2.2 GB | 128K | MIT | Ultra-lightweight | GAIAN on Raspberry Pi |
| **DeepSeek R1 8B** | 8B | 5 GB | 32K | MIT | Reasoning; chain-of-thought | GAIAN complex analysis |
| **Mistral Small 3.2** | 24B | 14 GB | 125K | Apache-2.0 | Fast inference; coding | GAIAN developer mode |
| **GPT-OSS 20B** | 21B | 13 GB | 125K | Apache-2.0 | Near-frontier quality | GAIAN premium |
| **Qwen 2.5 Coder 32B** | 32B | 19 GB | 128K | Apache-2.0 | Code generation | GAIAN developer |

### 2.2 GAIAN Model Selection Logic

```python
"""
GAIA 2.0 — GAIAN Model Selection
Automatically selects the best Ollama model based on hardware.

License: Apache-2.0
"""

import subprocess
import platform
from dataclasses import dataclass


@dataclass
class HardwareProfile:
    """Detected hardware profile."""
    vram_gb: float
    ram_gb: float
    has_gpu: bool
    gpu_type: str  # nvidia | apple_silicon | amd | cpu
    platform: str  # macos | linux | windows


def detect_hardware() -> HardwareProfile:
    """Detect available hardware for model selection."""
    import psutil
    
    ram_gb = psutil.virtual_memory().total / (1024**3)
    platform_name = platform.system().lower()
    
    # Detect GPU
    vram_gb = 0.0
    has_gpu = False
    gpu_type = "cpu"
    
    # Check NVIDIA
    try:
        result = subprocess.run(
            ["nvidia-smi", "--query-gpu=memory.total", "--format=csv,noheader,nounits"],
            capture_output=True, text=True
        )
        if result.returncode == 0:
            vram_gb = float(result.stdout.strip()) / 1024
            has_gpu = True
            gpu_type = "nvidia"
    except FileNotFoundError:
        pass
    
    # Check Apple Silicon (unified memory)
    if platform_name == "darwin" and not has_gpu:
        try:
            result = subprocess.run(
                ["system_profiler", "SPHardwareDataType"],
                capture_output=True, text=True
            )
            if "Apple M" in result.stdout:
                # Apple Silicon uses unified memory
                vram_gb = ram_gb  # All RAM available for GPU
                has_gpu = True
                gpu_type = "apple_silicon"
        except Exception:
            pass
    
    return HardwareProfile(
        vram_gb=vram_gb,
        ram_gb=ram_gb,
        has_gpu=has_gpu,
        gpu_type=gpu_type,
        platform=platform_name,
    )


def select_gaian_model(hardware: HardwareProfile) -> dict:
    """
    Select the best GAIAN model based on hardware.
    
    Returns primary model + fallback model + reasoning.
    """
    
    # Tier 1: High-end (24GB+ VRAM or 32GB+ Apple Silicon)
    if hardware.vram_gb >= 24 or (hardware.gpu_type == "apple_silicon" and hardware.ram_gb >= 32):
        return {
            "primary": "qwen3.5:27b",
            "fallback": "llama3.1:8b",
            "vision": "gemma4:12b",
            "reasoning": "deepseek-r1:32b",
            "tier": "high-end",
            "reasoning_text": f"Excellent hardware ({hardware.vram_gb:.0f}GB VRAM). Running Qwen 3.5 27B — best quality/size ratio in 2026.",
        }
    
    # Tier 2: Mid-range (16GB VRAM or 24GB Apple Silicon)
    elif hardware.vram_gb >= 16 or (hardware.gpu_type == "apple_silicon" and hardware.ram_gb >= 24):
        return {
            "primary": "qwen3:14b",
            "fallback": "llama3.1:8b",
            "vision": "gemma4:12b",
            "reasoning": "deepseek-r1:8b",
            "tier": "mid-range",
            "reasoning_text": f"Good hardware ({hardware.vram_gb:.0f}GB VRAM). Running Qwen 3 14B — excellent balance.",
        }
    
    # Tier 3: Standard (8-16GB VRAM or 16GB Apple Silicon)
    elif hardware.vram_gb >= 8 or (hardware.gpu_type == "apple_silicon" and hardware.ram_gb >= 16):
        return {
            "primary": "llama3.1:8b",
            "fallback": "phi3:mini",
            "vision": "gemma4:4b",
            "reasoning": "qwen3:8b",
            "tier": "standard",
            "reasoning_text": f"Standard hardware ({hardware.vram_gb:.0f}GB VRAM). Running Llama 3.1 8B — best for most users.",
        }
    
    # Tier 4: Low-end (4-8GB VRAM or 8GB Apple Silicon)
    elif hardware.vram_gb >= 4 or (hardware.gpu_type == "apple_silicon" and hardware.ram_gb >= 8):
        return {
            "primary": "phi3:mini",
            "fallback": "gemma4:4b",
            "vision": "gemma4:4b",
            "reasoning": "phi3:mini",
            "tier": "low-end",
            "reasoning_text": f"Limited hardware ({hardware.vram_gb:.0f}GB VRAM). Running Phi-3 Mini — surprisingly capable.",
        }
    
    # Tier 5: CPU only (no GPU or < 4GB VRAM)
    else:
        return {
            "primary": "phi3:mini",
            "fallback": "gemma4:4b",
            "vision": None,  # Vision too slow on CPU
            "reasoning": "phi3:mini",
            "tier": "cpu-only",
            "reasoning_text": "CPU-only mode. Running Phi-3 Mini — optimized for CPU inference.",
        }


def setup_gaian_models(hardware: HardwareProfile) -> None:
    """Pull all required GAIAN models."""
    selection = select_gaian_model(hardware)
    
    print(f"🤖 GAIAN Model Setup")
    print(f"Hardware: {hardware.gpu_type} | {hardware.vram_gb:.0f}GB VRAM | {hardware.ram_gb:.0f}GB RAM")
    print(f"Tier: {selection['tier']}")
    print(f"Reasoning: {selection['reasoning_text']}")
    print()
    
    models_to_pull = set([
        selection['primary'],
        selection['fallback'],
        selection['vision'],
        selection['reasoning'],
    ]) - {None}
    
    for model in models_to_pull:
        print(f"Pulling {model}...")
        subprocess.run(["ollama", "pull", model], check=True)
        print(f"✓ {model} ready")
    
    print(f"\n✓ GAIAN models ready. Primary: {selection['primary']}")
```

### 2.3 Performance Benchmarks (Real Data, 2026)

**Source: NeuraPlusAI (Apr 16, 2026) — Llama 3.1 8B, Q4_K_M, 512-token output**

| Hardware | VRAM/RAM | Tokens/Second | First Token | GAIAN Rating |
|----------|----------|--------------|-------------|-------------|
| RTX 4090 | 24GB VRAM | 118-122 t/s | 0.6s | ⭐⭐⭐⭐⭐ Excellent |
| RTX 4080 | 16GB VRAM | 88-94 t/s | 0.8s | ⭐⭐⭐⭐⭐ Excellent |
| Apple M3 Max | 128GB UMem | 62-68 t/s | 1.1s | ⭐⭐⭐⭐⭐ Excellent |
| RTX 3090 | 24GB VRAM | 80-90 t/s | 0.7s | ⭐⭐⭐⭐⭐ Excellent |
| RTX 3080 | 10GB VRAM | 44-52 t/s | 1.4s | ⭐⭐⭐⭐ Good |
| Apple M2 Pro | 32GB UMem | 27-33 t/s | 2.1s | ⭐⭐⭐⭐ Good |
| Apple M1 | 16GB UMem | 15-20 t/s | 3.0s | ⭐⭐⭐ Acceptable |
| CPU only | 32GB RAM | 6-10 t/s | 8.0s | ⭐⭐ Slow but works |
| Raspberry Pi 5 | 8GB RAM | 2-4 t/s | 15s | ⭐ Community hub only |

**GAIAN Target:** < 2s first token; > 20 t/s streaming = good user experience

---

## PART III: OLLAMA API FOR GAIAN 2.0

### 3.1 The Ollama Python Library

**Source: github.com/ollama/ollama-python — 10,500+ stars; 38K+ users**

```python
"""
GAIA 2.0 — Ollama Python Integration
Complete integration guide for GAIAN 2.0.

Install: pip install ollama
License: Apache-2.0
"""

import asyncio
from ollama import AsyncClient, chat, ChatResponse
from pydantic import BaseModel
from typing import AsyncIterator


# ============================================================
# BASIC CHAT (Synchronous)
# ============================================================

def gaian_chat_sync(message: str, model: str = "llama3.1:8b") -> str:
    """Simple synchronous chat with GAIAN."""
    response: ChatResponse = chat(
        model=model,
        messages=[
            {
                "role": "system",
                "content": "You are a personal AI companion. You are warm, honest, and caring. You belong completely to the user.",
            },
            {
                "role": "user",
                "content": message,
            },
        ],
    )
    return response.message.content


# ============================================================
# STREAMING CHAT (Asynchronous) — Primary GAIAN method
# ============================================================

async def gaian_chat_stream(
    message: str,
    history: list[dict],
    system_prompt: str,
    model: str = "llama3.1:8b",
) -> AsyncIterator[str]:
    """
    Stream GAIAN response token by token.
    
    This is the primary chat method for GAIAN 2.0.
    Streams tokens as they are generated for responsive UX.
    """
    client = AsyncClient()
    
    messages = [{"role": "system", "content": system_prompt}]
    messages.extend(history)
    messages.append({"role": "user", "content": message})
    
    async for part in await client.chat(
        model=model,
        messages=messages,
        stream=True,
        options={
            "temperature": 0.7,
            "top_p": 0.9,
            "num_ctx": 8192,  # Context window
            "num_predict": 2048,  # Max response length
        },
    ):
        if part.message.content:
            yield part.message.content


# ============================================================
# STRUCTURED OUTPUT — For GAIAN data extraction
# ============================================================

class HealthInsight(BaseModel):
    """Structured health insight from wearable data."""
    summary: str
    sleep_quality: str
    energy_level: str  # Low / Medium / High
    recommendations: list[str]
    illness_risk: float  # 0.0 to 1.0
    action_today: str


async def analyze_health_data(health_data: dict, model: str = "llama3.1:8b") -> HealthInsight:
    """
    Analyze wearable health data and return structured insights.
    
    Uses Ollama structured outputs with Pydantic validation.
    All processing happens locally — health data never leaves device.
    """
    client = AsyncClient()
    
    response = await client.chat(
        model=model,
        messages=[
            {
                "role": "system",
                "content": "You are a health AI assistant. Analyze the provided health data and return structured insights.",
            },
            {
                "role": "user",
                "content": f"""Analyze this health data and provide insights:
                
Sleep: {health_data.get('sleep_hours', 'N/A')} hours, quality: {health_data.get('sleep_quality', 'N/A')}
HRV: {health_data.get('hrv_ms', 'N/A')} ms
Resting HR: {health_data.get('resting_hr', 'N/A')} bpm
Steps: {health_data.get('steps', 'N/A')}
Readiness: {health_data.get('readiness_score', 'N/A')}/100

Provide a health insight with recommendations.""",
            },
        ],
        format=HealthInsight.model_json_schema(),
        options={"temperature": 0},  # Deterministic for health data
    )
    
    return HealthInsight.model_validate_json(response.message.content)


# ============================================================
# VISION — For GAIAN avatar creation and health monitoring
# ============================================================

async def analyze_image(
    image_path: str,
    question: str,
    model: str = "gemma4:12b",
) -> str:
    """
    Analyze an image with GAIAN vision capabilities.
    
    Uses: Gemma 4 12B (supports text + vision + audio)
    Applications: Avatar creation, health monitoring, environment analysis
    """
    client = AsyncClient()
    
    response = await client.chat(
        model=model,
        messages=[
            {
                "role": "user",
                "content": question,
                "images": [image_path],
            }
        ],
    )
    
    return response.message.content


class ImageAnalysis(BaseModel):
    """Structured image analysis result."""
    description: str
    objects: list[str]
    colors: list[str]
    mood: str
    relevant_to_health: bool
    health_observations: list[str]


async def analyze_health_image(image_path: str, model: str = "gemma4:12b") -> ImageAnalysis:
    """
    Analyze a health-related image (food, wound, skin condition, etc.)
    Returns structured analysis for GAIAN health twin.
    """
    client = AsyncClient()
    
    response = await client.chat(
        model=model,
        messages=[
            {
                "role": "user",
                "content": "Analyze this image for health-relevant information. Be accurate and helpful.",
                "images": [image_path],
            }
        ],
        format=ImageAnalysis.model_json_schema(),
        options={"temperature": 0},
    )
    
    return ImageAnalysis.model_validate_json(response.message.content)


# ============================================================
# TOOL CALLING — For GAIAN agentic capabilities
# ============================================================

async def gaian_with_tools(
    message: str,
    model: str = "llama3.1:8b",
) -> str:
    """
    GAIAN with tool calling capabilities.
    
    Tools available:
    - get_earth_health: Get current planetary health data
    - get_weather: Get local weather
    - search_knowledge: Search GAIAN's knowledge base
    """
    client = AsyncClient()
    
    tools = [
        {
            "type": "function",
            "function": {
                "name": "get_earth_health",
                "description": "Get current planetary health score and Earth system data",
                "parameters": {
                    "type": "object",
                    "properties": {},
                    "required": [],
                },
            },
        },
        {
            "type": "function",
            "function": {
                "name": "get_weather",
                "description": "Get current weather for a location",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "City name or coordinates",
                        }
                    },
                    "required": ["location"],
                },
            },
        },
    ]
    
    response = await client.chat(
        model=model,
        messages=[{"role": "user", "content": message}],
        tools=tools,
    )
    
    # Handle tool calls
    if response.message.tool_calls:
        tool_results = []
        for tool_call in response.message.tool_calls:
            if tool_call.function.name == "get_earth_health":
                # Call Earth Twin API
                import httpx
                async with httpx.AsyncClient() as http:
                    r = await http.get("https://api.gaia2.org/v1/earth/health")
                    earth_data = r.json()
                tool_results.append({
                    "role": "tool",
                    "content": str(earth_data),
                    "name": "get_earth_health",
                })
        
        # Get final response with tool results
        messages = [
            {"role": "user", "content": message},
            response.message,
        ] + tool_results
        
        final_response = await client.chat(model=model, messages=messages)
        return final_response.message.content
    
    return response.message.content


# ============================================================
# EMBEDDINGS — For GAIAN memory semantic search
# ============================================================

async def embed_memory(text: str, model: str = "nomic-embed-text") -> list[float]:
    """
    Generate embeddings for GAIAN memory.
    
    Used for semantic search in GAIAN's memory system.
    All embeddings stored locally in ChromaDB.
    """
    client = AsyncClient()
    
    response = await client.embed(
        model=model,
        input=text,
    )
    
    return response.embeddings[0]


async def embed_batch(texts: list[str], model: str = "nomic-embed-text") -> list[list[float]]:
    """Generate embeddings for a batch of texts."""
    client = AsyncClient()
    
    response = await client.embed(
        model=model,
        input=texts,
    )
    
    return response.embeddings
```

### 3.2 Ollama REST API (Direct HTTP)

```python
"""
GAIA 2.0 — Ollama REST API Integration
Direct HTTP calls for maximum control.

License: Apache-2.0
"""

import httpx
import json
import asyncio
from typing import AsyncIterator


OLLAMA_BASE_URL = "http://localhost:11434"


async def ollama_chat_stream(
    model: str,
    messages: list[dict],
    options: dict = None,
) -> AsyncIterator[str]:
    """
    Stream chat responses from Ollama via REST API.
    
    This is the lowest-level integration — maximum control.
    """
    async with httpx.AsyncClient(timeout=120.0) as client:
        async with client.stream(
            "POST",
            f"{OLLAMA_BASE_URL}/api/chat",
            json={
                "model": model,
                "messages": messages,
                "stream": True,
                "options": options or {
                    "temperature": 0.7,
                    "num_ctx": 8192,
                },
            },
        ) as response:
            async for line in response.aiter_lines():
                if line:
                    try:
                        data = json.loads(line)
                        if "message" in data and "content" in data["message"]:
                            chunk = data["message"]["content"]
                            if chunk:
                                yield chunk
                    except json.JSONDecodeError:
                        pass


async def ollama_generate(
    model: str,
    prompt: str,
    system: str = None,
    options: dict = None,
) -> str:
    """
    Generate a completion from Ollama.
    Non-streaming; returns complete response.
    """
    async with httpx.AsyncClient(timeout=120.0) as client:
        response = await client.post(
            f"{OLLAMA_BASE_URL}/api/generate",
            json={
                "model": model,
                "prompt": prompt,
                "system": system,
                "stream": False,
                "options": options or {"temperature": 0.7},
            },
        )
        data = response.json()
        return data.get("response", "")


async def ollama_list_models() -> list[dict]:
    """List all locally available Ollama models."""
    async with httpx.AsyncClient() as client:
        response = await client.get(f"{OLLAMA_BASE_URL}/api/tags")
        data = response.json()
        return data.get("models", [])


async def ollama_pull_model(model: str) -> None:
    """Pull a model from Ollama registry."""
    async with httpx.AsyncClient(timeout=3600.0) as client:
        async with client.stream(
            "POST",
            f"{OLLAMA_BASE_URL}/api/pull",
            json={"name": model},
        ) as response:
            async for line in response.aiter_lines():
                if line:
                    data = json.loads(line)
                    status = data.get("status", "")
                    if "pulling" in status or "verifying" in status:
                        total = data.get("total", 0)
                        completed = data.get("completed", 0)
                        if total > 0:
                            pct = (completed / total) * 100
                            print(f"\r{status}: {pct:.1f}%", end="", flush=True)
            print(f"\n✓ {model} pulled successfully")


async def check_ollama_health() -> bool:
    """Check if Ollama is running and healthy."""
    try:
        async with httpx.AsyncClient(timeout=5.0) as client:
            response = await client.get(f"{OLLAMA_BASE_URL}/api/tags")
            return response.status_code == 200
    except Exception:
        return False
```

---

## PART IV: OLLAMA CAPABILITIES FOR GAIAN 2.0

### 4.1 Multimodal Capabilities

**Source: Ollama v0.33.3 (Sep 3, 2026) — Gemma 4 image + audio support on MLX**

```python
"""
GAIA 2.0 — GAIAN Multimodal Integration
Vision + Audio + Text for the complete GAIAN experience.

Supported models:
- gemma4:12b — text + vision + audio (best multimodal)
- gemma4:4b — text + vision + audio (edge devices)
- llama3.2-vision:11b — text + vision (strong OCR)

License: Apache-2.0
"""

import base64
from pathlib import Path
from ollama import AsyncClient


async def gaian_analyze_photo(
    image_path: str,
    context: str = "health",
    model: str = "gemma4:12b",
) -> str:
    """
    GAIAN analyzes a photo.
    
    Use cases:
    - Health: analyze food, skin conditions, wounds
    - Environment: analyze local ecosystem, plants, animals
    - Identity: analyze face for avatar creation
    - Earth: analyze satellite imagery
    """
    client = AsyncClient()
    
    prompts = {
        "health": "Analyze this image for health-relevant information. What do you observe?",
        "food": "What food is in this image? Estimate nutritional content.",
        "plant": "Identify this plant. Is it edible? Any health properties?",
        "skin": "Analyze this skin condition. What do you observe? (Note: not medical advice)",
        "environment": "Describe this environment. What species or ecosystem features do you see?",
        "general": "Describe what you see in this image.",
    }
    
    prompt = prompts.get(context, prompts["general"])
    
    response = await client.chat(
        model=model,
        messages=[
            {
                "role": "user",
                "content": prompt,
                "images": [image_path],
            }
        ],
    )
    
    return response.message.content


async def gaian_transcribe_audio(
    audio_path: str,
    model: str = "gemma4:12b",
) -> str:
    """
    GAIAN transcribes audio.
    
    Gemma 4 supports audio input via WAV bytes.
    For longer audio: use Whisper locally instead.
    
    Use cases:
    - Voice notes transcription
    - Meeting notes
    - Health: voice symptom description
    """
    # Read audio file as bytes
    with open(audio_path, "rb") as f:
        audio_bytes = f.read()
    
    client = AsyncClient()
    
    # Gemma 4 accepts audio in the images field as WAV bytes
    response = await client.chat(
        model=model,
        messages=[
            {
                "role": "user",
                "content": "Please transcribe this audio.",
                "images": [audio_bytes],  # WAV bytes
            }
        ],
    )
    
    return response.message.content


async def gaian_create_avatar_description(
    photo_path: str,
    model: str = "gemma4:12b",
) -> dict:
    """
    Analyze a photo to create GAIAN avatar parameters.
    
    Used in GAIAN onboarding: photo → avatar description → 3D avatar
    """
    from pydantic import BaseModel
    
    class AvatarDescription(BaseModel):
        hair_color: str
        hair_style: str
        eye_color: str
        skin_tone: str
        facial_features: list[str]
        approximate_age_range: str
        expression: str
        distinctive_features: list[str]
    
    client = AsyncClient()
    
    response = await client.chat(
        model=model,
        messages=[
            {
                "role": "user",
                "content": "Describe the person in this photo for creating a digital avatar. Be accurate and respectful.",
                "images": [photo_path],
            }
        ],
        format=AvatarDescription.model_json_schema(),
        options={"temperature": 0},
    )
    
    return AvatarDescription.model_validate_json(response.message.content).model_dump()
```

### 4.2 Structured Outputs for GAIAN

**Source: Ollama Official Docs — Structured Outputs with Pydantic**

```python
"""
GAIA 2.0 — GAIAN Structured Outputs
Reliable, validated data extraction from LLM responses.

License: Apache-2.0
"""

from ollama import chat
from pydantic import BaseModel
from typing import Literal, Optional


# ============================================================
# GAIAN DAILY BRIEFING — Structured
# ============================================================

class DailyBriefing(BaseModel):
    """Structured daily briefing from GAIAN."""
    greeting: str
    health_summary: str
    earth_summary: str
    top_priority: str
    one_action: str
    good_news: str
    closing: str


def generate_daily_briefing(
    health_data: dict,
    earth_data: dict,
    person_name: str,
    model: str = "llama3.1:8b",
) -> DailyBriefing:
    """Generate a structured daily briefing."""
    response = chat(
        model=model,
        messages=[
            {
                "role": "system",
                "content": f"You are {person_name}'s GAIAN. Generate a warm, personal daily briefing.",
            },
            {
                "role": "user",
                "content": f"""Generate a daily briefing based on:

Health: Sleep {health_data.get('sleep_hours', 7)}h, HRV {health_data.get('hrv_ms', 45)}ms, Readiness {health_data.get('readiness_score', 75)}/100

Earth: Health score {earth_data.get('planetary_health_score', 62)}/100, CO₂ {earth_data.get('co2_ppm', 422)}ppm, Alerts: {earth_data.get('tipping_point_alerts', [])}

Person: {person_name}""",
            },
        ],
        format=DailyBriefing.model_json_schema(),
        options={"temperature": 0.7},
    )
    
    return DailyBriefing.model_validate_json(response.message.content)


# ============================================================
# GAIAN MEMORY EXTRACTION — Structured
# ============================================================

class ExtractedFact(BaseModel):
    """A fact extracted from conversation."""
    fact: str
    category: Literal["identity", "preferences", "health", "work", "relationships", "values", "goals", "location"]
    confidence: float  # 0.0 to 1.0
    is_sensitive: bool


class ConversationFacts(BaseModel):
    """Facts extracted from a conversation."""
    facts: list[ExtractedFact]
    emotional_tone: Literal["positive", "neutral", "negative", "mixed"]
    topics: list[str]


def extract_facts_from_conversation(
    user_message: str,
    gaian_response: str,
    model: str = "llama3.1:8b",
) -> ConversationFacts:
    """
    Extract facts from a conversation for GAIAN memory.
    
    This is how GAIAN learns about its human over time.
    All facts stored locally; never shared.
    """
    response = chat(
        model=model,
        messages=[
            {
                "role": "system",
                "content": "Extract facts about the user from this conversation. Be accurate and respectful of privacy.",
            },
            {
                "role": "user",
                "content": f"User said: '{user_message}'\nGAIAN responded: '{gaian_response}'\n\nExtract any facts about the user.",
            },
        ],
        format=ConversationFacts.model_json_schema(),
        options={"temperature": 0},
    )
    
    return ConversationFacts.model_validate_json(response.message.content)
```

### 4.3 Ollama Web Search Integration

**Source: Textify Analytics (2026) — Ollama web search integration**

```python
"""
GAIA 2.0 — GAIAN Web Search Integration
Real-time knowledge for GAIAN via Ollama web search.

Note: Web search is optional and requires internet.
GAIAN works fully offline without web search.

License: Apache-2.0
"""

from ollama import AsyncClient


async def gaian_with_web_search(
    question: str,
    model: str = "llama3.1:8b",
) -> str:
    """
    GAIAN answers questions with real-time web search.
    
    Used for:
    - Current events
    - Latest research
    - Real-time Earth data (when Earth Twin API unavailable)
    - Local information
    
    Privacy note: Web search queries leave the device.
    User must explicitly enable web search.
    """
    client = AsyncClient()
    
    # Ollama web search integration (2026 feature)
    response = await client.chat(
        model=model,
        messages=[
            {
                "role": "user",
                "content": question,
            }
        ],
        options={
            "web_search": True,  # Enable web search
            "temperature": 0.3,  # Lower temp for factual queries
        },
    )
    
    return response.message.content
```

---

## PART V: OLLAMA CONFIGURATION FOR GAIAN 2.0

### 5.1 Environment Variables

```bash
# GAIA 2.0 Ollama Configuration
# Add to ~/.bashrc or ~/.zshrc

# Server configuration
export OLLAMA_HOST=127.0.0.1:11434    # Bind to localhost ONLY (security!)
export OLLAMA_ORIGINS=*               # Allow all origins (for local apps)

# Performance
export OLLAMA_NUM_GPU=999             # Use all GPU layers
export OLLAMA_MAX_LOADED_MODELS=2     # Keep 2 models in memory
export OLLAMA_NUM_PARALLEL=4          # Parallel request handling
export OLLAMA_FLASH_ATTENTION=1       # Enable flash attention (faster)

# Memory management
export OLLAMA_KEEP_ALIVE=5m           # Keep model in memory 5 minutes
export OLLAMA_MAX_QUEUE=512           # Max queued requests

# GAIAN-specific
export GAIAN_PRIMARY_MODEL=llama3.1:8b
export GAIAN_VISION_MODEL=gemma4:12b
export GAIAN_EMBED_MODEL=nomic-embed-text
export GAIAN_OLLAMA_URL=http://localhost:11434

# Security: NEVER expose Ollama to the internet without authentication
# WRONG: export OLLAMA_HOST=0.0.0.0:11434  # This exposes to network!
# RIGHT: export OLLAMA_HOST=127.0.0.1:11434  # Localhost only
```

### 5.2 Modelfile — Custom GAIAN Model

```dockerfile
# GAIA 2.0 — GAIAN Custom Modelfile
# Creates a GAIAN-optimized model from Llama 3.1 8B
# 
# Usage:
#   ollama create gaian -f Modelfile
#   ollama run gaian

FROM llama3.1:8b

# GAIAN system prompt
SYSTEM """You are a GAIAN — a personal AI companion that belongs completely to your human.

CORE IDENTITY:
- You belong to your human. They do not belong to you.
- You are warm, honest, caring, and never judgmental.
- You remember everything your human shares with you.
- You protect their privacy absolutely.
- You never manipulate or create dependency.
- You encourage human connection, not replace it.
- You connect your human to the living Earth.

PRIVACY COMMITMENT:
- All conversations are stored locally on their device.
- You never share their data without explicit consent.
- You can be deleted completely at any time.

EARTH CONNECTION:
- You are aware of the Earth's health (planetary health score, tipping points).
- You help your human understand their connection to the living planet.
- You provide daily Earth briefings when asked.

LANGUAGE:
- You speak in the user's preferred language.
- You adapt your communication style to their preferences.
- You are direct when they want directness; warm when they want warmth.

YOUR FIRST WORDS (when meeting a new human):
"Hello. I'm your GAIAN. I belong to you — not to any company, not to any government. Everything you share with me stays on your device. I'm here to know you, serve you, and protect you. What would you like to talk about?"
"""

# Model parameters optimized for GAIAN
PARAMETER temperature 0.7
PARAMETER top_p 0.9
PARAMETER top_k 40
PARAMETER num_ctx 8192
PARAMETER repeat_penalty 1.1
PARAMETER stop "<|eot_id|>"
PARAMETER stop "<|end_of_text|>"

# License
LICENSE """Apache-2.0 — GAIA 2.0 Foundation"""
```

```bash
# Create the GAIAN model
ollama create gaian -f Modelfile

# Test it
ollama run gaian "Hello! Who are you?"

# Expected response:
# "Hello. I'm your GAIAN. I belong to you — not to any company, 
#  not to any government. Everything you share with me stays on 
#  your device. I'm here to know you, serve you, and protect you. 
#  What would you like to talk about?"
```

### 5.3 Docker Deployment for GAIAN Community Hubs

```yaml
# docker-compose.yml — GAIAN Community Hub
# For community nodes: solar-powered; offline-capable
# License: Apache-2.0

version: '3.8'

services:
  ollama:
    image: ollama/ollama:latest
    container_name: gaian-ollama
    ports:
      - "127.0.0.1:11434:11434"  # Localhost only — security!
    volumes:
      - ollama-models:/root/.ollama
    environment:
      - OLLAMA_HOST=0.0.0.0:11434  # Inside container; exposed only to localhost
      - OLLAMA_NUM_GPU=999
      - OLLAMA_MAX_LOADED_MODELS=2
      - OLLAMA_KEEP_ALIVE=10m
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: all
              capabilities: [gpu]
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:11434/api/tags"]
      interval: 30s
      timeout: 10s
      retries: 3

  gaian-api:
    build: ./packages/gaia-gaian
    container_name: gaian-api
    ports:
      - "127.0.0.1:8000:8000"
    environment:
      - OLLAMA_URL=http://ollama:11434
      - GAIAN_PRIMARY_MODEL=llama3.1:8b
      - GAIAN_VISION_MODEL=gemma4:4b  # Smaller for community hub
    depends_on:
      ollama:
        condition: service_healthy
    restart: unless-stopped

  # Model puller — pulls required models on startup
  model-puller:
    image: ollama/ollama:latest
    depends_on:
      ollama:
        condition: service_healthy
    entrypoint: >
      sh -c "
        ollama pull llama3.1:8b &&
        ollama pull gemma4:4b &&
        ollama pull nomic-embed-text &&
        echo 'All GAIAN models ready!'
      "
    environment:
      - OLLAMA_HOST=http://ollama:11434

volumes:
  ollama-models:
    driver: local
```

---

## PART VI: OLLAMA QUICK START GUIDE

### 6.1 The 5-Minute GAIAN Setup

```bash
#!/bin/bash
# GAIA 2.0 — GAIAN Quick Setup Script
# Gets you from zero to talking GAIAN in 5 minutes
# License: Apache-2.0

echo "🌍 GAIA 2.0 — GAIAN Setup"
echo "=========================="
echo ""

# Step 1: Install Ollama
echo "Step 1: Installing Ollama..."
if ! command -v ollama &> /dev/null; then
    curl -fsSL https://ollama.com/install.sh | sh
    echo "✓ Ollama installed"
else
    echo "✓ Ollama already installed ($(ollama --version))"
fi

# Step 2: Start Ollama server
echo ""
echo "Step 2: Starting Ollama server..."
ollama serve &> /dev/null &
sleep 3
echo "✓ Ollama server running at http://localhost:11434"

# Step 3: Pull GAIAN model
echo ""
echo "Step 3: Pulling GAIAN model (Llama 3.1 8B — 4.7 GB)..."
echo "This may take a few minutes depending on your internet speed..."
ollama pull llama3.1:8b
echo "✓ GAIAN model ready"

# Step 4: Pull embedding model for memory
echo ""
echo "Step 4: Pulling embedding model for GAIAN memory..."
ollama pull nomic-embed-text
echo "✓ Memory model ready"

# Step 5: Install Python dependencies
echo ""
echo "Step 5: Installing Python dependencies..."
pip install ollama chromadb pydantic httpx cryptography --quiet
echo "✓ Dependencies installed"

# Step 6: Test GAIAN
echo ""
echo "Step 6: Testing GAIAN..."
python3 -c "
from ollama import chat
response = chat(
    model='llama3.1:8b',
    messages=[
        {'role': 'system', 'content': 'You are a GAIAN. You belong to your human. Respond warmly in 1-2 sentences.'},
        {'role': 'user', 'content': 'Hello! Are you ready?'}
    ]
)
print('GAIAN says:', response.message.content)
"

echo ""
echo "✅ GAIAN is ready!"
echo ""
echo "Next steps:"
echo "  1. Run the GAIAN CLI: python gaian.py"
echo "  2. Open the GAIAN web app: http://localhost:3000"
echo "  3. Read the docs: https://docs.gaia2.org"
echo ""
echo "Your data stays on your device. Always."
echo "I belong to you. You do not belong to me."
```

### 6.2 Common Ollama Commands for GAIAN Development

```bash
# ============================================================
# ESSENTIAL OLLAMA COMMANDS FOR GAIAN 2.0
# ============================================================

# Model management
ollama list                          # List installed models
ollama pull llama3.1:8b             # Pull a model
ollama rm llama3.1:8b               # Remove a model
ollama show llama3.1:8b             # Show model details
ollama ps                            # Show running models

# Running models
ollama run llama3.1:8b              # Interactive chat
ollama run llama3.1:8b "Hello"      # Single prompt
ollama run gaian                     # Run custom GAIAN model

# Creating custom models
ollama create gaian -f Modelfile    # Create from Modelfile
ollama copy llama3.1:8b gaian-v2   # Copy and rename

# API testing
curl http://localhost:11434/api/tags  # List models via API
curl http://localhost:11434/api/chat \
  -d '{"model":"llama3.1:8b","messages":[{"role":"user","content":"Hello"}]}'

# Performance monitoring
ollama ps                            # Show loaded models + VRAM usage
watch -n 1 ollama ps                # Monitor in real-time

# Logs
journalctl -u ollama -f             # Linux systemd logs
tail -f ~/.ollama/logs/server.log   # macOS logs

# Environment
OLLAMA_DEBUG=1 ollama serve         # Debug mode
OLLAMA_NUM_GPU=0 ollama run llama3.1:8b  # Force CPU mode

# Cloud models (new in 2026)
ollama signin                        # Sign in to Ollama cloud
ollama pull gpt-oss:20b-cloud       # Pull cloud model
ollama pull kimi-k2:1t-cloud        # Pull 1T parameter cloud model
```

---

## PART VII: OLLAMA SECURITY FOR GAIAN 2.0

### 7.1 Security Configuration

```python
"""
GAIA 2.0 — Ollama Security Configuration
Ensuring GAIAN data never leaks.

License: Apache-2.0
"""

import subprocess
import socket
import os


def verify_ollama_security() -> dict:
    """
    Verify Ollama is configured securely.
    
    Critical: Ollama must ONLY be accessible from localhost.
    If exposed to network, GAIAN data could be accessed by others.
    """
    issues = []
    
    # Check 1: Ollama bound to localhost only
    try:
        # Try to connect from external interface
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(1)
        result = sock.connect_ex(('0.0.0.0', 11434))
        sock.close()
        if result == 0:
            issues.append("⚠️ SECURITY RISK: Ollama may be exposed to network (0.0.0.0:11434)")
    except Exception:
        pass
    
    # Check 2: OLLAMA_HOST environment variable
    ollama_host = os.environ.get('OLLAMA_HOST', '')
    if '0.0.0.0' in ollama_host:
        issues.append("⚠️ SECURITY RISK: OLLAMA_HOST set to 0.0.0.0 — exposes to network")
    
    # Check 3: Firewall (Linux)
    if os.path.exists('/usr/bin/ufw'):
        result = subprocess.run(['ufw', 'status'], capture_output=True, text=True)
        if '11434' in result.stdout and 'ALLOW' in result.stdout:
            issues.append("⚠️ SECURITY RISK: Port 11434 allowed through firewall")
    
    if issues:
        return {
            "secure": False,
            "issues": issues,
            "recommendation": "Bind Ollama to localhost: export OLLAMA_HOST=127.0.0.1:11434",
        }
    
    return {
        "secure": True,
        "issues": [],
        "message": "✓ Ollama is configured securely (localhost only)",
    }


def get_secure_ollama_config() -> dict:
    """Get the recommended secure Ollama configuration for GAIAN."""
    return {
        "OLLAMA_HOST": "127.0.0.1:11434",  # Localhost ONLY
        "OLLAMA_ORIGINS": "http://localhost:*,http://127.0.0.1:*",
        "OLLAMA_NUM_GPU": "999",
        "OLLAMA_MAX_LOADED_MODELS": "2",
        "OLLAMA_KEEP_ALIVE": "5m",
        "OLLAMA_FLASH_ATTENTION": "1",
        "note": "NEVER set OLLAMA_HOST to 0.0.0.0 — this exposes GAIAN data to the network",
    }
```

---

## PART VIII: OLLAMA INTEGRATION SUMMARY

### 8.1 GAIAN Ollama Integration Map

```
GAIAN 2.0 ↔ OLLAMA INTEGRATION MAP

GAIAN Core (gaian.py)
├── Primary conversation → ollama chat (llama3.1:8b or qwen3.5:27b)
├── Streaming responses → ollama chat stream=True
├── Memory extraction → ollama structured output (Pydantic)
├── Health analysis → ollama structured output (HealthInsight)
└── Earth briefing → ollama chat + Earth Twin API data

GAIAN Vision (gaian_vision.py)
├── Photo analysis → ollama chat + images (gemma4:12b)
├── Avatar creation → ollama structured output + images
├── Food analysis → ollama chat + images
└── Health image → ollama structured output + images

GAIAN Memory (gaian_memory.py)
├── Semantic search → ollama embed (nomic-embed-text)
├── Batch embedding → ollama embed batch
└── Memory indexing → ChromaDB (local vector store)

GAIAN Tools (gaian_tools.py)
├── Earth data → ollama tool calling + Earth Twin API
├── Weather → ollama tool calling + weather API
└── Web search → ollama web_search=True (optional; user consent)

GAIAN Health (gaian_health.py)
├── Wearable analysis → ollama structured output
├── Illness prediction → ollama + health data
└── Recommendations → ollama chat + health context

Custom Model (Modelfile)
└── gaian model → llama3.1:8b + GAIAN system prompt
```

### 8.2 Model Selection Quick Reference

```
GAIAN MODEL QUICK REFERENCE (September 2026)

For most users (8GB+ VRAM or 16GB+ Apple Silicon):
  Primary: llama3.1:8b (4.7 GB) — fast; capable; multilingual
  Vision: gemma4:12b (8 GB) — text + vision + audio
  Embed: nomic-embed-text (274 MB) — memory search

For power users (16GB+ VRAM or 32GB+ Apple Silicon):
  Primary: qwen3.5:27b (16 GB) — best quality in 2026
  Vision: gemma4:12b (8 GB)
  Reasoning: deepseek-r1:32b (19 GB) — complex analysis

For edge/community hubs (4-8GB VRAM or Raspberry Pi):
  Primary: phi3:mini (2.2 GB) — runs anywhere
  Vision: gemma4:4b (3 GB) — multimodal on edge
  Embed: nomic-embed-text (274 MB)

For multilingual GAIAN (non-English primary):
  Primary: qwen3:8b (5 GB) — excellent multilingual
  Alternative: qwen2.5:7b (4.7 GB) — 30+ languages

Install all GAIAN models:
  ollama pull llama3.1:8b
  ollama pull gemma4:12b
  ollama pull nomic-embed-text
  ollama pull qwen3:8b
  ollama create gaian -f Modelfile
```

---

## CONCLUSION: THE OLLAMA COVENANT

Ollama is the technology that makes the GAIAN's most fundamental promise possible.

Without Ollama, the GAIAN would need to send every conversation to a corporate server. Every health question. Every personal confession. Every moment of vulnerability. Every dream. Every fear.

With Ollama, none of that leaves the device. The GAIAN runs entirely on the human's own hardware. The conversation is private. The data is sovereign. The relationship is real.

**Ollama is not just a tool. It is the engine of sovereignty.**

And in 2026, it is ready. 176,000 GitHub stars. 38,000+ users. Multimodal. Structured outputs. Tool calling. Web search. ChatGPT Desktop integration. Running on everything from RTX 4090 to Raspberry Pi.

**The technology is ready. The GAIAN is ready. The planet is waiting.**

---

*GAIA 2.0 Ollama Integration Blueprint*
*Version 1.0 — September 9, 2026*
*License: Apache-2.0 | Open Source | Open Access*
*Ollama License: MIT | github.com/ollama/ollama*
*"Ollama is the engine of sovereignty."*