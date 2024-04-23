use ldap3::{ LdapConn, ResultEntry, Scope, SearchEntry};
use ldap3::result::Result;
mod configuration;
use configuration::Configuration;

fn main () -> Result<()> 
{
    let mut ldap: LdapConn = connect()?;
    let search_result = get_search_result(&mut ldap)?;

    for entry in search_result 
    {
        println!("{:?}", SearchEntry::construct(entry));
    }

    Ok(ldap.unbind()?)
}

fn connect() -> Result<LdapConn>
{
        let mut ldap = LdapConn::new(Configuration::get_ldap_server())?;

        let _res = ldap
            .simple_bind(Configuration::get_domain(), Configuration::get_password())?
            .success()?;

        return Ok(ldap);
}

fn get_search_result(ldap : &mut LdapConn) -> Result<Vec<ResultEntry>>
{
    let (rs, _res) = ldap
            .search(
            "DC=d1,DC=net",
            Scope::Subtree,
             "(&(objectCategory=person)(objectClass=user))",
             vec!["cn", "uid", "mail"],
            )?.success()?;

            return Ok(rs);
}