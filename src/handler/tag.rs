// tag msg handler

use db::dba::Dba;
use actix_web::{ actix::Handler, error, Error };
use diesel::{ self, QueryDsl, ExpressionMethods, RunQueryDsl };
use chrono::Utc;
use uuid;

use model::tag::{ 
    Tag, NewTag, CheckTag, UpdateTag, TagsPerID, TagRut, NewTagRut, RutTag,
    StarTag, StarOrTag, TagStar, StarTagStatus 
};
use model::msg::{ Msg, TagMsg, TagListMsg };

// handle msg from api::tag.new_tag and get_tag
impl Handler<CheckTag> for Dba {
    type Result = Result<TagMsg, Error>;

    fn handle(&mut self, tg: CheckTag, _: &mut Self::Context) -> Self::Result {
        use db::schema::tags::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let action = tg.action;
        if &action == "POST" {
            let newtag = NewTag {
                id: &tg.tname,
                tname: &tg.tname,
                intro: "",
                logo: "",
                pname: "",
                item_count: 0,
                rut_count: 0,
                etc_count: 0,
                star_count: 0,
            };
            let tag_new = diesel::insert_into(tags)
                .values(&newtag)
                .get_result::<Tag>(conn)
                .map_err(error::ErrorInternalServerError)?;

            Ok( TagMsg { 
                status: 200, 
                message: "Added".to_string(),
                tag: tag_new.clone(),
            })
        } else { // GET
            let tag_q = tags.filter(&tname.eq(&tg.tname))
                .get_result::<Tag>(conn)
                .map_err(error::ErrorInternalServerError)?;

            Ok( TagMsg { 
                status: 200, 
                message: "Get".to_string(),
                tag: tag_q.clone(),
            })
        }
    }
}

// handle msg from api::tag.get_tag_list
impl Handler<TagsPerID> for Dba {
    type Result = Result<TagListMsg, Error>;

    fn handle(&mut self, per: TagsPerID, _: &mut Self::Context) -> Self::Result {
        use db::schema::tags::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let mut tag_list: Vec<String> = Vec::new();
        
        match per {
            TagsPerID::RutID(r) => {
                use db::schema::tagruts::dsl::*;
                tag_list = tagruts.filter(&rut_id.eq(&r)).select(tname)
                    .order(count.desc()).limit(10) // order per count
                    .load::<String>(conn)           
                    .map_err(error::ErrorInternalServerError)?;
            },
            TagsPerID::ItemID(i) => {
                use db::schema::tagitems::dsl::*;
                tag_list = tagitems.filter(&item_id.eq(&i)).select(tname)
                    .order(count.desc()).limit(10) 
                    .load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            TagsPerID::TagID(t) => {
                tag_list = tags.filter(&pname.eq(&t)).select(tname)
                    .order(vote.desc()).limit(10) 
                    .load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            TagsPerID::UserID(u) => { 
                use db::schema::startags::dsl::*;
                tag_list = startags.filter(&uname.eq(&u)).select(tname)
                    .order(star_at.desc()).limit(42) 
                    .load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?;; 
            },
            TagsPerID::Index(_) => {
                tag_list = tags.select(tname).order(vote.desc()).limit(16)
                    .load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
        }

        Ok( TagListMsg { 
            status: 200, 
            message: "Success".to_string(),
            tags: tag_list.clone(),
            count: tag_list.len(),
        })
    }
}

// handle msg from api::tag.update_tag
impl Handler<UpdateTag> for Dba {
    type Result = Result<TagMsg, Error>;

    fn handle(&mut self, tg: UpdateTag, _: &mut Self::Context) -> Self::Result {
        use db::schema::tags::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let tag_update = diesel::update(tags.filter(&tname.eq(&tg.tname)))
            .set((
                intro.eq(tg.intro.clone()),
                logo.eq(tg.logo.clone()),
                pname.eq(tg.pname.clone()),  // to check if pname existing?
            ))
            .get_result::<Tag>(conn)
            .map_err(error::ErrorInternalServerError)?;

        Ok( TagMsg { 
            status: 200, 
            message: "Updated".to_string(),
            tag: tag_update.clone(),
        })
    }
}

// handle msg from api::tag.tag_rut :  to be optimized
impl Handler<RutTag> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, rutg: RutTag, _: &mut Self::Context) -> Self::Result {
        use db::schema::tagruts::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let action = rutg.action;
        let rutID = rutg.rut_id;

        if &action == "1" {  // tag
            for rtg in rutg.tnames {
                // to check if tagged with a same tag
                let tr = tagruts.filter(&tname.eq(&rtg)).filter(&rut_id.eq(&rutID))
                    .load::<TagRut>(conn)
                    .map_err(error::ErrorInternalServerError)?.pop();
                match tr {
                    // if tagged, update count + 1 in tagruts
                    Some(tgr) => {
                        diesel::update(&tgr)
                        .set(count.eq(count + 1))
                        .execute(conn).map_err(error::ErrorInternalServerError)?;
                    },
                    // else new tag-rut
                    None => {
                        let new_tag_rut = NewTagRut {
                            id: &(rtg.clone() + "-" + &rutID),
                            tname: &rtg,
                            rut_id: &rutID,
                            count: 1,
                        };
                        //  to check if tname in tags? otherwise, new_tag
                        use db::schema::tags::dsl::*;
                        let tag_check = tags.filter(&tname.eq(&rtg)).load::<Tag>(conn)
                            .map_err(error::ErrorInternalServerError)?.pop();
                        match tag_check {
                            // if existing, tag then rut_count + 1 in tags
                            Some(t) => {
                                diesel::insert_into(tagruts).values(&new_tag_rut)
                                    .execute(conn).map_err(error::ErrorInternalServerError)?;
                                // then update tags.rut_count
                                diesel::update(&t)
                                .set(rut_count.eq(rut_count + 1)).execute(conn)
                                .map_err(error::ErrorInternalServerError)?;
                            },
                            // if no existing tname, new_tag
                            None => {
                                let newtag = NewTag {
                                    id: &rtg,
                                    tname: &rtg,
                                    intro: "",
                                    logo: "",
                                    pname: "",
                                    item_count: 0,
                                    rut_count: 1,
                                    etc_count: 0,
                                    star_count: 0,
                                };
                                // new_tag 
                                diesel::insert_into(tags).values(&newtag).execute(conn)
                                    .map_err(error::ErrorInternalServerError)?;
                                // then tag_rut
                                diesel::insert_into(tagruts).values(&new_tag_rut)
                                    .execute(conn).map_err(error::ErrorInternalServerError)?;
                            },
                        }  
                    },
                }
            }
        } else { // untag
            for rtg in rutg.tnames {
                diesel::delete(tagruts.filter(&tname.eq(&rtg))) 
                .execute(conn).map_err(error::ErrorInternalServerError)?;
            }
        }

        Ok( Msg { 
            status: 200, 
            message: "Done".to_string(),
        })
    }
}

// handle msg from api::tag.star_unstar_tag
impl Handler<StarOrTag> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, act: StarOrTag, _: &mut Self::Context) -> Self::Result {
        use db::schema::startags::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        match act.action {
            1  => {  // star
                // limit star tag to 42
                let tag_star_num = startags.filter(&uname.eq(&act.uname)).count()
                    .execute(conn).map_err(error::ErrorInternalServerError)?;
                if tag_star_num > 42 {
                    return Ok( Msg { status: 418, message: "unstar".to_string(),})
                }
                
                let uid = format!("{}", uuid::Uuid::new_v4());
                let new_star = TagStar {
                    id: &uid,
                    uname: &act.uname,
                    tname: &act.tname,
                    star_at: Utc::now().naive_utc(),
                    note: &act.note,
                };
                diesel::insert_into(startags).values(&new_star)
                        .execute(conn).map_err(error::ErrorInternalServerError)?;
                // to update star_count + 1 in tag
                use db::schema::tags::dsl::{tags, tname, star_count, rut_count, vote};
                diesel::update(tags.filter(&tname.eq(&act.tname)))
                    .set((
                        star_count.eq(star_count + 1),
                        vote.eq(rut_count * 2 + star_count)  // cal vote, to be task
                    ))
                    .execute(conn)
                    .map_err(error::ErrorInternalServerError)?;

                Ok( Msg { status: 200, message: "star".to_string(),})
            },
            0 => { // unsatr
                diesel::delete(
                    startags.filter(&tname.eq(&act.tname))
                            .filter(&uname.eq(&act.uname))
                )
                .execute(conn).map_err(error::ErrorInternalServerError)?;

                // to update the star_count - 1 in tag
                use db::schema::tags::dsl::{tags, tname as t_name, star_count};
                diesel::update(tags.filter(&t_name.eq(&act.tname)))
                    .set(star_count.eq(star_count - 1)).execute(conn)
                    .map_err(error::ErrorInternalServerError)?;

                Ok( Msg { status: 200, message: "unstar".to_string(),})
            },
            _ =>  { Ok( Msg { status: 400, message: "unstar".to_string(),}) },
        }
    }
}

// handle msg from api::tag.star_tag_status
impl Handler<StarTagStatus> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, status: StarTagStatus, _: &mut Self::Context) -> Self::Result {
        use db::schema::startags::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let check_status = startags
            .filter(&uname.eq(&status.uname))
            .filter(&tname.eq(&status.tname))
            .load::<StarTag>(conn)
            .map_err(error::ErrorInternalServerError)?.pop();
        
        match check_status {
            Some(_) => { Ok( Msg {status: 200, message: "star".to_string() }) },
            None => { Ok( Msg { status: 200, message: "unstar".to_string() }) },
        }
    }
}
