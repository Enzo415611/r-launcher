use toasty::Db;

use crate::db::{dto::instances_dto::InstanceDto, model::instances_model::InstanceModel};

pub async fn save_instance(mut db: Db, new_instance: InstanceDto) -> anyhow::Result<InstanceDto> {
    match InstanceModel::create()
        .name(&new_instance.name)
        .loader(&new_instance.loader)
        .loader_version(new_instance.loader_version)
        .version(&new_instance.version)
        .exec(&mut db)
        .await
    {
        Ok(instance) => Ok(InstanceDto {
            id: instance.id,
            name: instance.name,
            loader_version: instance.loader_version,
            loader: instance.loader,
            version: instance.version,
        }),
        Err(err) => Err(err.into()),
    }
}

pub async fn get_instance(mut db: Db, id: u64) -> anyhow::Result<InstanceDto> {
    match InstanceModel::get_by_id(&mut db, id).await {
        Ok(i) => Ok(InstanceDto {
            id: i.id,
            name: i.name,
            loader_version: i.loader_version,
            loader: i.loader,
            version: i.version,
        }),
        Err(err) => {
            eprintln!("{}", err);
            Err(err.into())
        }
    }
}

pub async fn get_all(mut db: Db) -> anyhow::Result<Vec<InstanceDto>> {
    match InstanceModel::all().exec(&mut db).await {
        Ok(v) => Ok(v
            .into_iter()
            .map(|ins| InstanceDto {
                id: ins.id,
                name: ins.name,
                version: ins.version,
                loader: ins.loader,
                loader_version: ins.loader_version,
            })
            .collect()),
        Err(err) => Err(err.into()),
    }
}
pub async fn update_instance(
    mut db: Db,
    new_instance: InstanceDto,
    id: u64,
) -> anyhow::Result<InstanceDto> {
    let mut update = InstanceModel::update_by_id(id);
    update.set_name(&new_instance.name);
    update.set_loader(&new_instance.loader);
    update.set_version(&new_instance.version);
    update.set_loader_version(&new_instance.loader_version);

    match update.exec(&mut db).await {
        Ok(_) => Ok(new_instance),
        Err(err) => {
            eprintln!("{}", err);
            Err(err.into())
        }
    }
}
pub async fn remove_instance(mut db: Db, id: u64) -> anyhow::Result<()> {
    match InstanceModel::delete_by_id(&mut db, id).await {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("{}", err);
            Err(err.into())
        }
    }
}
