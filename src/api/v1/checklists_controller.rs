use crate::{
    api::{
        auth::user_guard::AuthenticatedUser,
        helpers::{parse_uuid_path_param, ApiResultFrom, ArgentApiResult, ArgentResult, NewData},
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
    error::ArgentError,
};
use rocket::{delete, get, post, routes, serde::json::Json, Route};
use uuid::Uuid;

#[get("/checklists")]
async fn get_checklists(
    mut checklists_store: ChecklistStore,
    user: AuthenticatedUser,
) -> ArgentApiResult<Vec<Checklist>> {
    let lists = checklists_store.get_checklists_for_user(user.get()).await;
    ArgentApiResult::new(lists)
}

#[post("/checklists", data = "<checklist_request>")]
async fn create_checklist(
    mut checklists_store: ChecklistStore,
    user: AuthenticatedUser,
    checklist_request: Json<ChecklistRequest>,
) -> ArgentApiResult<()> {
    let checklist = Checklist::from_request(checklist_request.0);
    checklists_store
        .create_checklist(checklist, user.get())
        .await;
    ArgentApiResult::new(())
}

#[get("/checklists/<id>/items")]
async fn get_checklist_items(
    mut checklists_store: ChecklistStore,
    id: String,
    user: AuthenticatedUser,
) -> ArgentApiResult<Vec<ChecklistItem>> {
    let id = parse_uuid_path_param(&id)?;
    check_access(&mut checklists_store, id, user.get()).await?;
    checklists_store.get_checklist_items(id).await.api()
}

#[post("/checklistitems", data = "<checklistitem_request>")]
async fn create_checklistitem(
    mut checklists_store: ChecklistStore,
    checklistitem_request: Json<ChecklistItemRequest>,
    user: AuthenticatedUser,
) -> ArgentApiResult<()> {
    let item = checklistitem_request.into_inner().get()?;
    check_access(&mut checklists_store, item.checklist, user.get()).await?;
    checklists_store.add_item(item).await;
    ArgentApiResult::new(())
}

#[post("/checklistitems/<id>/done")]
async fn set_item_done(
    mut checklists_store: ChecklistStore,
    id: String,
    _user: AuthenticatedUser,
) -> ArgentApiResult<()> {
    let id = parse_uuid_path_param(&id)?;
    checklists_store.set_item_done(id, true).await;
    ArgentApiResult::new(())
}

#[post("/checklistitems/<id>/not-done")]
async fn set_item_not_done(
    mut checklists_store: ChecklistStore,
    id: String,
    _user: AuthenticatedUser,
) -> ArgentApiResult<()> {
    let id = parse_uuid_path_param(&id)?;
    checklists_store.set_item_done(id, false).await;
    ArgentApiResult::new(())
}

#[delete("/checklists/<id>")]
async fn delete_checklist(
    mut checklists_store: ChecklistStore,
    id: String,
    user: AuthenticatedUser,
) -> ArgentApiResult<()> {
    let id = parse_uuid_path_param(&id)?;
    check_owner(&mut checklists_store, id, user.get()).await?;
    checklists_store.delete_checklist(id).await;
    ArgentApiResult::new(())
}

#[get("/checklists/<id>")]
async fn get_checklist(
    mut checklists_store: ChecklistStore,
    id: String,
    user: AuthenticatedUser,
) -> ArgentApiResult<Checklist> {
    let id = parse_uuid_path_param(&id)?;
    check_access(&mut checklists_store, id, user.get()).await?;
    checklists_store.get_checklist_by_id(id).await.api()
}

#[post("/checklists/<id>/clear-done")]
async fn clear_done(
    mut checklists_store: ChecklistStore,
    id: String,
    user: AuthenticatedUser,
) -> ArgentApiResult<()> {
    let id = parse_uuid_path_param(&id)?;
    check_access(&mut checklists_store, id, user.get()).await?;
    checklists_store.clear_done(id).await;
    ArgentApiResult::new(())
}

#[post("/checklists/<id>/share", data = "<share_req>")]
async fn share(
    mut checklists_store: ChecklistStore,
    id: String,
    user: AuthenticatedUser,
    share_req: Json<ShareRequest>,
) -> ArgentApiResult<()> {
    let checklist_id = parse_uuid_path_param(&id)?;
    let share_req = share_req.into_inner();
    let user_id = Uuid::parse_str(&share_req.user_id).map_err(|_| ArgentError::bad_request())?;
    check_owner(&mut checklists_store, checklist_id, user.get()).await?;
    checklists_store
        .add_user_access(checklist_id, user_id, share_req.access_type)
        .await;
    ArgentApiResult::new(())
}

#[post("/checklists/<id>/unshare/<user_id>")]
async fn un_share(
    mut checklists_store: ChecklistStore,
    id: String,
    user_id: String,
    user: AuthenticatedUser,
) -> ArgentApiResult<()> {
    let checklist_id = parse_uuid_path_param(&id)?;
    let user_id = parse_uuid_path_param(&user_id)?;
    check_owner(&mut checklists_store, checklist_id, user.get()).await?;
    let users = checklists_store
        .get_users_access_for_checklist(checklist_id)
        .await;
    let owners = users
        .iter()
        .filter(|user_access| user_access.access_type == AccessType::Owner)
        .collect::<Vec<_>>();
    if owners.len() == 1 {
        return Err(ArgentError::bad_request_msg(
            "cannot remove the last owner of a checklist",
        ));
    }
    //     val checklistOwners = userDataStore.getUsersForChecklist(checklistId)
    //     .filter { it.checklistAccessType == ChecklistAccessType.Owner }
    // if (checklistOwners.size == 1 && checklistOwners.first().id == userId) {
    //     throw BadRequestException("Cannot remove last owner of a list")
    // }
    checklists_store
        .remove_useraccess(checklist_id, user_id)
        .await;
    ArgentApiResult::new(())
}

#[get("/checklists/<id>/users")]
async fn get_users_for_checklist(
    mut checklists_store: ChecklistStore,
    id: String,
    user: AuthenticatedUser,
) -> ArgentApiResult<Vec<UserAccess>> {
    let checklist_id = parse_uuid_path_param(&id)?;
    check_access(&mut checklists_store, checklist_id, user.get()).await?;
    let user_accesses = checklists_store
        .get_users_access_for_checklist(checklist_id)
        .await;
    ArgentApiResult::new(user_accesses)
}

async fn check_access(
    checklists_store: &mut ChecklistStore,
    checklist_id: Uuid,
    user: User,
) -> ArgentResult<()> {
    match checklists_store.get_access_type(checklist_id, user).await {
        None => Err(ArgentError::forbidden()),
        Some(_) => Ok(()),
    }
}

async fn check_owner(
    checklist_store: &mut ChecklistStore,
    checklist_id: Uuid,
    user: User,
) -> ArgentResult<()> {
    match checklist_store.get_access_type(checklist_id, user).await {
        None | Some(AccessType::Editor) => Err(ArgentError::forbidden()),
        Some(AccessType::Owner) => Ok(()),
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
