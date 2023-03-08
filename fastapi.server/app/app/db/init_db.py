from datetime import datetime, timedelta
from sqlmodel.ext.asyncio.session import AsyncSession
from app import crud
from app.schemas.user_schema import IUserCreate

async def init_db(db_session: AsyncSession) -> None:
    users: list[IUserCreate] = []
    for index in range(1, 1001):
        str_index = str(index)
        users.append(
            IUserCreate(
            first_name=f"FirstName {str_index}",
            last_name=f"LastName {str_index}",
            email=f"user{str_index}@test.com",
            birthdate=datetime.utcnow()-timedelta(days=365*10),
            framework="Python (FastAPI)"
            )
        )

    for user in users:
        current_user = await crud.user.get_by_email(
            email=user.email, db_session=db_session
        )
        if not current_user:
            await crud.user.create(obj_in=user, db_session=db_session)
