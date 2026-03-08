"""
Hexa-ISA Trace Dashboard (FastAPI)
TODO: M2 완료 후 구현
"""
# pip install fastapi uvicorn
# uvicorn app:app --reload

from fastapi import FastAPI

app = FastAPI(title="Hexa-ISA Trace Dashboard")

@app.get("/")
def root():
    return {"status": "Hexa-ISA Dashboard - TODO"}
