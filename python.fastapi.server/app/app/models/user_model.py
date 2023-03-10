from app.models.base_uuid_model import BaseUUIDModel
from datetime import datetime
from sqlmodel import Field, SQLModel, Column, DateTime
from typing import Optional
from pydantic import EmailStr


class UserBase(SQLModel):
    first_name: str
    last_name: str
    email: EmailStr = Field(
        nullable=True, index=True, sa_column_kwargs={"unique": True}
    )
    birthdate: Optional[datetime] = Field(
        sa_column=Column(DateTime(timezone=True), nullable=True)
    )  # birthday with timezone
    framework: Optional[str]


class User(BaseUUIDModel, UserBase, table=True):
    pass
