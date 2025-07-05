# python/llm_call.py
import sys
import openai
import os

openai.api_key = os.getenv("OPENAI_API_KEY")

prompt = sys.argv[1]
model = sys.argv[2] if len(sys.argv) > 2 else "gpt-4.0"

response = openai.ChatCompletion.create(
    model=model,
    messages=[{"role": "user", "content": prompt}]
)

print(response["choices"][0]["message"]["content"])
