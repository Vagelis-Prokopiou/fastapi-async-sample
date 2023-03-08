from pydantic import BaseModel
from enum import Enum


class IMetaGeneral(BaseModel):
    pass


class IOrderEnum(str, Enum):
    ascendent = "ascendent"
    descendent = "descendent"


class TokenType(str, Enum):
    ACCESS = "access_token"
    REFRESH = "refresh_token"
