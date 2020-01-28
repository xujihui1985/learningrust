use bcrypt::BcryptError;
use sqlite::Error as SqlErr;

#[derive(Debug)]
pub struct User {
    uname: String,
    pass_hash: String,
}

#[derive(Debug)]
pub enum UBaseErr {
    DbErr(SqlErr),
    HashError(BcryptError),
}

impl From<SqlErr> for UBaseErr {
    fn from(e: SqlErr) -> Self {
        UBaseErr::DbErr(e)
    }
}

impl From<BcryptError> for UBaseErr {
    fn from(e: BcryptError) -> Self {
        UBaseErr::HashError(e)
    }
}

pub struct UserBase {
    fname: String,
}

impl UserBase {
    pub fn new(db_file: &str) -> Self {
        UserBase {
            fname: db_file.to_string(),
        }
    }
    pub fn add_user(&self, uname: &str, pwd: &str) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let hpass = bcrypt::hash(pwd, 6)?;
        let mut st = conn.prepare("insert into users(u_name,p_word) values (?,?)")?;
        st.bind(1, uname)?;
        st.bind(2, &hpass[..])?;
        st.next()?;
        Ok(())
    }

    pub fn validate_user(&self, u_name: &str, p_word: &str) -> Result<bool, UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare("select u_name, p_word from users where u_name = ?")?;
        st.bind(1, u_name)?;
        if let Ok(sqlite::State::Row) = st.next() {
            let phash = st.read::<String>(1)?;
            return Ok(bcrypt::verify(p_word, &phash)?);
        }
        Ok(false)
    }
}

impl User {
    pub fn new(uname: String, pwd: String) -> Result<User, BcryptError> {
        Ok(User {
            uname,
            pass_hash: bcrypt::hash(&pwd, 6)?,
        })
    }

    pub fn verify(&self, pwd: &str) -> bool {
        bcrypt::verify(pwd, &self.pass_hash).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_user_test() {
        let fname = "test_data/users.db";
        let ub = UserBase {
            fname: fname.to_string(),
        };
        ub.add_user("sean", "hello").unwrap();
        assert_eq!(ub.validate_user("sean", "hello").unwrap(), true);
    }
}