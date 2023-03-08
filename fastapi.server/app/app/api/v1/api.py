from fastapi import APIRouter
from app.api.v1.endpoints import (
    user,
    weather,
)

api_router = APIRouter()
api_router.include_router(user.router, prefix="/user", tags=["user"])
api_router.include_router(weather.router, prefix="/weather", tags=["weather"])
