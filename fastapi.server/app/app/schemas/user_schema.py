from app.utils.partial import optional
from app.models.user_model import UserBase
from uuid import UUID


class IUserCreate(UserBase):
    pass


# All these fields are optional
@optional
class IUserUpdate(UserBase):
    pass


class IUserRead(UserBase):
    id: UUID
