use std::sync::RwLock;

use self::{inspector::Inspector, operator::Operator};

use super::{declaration::Declaration, misc::location::Location};
use crate::prelude::*;
use chrono::naive::NaiveTime;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;
mod inspector;
mod operator;

// #[derive(Clone, PartialEq, PartialOrd, Debug)]
#[derive(Default, Debug)]
pub(super) struct Customs<'a> {
    id: Uuid,
    work_hours: (NaiveTime, NaiveTime),
    name: String,
    location: Option<&'a Location>,
    competence: String,
    phone_number: String,
    email: String,
    declarations: Vec<Declaration>,
    processor_channel: Option<Receiver<Declaration>>,
    inspectors: Vec<Inspector>,
    operators: Vec<Operator>,
}

impl<'a> Customs<'a> {
    #![allow(clippy::unwrap_used)]
    pub async fn new(id: Uuid, name: &str, location: &'a Location) -> Customs<'a> {
        Self {
            id,
            work_hours: (
                NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
            ),
            name: name.to_string(),
            location: Some(location),
            competence: String::new(),
            phone_number: String::new(),
            email: String::new(),
            declarations: Vec::default(),
            processor_channel: None,
            inspectors: Vec::default(),
            operators: Vec::default(),
        }
    }
    // getter_mut!( { async } name: &mut String, { async } post: &mut String);
    // setter!( { async } name: &str, { async } post: &str);
}

pub trait Logic {
    async fn update_decl(&mut self, decl: Declaration) -> Result<(), Box<dyn Error>>;
    async fn get_declaration(&self, id: Uuid) -> Option<&Declaration>;
    async fn receive_docs(&mut self) -> Result<(), Box<dyn Error>>;
}

///
/// We Hide Business Logic behind separate module.
/// We do this in order to if we want to turn current Structs
/// into DTO Structs (or just strip it out of said logic).
///
/// Import Logic: use <path>::<struct>::logic::*;
///
mod logic {
    use super::Logic;
    use crate::errors::declaration::Err as DErr;
    use crate::{errors::channel::Err as ChErr, models::declaration::Declaration};
    use futures::stream;
    use futures::StreamExt;
    use std::error::Error;
    use uuid::Uuid;

    impl<'a> Logic for super::Customs<'a> {
        async fn receive_docs(&mut self) -> Result<(), Box<dyn Error>> {
            // We need to take option in order to make second mut borrow for *self
            let mut ch = self.processor_channel.take();
            let res = if let Some(ch) = &mut ch {
                Ok(self
                    .update_decl(ch.recv().await.ok_or(ChErr::ChannelWasClosed {
                        customs_id: self.id,
                    })?)
                    .await?)
            } else {
                Err(Box::new(ChErr::NoOpenedChannel {
                    customs_id: self.id,
                }))
            };

            // Return taken option
            self.processor_channel = ch;

            Ok(res?)
        }

        async fn get_declaration(&self, id: Uuid) -> Option<&Declaration> {
            stream::iter(&self.declarations)
                .filter_map(async move |decl| {
                    if decl.id().await == id {
                        Some(decl)
                    } else {
                        None
                    }
                })
                .collect::<Vec<&Declaration>>()
                .await
                .pop()
        }

        async fn update_decl(&mut self, decl: Declaration) -> Result<(), Box<dyn Error>> {
            let id = decl.id().await;
            for declar in &mut self.declarations {
                if declar.id().await == id {
                    *declar = decl;
                    return Ok(());
                }
            }
            self.declarations.push(decl);
            Ok(())
        }
    }
}

/// Boilerplate
impl<'a> Customs<'a> {
    getter_ref!(
        { async } id: &Uuid,
        { async } name: &str,
        { async } competence: &str,
        { async } phone_number: &str,
        { async } email: &str,
        { async } declarations: &[Declaration],
        { async } processor_channel: &Option<Receiver<Declaration>>,
        { async } inspectors: &[Inspector],
        { async } operators: &[Operator]
    );

    setter!(
        { async } id: Uuid,
        { async } name: &str,
        { async } competence: &str,
        { async } phone_number: &str,
        { async } email: &str,
        { async } declarations: Vec<Declaration>,
        { async } inspectors: Vec<Inspector>,
        { async } operators: Vec<Operator>
    );

    getter_mut!(
        { async } id: &mut Uuid,
        { async } name: &mut String,
        { async } competence: &mut String,
        { async } phone_number: &mut String,
        { async } email: &mut String,
        { async } declarations: &mut [Declaration],
        { async } processor_channel: &mut Option<Receiver<Declaration>>,
        { async } inspectors: &mut [Inspector],
        { async } operators: &mut [Operator]
    );

    pub async fn processor_channel(&self) -> &Option<Receiver<Declaration>> {
        &self.processor_channel
    }

    pub async fn set_processor_channel(&mut self, processor_channel: Receiver<Declaration>) {
        self.processor_channel = Some(processor_channel);
    }
}
