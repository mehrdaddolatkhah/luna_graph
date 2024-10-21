use diesel::prelude::*;
use uuid::Uuid;

pub struct UserPostgresRepository {}

impl UserOutboundPort for UserPostgresRepository {}
