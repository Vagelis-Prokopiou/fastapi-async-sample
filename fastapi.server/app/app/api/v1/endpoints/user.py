from app import crud
from fastapi import (
    APIRouter,
    Depends,
)
from app.schemas.response_schema import (
    IGetResponsePaginated,
    create_response,
)
from app.schemas.user_schema import (
    IUserRead,
)
from fastapi_pagination import Params

router = APIRouter()


@router.get("/list")
async def read_users_list(
    params: Params = Depends(),
) -> IGetResponsePaginated[IUserRead]:
    """
    Retrieve users. Requires admin or manager role
    """
    users = await crud.user.get_multi_paginated(params=params)
    return create_response(data=users)
