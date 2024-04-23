pub struct Configuration;

impl Configuration
{
    pub fn get_ldap_server() -> &'static str 
    {
        return "LDAP://localhost:1800";
    }

    pub fn get_domain() -> &'static str 
    {
        return "domain";
    }

    pub fn get_password() -> &'static str  
    {
       return "password";
    }
}