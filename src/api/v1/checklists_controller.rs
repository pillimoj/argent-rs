use crate::{
    api::{
        auth::user_guard::AuthenticatedUser,
        helpers::{
            convert_uuid, parse_uuid, ApiResultFrom, ArgentApiResult, ArgentResult, NewData, OkData,
        },
    },
    data::{
        checklists::{
            models::{
                AccessType, Checklist, ChecklistItem, ChecklistItemRequest, ChecklistRequest,
                ShareRequest, UserAccess,
            },
            store::ChecklistStore,
        },
        users::models::User,
    },
    error::{ArgentError, SimpleMessage},
};
use rocket::{delete, get, http::Status, post, routes, serde, serde::json::Json, Route};
use uuid::Uuid;

#[get("/checklists")]
async fn get_checklists(
    mut checklists_store: ChecklistStore,
    user: AuthenticatedUser,
) -> ArgentApiResult<Vec<Checklist>> {
    let lists = checklists_store.get_checklists_for_user(user.get()).await?;
    ArgentApiResult::new(lists)
}

#[post("/checklists", data = "<checklist_request>")]
async fn create_checklist(
    mut checklists_store: ChecklistStore,
    user: AuthenticatedUser,
    checklist_request: Json<ChecklistRequest>,
) -> ArgentApiResult<SimpleMessage> {
    let checklist = Checklist::from_request(checklist_request.0);
    checklists_store
        .create_checklist(checklist, user.get())
        .await?;
    ArgentApiResult::new_ok()
}

#[get("/checklists/<id>/items")]
async fn get_checklist_items(
    mut checklists_store: ChecklistStore,
    id: serde::uuid::Uuid,
    user: AuthenticatedUser,
) -> ArgentApiResult<Vec<ChecklistItem>> {
    let checklist_id = convert_uuid(&id);
    check_access(&mut checklists_store, checklist_id, user.get()).await?;
    checklists_store
        .get_checklist_items(checklist_id)
        .await
        .api()
}

#[post("/checklistitems", data = "<checklistitem_request>")]
async fn create_checklistitem(
    mut checklists_store: ChecklistStore,
    checklistitem_request: Json<ChecklistItemRequest>,
    user: AuthenticatedUser,
) -> ArgentApiResult<SimpleMessage> {
    let item = checklistitem_request.into_inner().get()?;
    check_access(&mut checklists_store, item.checklist, user.get()).await?;
    checklists_store.add_item(item).await?;
    ArgentApiResult::new_ok()
}

#[post("/checklistitems/<id>/done")]
async fn set_item_done(
    mut checklists_store: ChecklistStore,
    id: serde::uuid::Uuid,
    _user: AuthenticatedUser,
) -> ArgentApiResult<SimpleMessage> {
    let item_id = convert_uuid(&id);
    checklists_store.set_item_done(item_id, true).await?;
    ArgentApiResult::new_ok()
}

#[post("/checklistitems/<id>/not-done")]
async fn set_item_not_done(
    mut checklists_store: ChecklistStore,
    id: serde::uuid::Uuid,
    _user: AuthenticatedUser,
) -> ArgentApiResult<SimpleMessage> {
    let item_id = convert_uuid(&id);
    checklists_store.set_item_done(item_id, false).await?;
    ArgentApiResult::new_ok()
}

#[delete("/checklists/<id>")]
async fn delete_checklist(
    mut checklists_store: ChecklistStore,
    id: serde::uuid::Uuid,
    user: AuthenticatedUser,
) -> ArgentApiResult<SimpleMessage> {
    let checklist_id = convert_uuid(&id);
    check_owner(&mut checklists_store, checklist_id, user.get()).await?;
    checklists_store.delete_checklist(checklist_id).await?;
    ArgentApiResult::new_ok()
}

#[get("/checklists/<id>")]
async fn get_checklist(
    mut checklists_store: ChecklistStore,
    id: serde::uuid::Uuid,
    user: AuthenticatedUser,
) -> ArgentApiResult<Checklist> {
    let checklist_id = convert_uuid(&id);
    check_access(&mut checklists_store, checklist_id, user.get()).await?;
    checklists_store
        .get_checklist_by_id(checklist_id)
        .await
        .api()
}

#[post("/checklists/<id>/clear-done")]
async fn clear_done(
    mut checklists_store: ChecklistStore,
    id: serde::uuid::Uuid,
    user: AuthenticatedUser,
) -> ArgentApiResult<SimpleMessage> {
    let checklist_id = convert_uuid(&id);
    check_access(&mut checklists_store, checklist_id, user.get()).await?;
    checklists_store.clear_done(checklist_id).await?;
    ArgentApiResult::new_ok()
}

#[post("/checklists/<id>/share", data = "<share_req>")]
async fn share(
    mut checklists_store: ChecklistStore,
    id: serde::uuid::Uuid,
    user: AuthenticatedUser,
    share_req: Json<ShareRequest>,
) -> ArgentApiResult<SimpleMessage> {
    let checklist_id = convert_uuid(&id);
    let share_req = share_req.into_inner();
    let user_id = parse_uuid(&share_req.user_id, Status::BadRequest)?;
    check_owner(&mut checklists_store, checklist_id, user.get()).await?;
    checklists_store
        .add_user_access(checklist_id, user_id, share_req.access_type)
        .await?;
    ArgentApiResult::new_ok()
}

#[post("/checklists/<id>/unshare/<user_id>")]
async fn un_share(
    mut checklists_store: ChecklistStore,
    id: serde::uuid::Uuid,
    user_id: serde::uuid::Uuid,
    user: AuthenticatedUser,
) -> ArgentApiResult<SimpleMessage> {
    let checklist_id = convert_uuid(&id);
    let user_id = convert_uuid(&user_id);
    check_owner(&mut checklists_store, checklist_id, user.get()).await?;
    let users = checklists_store
        .get_users_access_for_checklist(checklist_id)
        .await?;
    let owners = users
        .iter()
        .filter(|user_access| user_access.access_type == AccessType::Owner)
        .collect::<Vec<_>>();
    if owners.len() == 1 {
        return Err(ArgentError::bad_request_msg(
            "cannot remove the last owner of a checklist",
        ));
    }
    checklists_store
        .remove_useraccess(checklist_id, user_id)
        .await?;
    ArgentApiResult::new_ok()
}

#[get("/checklists/<id>/users")]
async fn get_users_for_checklist(
    mut checklists_store: ChecklistStore,
    id: serde::uuid::Uuid,
    user: AuthenticatedUser,
) -> ArgentApiResult<Vec<UserAccess>> {
    let checklist_id = convert_uuid(&id);
    check_access(&mut checklists_store, checklist_id, user.get()).await?;
    let user_accesses = checklists_store
        .get_users_access_for_checklist(checklist_id)
        .await?;
    ArgentApiResult::new(user_accesses)
}

async fn check_access(
    checklists_store: &mut ChecklistStore,
    checklist_id: Uuid,
    user: User,
) -> ArgentResult<()> {
    match checklists_store.get_access_type(checklist_id, user).await? {
        AccessType::Owner | AccessType::Editor => Ok(()),
        AccessType::None => Err(ArgentError::forbidden()),
    }
}

async fn check_owner(
    checklist_store: &mut ChecklistStore,
    checklist_id: Uuid,
    user: User,
) -> ArgentResult<()> {
    match checklist_store.get_access_type(checklist_id, user).await? {
        AccessType::Owner => Ok(()),
        _ => Err(ArgentError::forbidden()),
    }
}

pub fn checklist_routes() -> Vec<Route> {
    routes![
        get_checklists,
        get_checklist_items,
        create_checklist,
        create_checklistitem,
        set_item_done,
        set_item_not_done,
        delete_checklist,
        get_checklist,
        clear_done,
        share,
        un_share,
        get_users_for_checklist
    ]
}
