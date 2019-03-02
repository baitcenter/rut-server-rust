// item msg handler

use db::dba::Dba;
use actix_web::{ actix::Handler, error, Error };
use diesel::{ self, QueryDsl, ExpressionMethods, RunQueryDsl };
use chrono::Utc;
use uuid;

use model::item::{
    Item, NewItem, SubmitItem, UpdateItem, ItemID, ItemsPerID,
    Collect, NewCollect, CollectItem, CollectID 
};
use model::msg::{ Msg, ItemMsg, ItemListMsg, CollectMsg };
use model::rut::Rut;

// handle msg from api::item.submit_item
impl Handler<SubmitItem> for Dba {
    type Result = Result<ItemMsg, Error>;

    fn handle(&mut self, submit_item: SubmitItem, _: &mut Self::Context) -> Self::Result {
        use db::schema::items::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_item = NewItem {
            id: &uuid,
            title: &submit_item.title,
            uiid: &submit_item.uiid,
            authors: &submit_item.authors,  
            pub_at: &submit_item.pub_at,   
            publisher: &submit_item.publisher,
            category: &submit_item.category, 
            url: &submit_item.url,
            cover: &submit_item.cover,
            edition: &submit_item.edition,
            detail: &submit_item.detail,
            rut_count: 0,
            etc_count: 0, 
            done_count: 0, 
        };
        let item_new = diesel::insert_into(items)
            .values(&new_item)
            .get_result::<Item>(conn)
            .map_err(error::ErrorInternalServerError)?;

        Ok( ItemMsg { 
            status: 200, 
            message: "Submitted".to_string(),
            item: item_new.clone(),
        })
    }
}

// handle msg from api::item.get_item
impl Handler<ItemID> for Dba {
    type Result = Result<ItemMsg, Error>;

    fn handle(&mut self, itemid: ItemID, _: &mut Self::Context) -> Self::Result {
        use db::schema::items::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let item_query = items.filter(&id.eq(&itemid.item_id))
            .load::<Item>(conn).map_err(error::ErrorInternalServerError)?.pop();
        let mut item = Item::new(); 
        match item_query {
            Some(q) => {
                item = q.clone();
            },
            None => (),
        }
    
        Ok( ItemMsg { 
            status: 200, 
            message: "Success".to_string(),
            item: item,
        })
    }
}

// handle msg from api::item.get_item_list
impl Handler<ItemsPerID> for Dba {
    type Result = Result<ItemListMsg, Error>;

    fn handle(&mut self, perid: ItemsPerID, _: &mut Self::Context) -> Self::Result {
        use db::schema::items::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let mut item_id_vec: Vec<String> = Vec::new();
        let mut item_list: Vec<Item> = Vec::new();

        match perid {
            ItemsPerID::ItemID(i) => {
                item_list = items.filter(&id.eq(&i)).load::<Item>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            ItemsPerID::Title(t) => {
                item_list = items
                    .filter(&title.eq(&t)).load::<Item>(conn) //to do contains
                    .map_err(error::ErrorInternalServerError)?;
            },
            ItemsPerID::Uiid(d) => {
                item_list = items.filter(&uiid.eq(&d)).load::<Item>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            ItemsPerID::ItemUrl(u) => {
                item_list = items.filter(&url.eq(&u)).load::<Item>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            ItemsPerID::RutID(pid) => {
                use db::schema::collects::dsl::*;
                item_id_vec = collects
                    .filter(&rut_id.eq(&pid)).select(item_id).load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            ItemsPerID::TagID(pid) => {
                use db::schema::tagitems::dsl::*;
                item_id_vec = tagitems
                    .filter(&tname.eq(&pid)).select(item_id).load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            // ItemsPerID::UserID(pid, flag) => {},
        };

        // let item_id_vec = item_id_q.map_err(error::ErrorInternalServerError)?;
        
        for i in item_id_vec {
            let mut items_query = items
                .filter(&id.eq(&i)).load::<Item>(conn)
                .map_err(error::ErrorInternalServerError)?;
            item_list.append(&mut items_query);
        }
    
        Ok( ItemListMsg { 
            status: 200, 
            message: "Success".to_string(),
            items: item_list.clone(),
            count: item_list.clone().len(),
        })
    }
}

// handle msg from api::item.update_item
impl Handler<UpdateItem> for Dba {
    type Result = Result<ItemMsg, Error>;

    fn handle(&mut self, item: UpdateItem, _: &mut Self::Context) -> Self::Result {
        use db::schema::items::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let item_update = diesel::update(items)
            .filter(&id.eq(&item.id))
            .set( &UpdateItem{
                id: item.id.clone(),
                title: item.title.clone(),
                uiid: item.uiid.clone(),
                authors: item.authors.clone(),
                pub_at: item.pub_at.clone(),
                publisher: item.publisher.clone(),
                category: item.category.clone(),
                url: item.url.clone(),
                cover: item.cover.clone(),
                edition: item.edition.clone(), 
                detail: item.detail.clone(),
            })
            .get_result::<Item>(conn)
            .map_err(error::ErrorInternalServerError)?;

        Ok( ItemMsg { 
            status: 200, 
            message: "Updated".to_string(),
            item: item_update.clone(),
        })
    }
}

// handle msg from api::item.collect_item
impl Handler<CollectItem> for Dba {
    type Result = Result<CollectMsg, Error>;

    fn handle(&mut self, collect: CollectItem, _: &mut Self::Context) -> Self::Result {
        use db::schema::collects::dsl::*;
        use db::schema::ruts::dsl::{ruts, id as rid, item_count}; 
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        // to gen item order, curr_item_count + 1, or pass fron frontend
        let rutID = collect.rut_id;
        let item_num = ruts.filter(&rid.eq(&rutID)).select(item_count)
                        .first::<i32>(conn).map_err(error::ErrorInternalServerError)?;
        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_collect = NewCollect {
            id: &uuid,
            rut_id: &rutID,
            item_id: &collect.item_id, // need to check??
            item_order: item_num + 1,
            content: &collect.content,
            user_id: &collect.user_id,
            collect_at: Utc::now().naive_utc(),
        };
        let collect_new = diesel::insert_into(collects)
            .values(&new_collect)
            .get_result::<Collect>(conn)
            .map_err(error::ErrorInternalServerError)?;
    
        Ok( CollectMsg { 
            status: 200, 
            message: "Collected".to_string(),
            rut_id: rutID.clone(),
            collects: vec!(collect_new),
        })
    }
}

// handle msg from api::item.get_collect
impl Handler<CollectID> for Dba {
    type Result = Result<CollectMsg, Error>;

    fn handle(&mut self, cid: CollectID, _: &mut Self::Context) -> Self::Result {
        use db::schema::collects::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let c_query = collects
            .filter(&item_id.eq(&cid.item_id)).filter(&rut_id.eq(&cid.rut_id))
            .load::<Collect>(conn).map_err(error::ErrorInternalServerError)?.pop();
        
        if let Some(c) = c_query {
            Ok( CollectMsg { 
                status: 200, 
                message: "Success".to_string(),
                rut_id: cid.rut_id.clone(),
                collects: vec!(c),
            })
        } else {
            Ok( CollectMsg { 
                status: 400, 
                message: "Nothing".to_string(),
                rut_id: cid.rut_id.clone(),
                collects: Vec::new(),
            })
        }
    }
}
