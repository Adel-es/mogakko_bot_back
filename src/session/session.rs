use rocket_session_store::Session; 

#[derive(Default, Clone)]
pub struct SessionData{
    login : bool, 
    auth_code : u64, 
}

async fn get_session_data(session : &Session<'_, SessionData>) -> SessionData {
    let sess = session.get().await.unwrap_or(None); 
    match sess {
        None => {
            session.set(SessionData { login: false, auth_code: 0 }).await.unwrap(); 
            return SessionData {login:false, auth_code : 0}
        }
        Some(session_value) => session_value
    }    
}

pub async fn get_session_login(session: &Session<'_, SessionData>  ) -> bool{
    let session_value : SessionData = get_session_data(session).await; 
    session_value.login
}

pub async fn get_session_auth_code(session : &Session<'_, SessionData> ) -> u64{
    let session_value : SessionData = get_session_data(session).await; 
    session_value.auth_code
}

async fn set_session_data(session : &Session<'_, SessionData>, session_value : SessionData) -> Result<(), ()> {
    let session_result = session.set(session_value).await;
    match session_result {
        Ok(_) => Ok(()), 
        Err(e) => {
            println!("[session error] {}", e); 
            Err(())
        }
    } 
}

pub async fn set_session_login(session : &Session<'_, SessionData>, login_value : bool)-> Result<(), ()>{
    let mut session_value : SessionData = get_session_data(session).await; 
    session_value.login = login_value; 
    set_session_data(session, session_value).await
}

pub async fn set_session_auth_code(session : &Session<'_, SessionData>, auth_code : u64)-> Result<(), ()>{
    let mut session_value : SessionData = get_session_data(session).await; 
    session_value.auth_code = auth_code; 
    set_session_data(session, session_value).await 
}