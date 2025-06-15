#!/usr/bin/env python3
import os
import json
import argparse
from pathlib import Path
import litellm
import warnings

warnings.filterwarnings("ignore", category=UserWarning, module="pydantic")

# --- CLI args ---
parser = argparse.ArgumentParser()
parser.add_argument("--key", required=True, help="API key for the LLM provider")
parser.add_argument("--provider", required=True, help="Provider name (openai, deepseek, together, custom)")
parser.add_argument("--model", required=True, help="Model name (e.g. gpt-4o, deepseek-chat, mistral)")
parser.add_argument("--base_url", type=str, help="Base URL for custom provider")
parser.add_argument("--n", type=int, default=1, help="Number of questions to generate")
parser.add_argument("--lang", choices=["en", "zh"], default="en", help="Language of questions")
args = parser.parse_args()

# --- Auth setup ---
ENV_MAP = {
    "openai": "OPENAI_API_KEY",
    "deepseek": "DEEPSEEK_API_KEY",
    "together": "TOGETHERAI_API_KEY"
}
if args.provider == "custom":
    if not args.base_url:
        raise ValueError("Need --base_url for provider=custom")
    litellm.api_key = args.key
    litellm.api_base = args.base_url
else:
    env_var = ENV_MAP.get(args.provider)
    if not env_var:
        raise ValueError(f"Unsupported provider: {args.provider}")
    os.environ[env_var] = args.key

MODEL = args.model if args.provider == "openai" else f"{args.provider}/{args.model}"

# --- Paths ---
BASE_DIR = Path(__file__).resolve().parent.parent
PROMPT_DIR = BASE_DIR / "prompt"
SYSTEM_PROMPT_FILE = PROMPT_DIR / ("user.txt" if args.lang == "en" else "user-zh.txt")
OUTPUT_FILE = BASE_DIR / "data" / f"question-{args.lang}.jsonl"

# --- Prompts ---
system_prompt = SYSTEM_PROMPT_FILE.read_text(encoding="utf-8")
user_prompts = {
    "en": lambda n: f"Generate {n} different, user questions. Output one per line. No numbering, no quotes.",
    "zh": lambda n: f"请生成 {n} 个不同的的用户问题，每行一个。不加序号、不加引号。"
}

# --- Generate N questions at once ---
def generate_batch(n: int):
    try:
        res = litellm.completion(
            model=MODEL,
            messages=[
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_prompts[args.lang](n)},
            ],
            temperature=1.5,
            top_p=1.0,
            presence_penalty=1.0
        )
        raw = res.choices[0].message["content"].strip()
        lines = [line.strip() for line in raw.splitlines() if line.strip()]
        return [{"instruction": q} for q in lines]
    except Exception as e:
        print("⚠️ Generation failed:", e)
        return []

# --- Write each question as a JSON line ---
with open(OUTPUT_FILE, "a", encoding="utf-8") as f:
    results = generate_batch(args.n)
    for q in results:
        f.write(json.dumps(q, ensure_ascii=False) + "\n")
        print("✅", q["instruction"])
