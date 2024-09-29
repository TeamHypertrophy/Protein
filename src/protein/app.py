from fastapi import FastAPI

app = FastAPI()


@app.get("/")
async def root() -> None:
    return {"Hello": "World"}