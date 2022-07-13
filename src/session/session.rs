use rocket_session_store::{Session, SessionError}; 

#[derive(Default, Clone)]
pub struct LoginSession{
    login : bool, 
}

#[derive(Default, Clone)]
pub struct VerifySession{
    auth_code : u64, 
}

pub async fn get_verify_code(session : &Session<'_, VerifySession>) -> Option<u64> {
    match session.get().await {
        Ok(v_session) => {
            match v_session{
                Some(s) => {
                    println!("get code : {}", s.auth_code); 
                    Some(s.auth_code)
                } 
                None => {
                    println!("nothing registerd..."); 
                    None
                }
            }   
        }
        Err(e) => {
            println!("[session error] {}", e); 
            None
        } 
    }
}


pub async fn set_verify_code(session : &Session<'_, VerifySession>, auth_code : u64)-> Result<(), SessionError>{
    let session_result = session.set(VerifySession{auth_code : auth_code}).await; 
    println!("succesfully set code {}", auth_code); 
    match session_result {
        Ok(t) => {
            println!("successfully set!"); 
            Ok(())
        }, 
        Err(e) => {
            println!("[session error] {}", e); 
            panic!(); 
            Err(e)
        }
    }
}
