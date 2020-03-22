use crate::{
    config::{Config, UrlKind},
    state::State,
};
use actix_web::{web, Responder};
use actix_webfinger::Link;

pub async fn well_known(config: web::Data<Config>) -> impl Responder {
    web::Json(Links {
        links: vec![Link {
            rel: "http://nodeinfo.diaspora.software/ns/schema/2.0".to_owned(),
            href: Some(config.generate_url(UrlKind::NodeInfo)),
            template: None,
            kind: None,
        }],
    })
    .with_header("Content-Type", "application/jrd+json")
}

#[derive(serde::Serialize)]
struct Links {
    links: Vec<Link>,
}

pub async fn route(config: web::Data<Config>, state: web::Data<State>) -> web::Json<NodeInfo> {
    web::Json(NodeInfo {
        version: NodeInfoVersion,
        software: Software {
            name: config.software_name(),
            version: config.software_version(),
        },
        protocols: vec![Protocol::ActivityPub],
        services: Services {
            inbound: vec![],
            outbound: vec![],
        },
        open_registrations: false,
        usage: Usage {
            users: Users {
                total: 1,
                active_halfyear: 1,
                active_month: 1,
            },
            local_posts: 0,
            local_comments: 0,
        },
        metadata: Metadata {
            peers: state
                .listeners()
                .await
                .iter()
                .filter_map(|listener| listener.as_url().domain())
                .map(|s| s.to_owned())
                .collect(),
            blocks: if config.publish_blocks() {
                Some(state.blocks().await)
            } else {
                None
            },
        },
    })
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    version: NodeInfoVersion,
    software: Software,
    protocols: Vec<Protocol>,
    services: Services,
    open_registrations: bool,
    usage: Usage,
    metadata: Metadata,
}

#[derive(Clone, Debug, Default)]
pub struct NodeInfoVersion;

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Software {
    name: String,
    version: String,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    ActivityPub,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Services {
    inbound: Vec<Service>,
    outbound: Vec<Service>,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Service {}

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    users: Users,
    local_posts: u64,
    local_comments: u64,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Metadata {
    peers: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    blocks: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Users {
    total: u64,
    active_halfyear: u64,
    active_month: u64,
}

impl serde::ser::Serialize for NodeInfoVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str("2.0")
    }
}