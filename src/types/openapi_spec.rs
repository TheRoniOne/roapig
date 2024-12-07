// https://spec.openapis.org/oas/v3.1.1.html#oas-version

use std::{any::Any, collections::HashMap};

pub struct Contact {
    name: String,
    url: String,
    email: String,
}

pub struct License {
    name: String,
    identifier: String,
    url: String,
}

pub struct Info {
    title: String,
    summary: Option<String>,
    description: Option<String>,
    terms_of_service: Option<String>,
    contact: Option<Contact>,
    license: Option<License>,
    version: String,
}

pub struct ServerVariable {
    enum_values: Vec<String>,
    default: String,
    description: Option<String>,
}

pub struct Server {
    url: String,
    description: Option<String>,
    variables: HashMap<String, ServerVariable>,
}

pub struct ExternalDocs {
    description: Option<String>,
    url: String,
}

pub enum InParameter {
    Query,
    Header,
    Path,
    Cookie,
}

pub struct Reference {
    ref_value: String,
    summary: Option<String>,
    description: Option<String>,
}

pub struct Response {
    ref_value: String,
    summary: Option<String>,
    description: Option<String>,
}

type Security = HashMap<String, Box<dyn Any>>;

pub struct Operation {
    tags: Vec<String>,
    summary: Option<String>,
    description: Option<String>,
    external_docs: Option<ExternalDocs>,
    operation_id: String,
    parameters: Vec<Reference>,
    request_body: Option<Reference>,
    responses: HashMap<String, Response>,
    callbacks: Option<HashMap<String, Reference>>,
    deprecated: bool,
    security: Option<Security>,
    servers: Option<Vec<Server>>,
}

pub struct PathItem {
    ref_value: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    get: Option<Operation>,
    put: Option<Operation>,
    post: Option<Operation>,
    delete: Option<Operation>,
    options: Option<Operation>,
    head: Option<Operation>,
    patch: Option<Operation>,
    trace: Option<Operation>,
    servers: Option<Vec<Server>>,
    parameters: Option<Vec<Reference>>,
}

pub struct Component {
    schemas: HashMap<String, Box<dyn Any>>,
    responses: HashMap<String, Response>,
    parameters: HashMap<String, Reference>,
    examples: HashMap<String, Reference>,
    request_bodies: HashMap<String, Reference>,
    headers: HashMap<String, Reference>,
    security_schemes: HashMap<String, Reference>,
    links: HashMap<String, Reference>,
    callbacks: HashMap<String, Reference>,
    path_items: HashMap<String, PathItem>,
}

pub struct Tag {
    name: String,
    description: Option<String>,
    external_docs: Option<ExternalDocs>,
}

pub struct OpenApiSpec3_1 {
    openapi: String,
    info: Info,
    json_schema_dialect: &'static str,
    paths: HashMap<String, PathItem>,
    webhooks: Option<HashMap<String, PathItem>>,
    components: Option<HashMap<String, Reference>>,
    security: Option<Security>,
    tags: Option<Vec<Tag>>,
    external_docs: Option<ExternalDocs>,
}
