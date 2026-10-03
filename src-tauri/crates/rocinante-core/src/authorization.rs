use crate::types::Principal;

pub fn principal_is_admin(principal: &Principal) -> bool {
    principal.roles.iter().any(|role| role == "admin")
}

#[derive(Debug, Clone, Copy)]
pub struct AdminPrincipal<'a>(&'a Principal);

impl<'a> AdminPrincipal<'a> {
    pub fn user(&self) -> &'a str {
        &self.0.user
    }
}

pub fn authorize_admin(principal: &Principal) -> Option<AdminPrincipal<'_>> {
    principal_is_admin(principal).then_some(AdminPrincipal(principal))
}
