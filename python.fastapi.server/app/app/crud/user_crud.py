from app.schemas.user_schema import IUserCreate, IUserUpdate
from app.models.user_model import User
from typing import Optional
from app.crud.base_crud import CRUDBase
from sqlmodel import select
from sqlmodel.ext.asyncio.session import AsyncSession


class CRUDUser(CRUDBase[User, IUserCreate, IUserUpdate]):
    async def get_by_email(
        self, *, email: str, db_session: Optional[AsyncSession] = None
    ) -> Optional[User]:
        db_session = db_session or super().get_db().session
        users = await db_session.execute(select(User).where(User.email == email))
        return users.scalar_one_or_none()


user = CRUDUser(User)
