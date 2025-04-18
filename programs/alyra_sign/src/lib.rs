use anchor_lang::prelude::*;

declare_id!("FuhRcrvsVAj6iiDvEy5cUuSmsnGKWxdHTbJSod7CU4CS");

#[program]
pub mod alyra_sign {
    use super::*;

    pub fn create_registry(ctx: Context<CreateRegistry>) -> Result<()> {
        msg!("Creating global registry from: {:?}", ctx.program_id);
        let registry = &mut ctx.accounts.registry;
    
        registry.authority = ctx.accounts.authority.key();
        registry.events_count = 0;
        Ok(())
    }

    // Event creation
    pub fn create_event(ctx: Context<CreateEvent>, event_id: u64, event_code: String, title: String) -> Result<()> {
        msg!("Creating an event...");
        let event = &mut ctx.accounts.event;
        event.event_id = event_id;
        event.title =  title;
        event.event_code = event_code;
        event.sessions_count = 0;
        event.attendees_count = 0;
        event.authority = ctx.accounts.authority.key();


        let registry = &mut ctx.accounts.registry;
        registry.events_count = registry.events_count.checked_add(1).ok_or(ProgramError::ArithmeticOverflow)?;

        require!(ctx.accounts.authority.key() == registry.authority, SecurityError::WrongAuthority);

        Ok(())
    }


    // Registrer attendee
    pub fn register_attendee(ctx: Context<RegisterAttendee>, first_name: String, last_name: String, email: String) -> Result<()> {
        msg!("Creating a registered attendee...");
        let registered_attendee = &mut ctx.accounts.registered_attendee;
        registered_attendee.first_name =  first_name;
        registered_attendee.last_name =  last_name;
        registered_attendee.email =  email;
        registered_attendee.event = ctx.accounts.event.key();
        registered_attendee.attendee = ctx.accounts.attendee.key();
        registered_attendee.attendee_id = ctx.accounts.event.attendees_count + 1;
        registered_attendee.clockin = Vec::new();

        let event = &mut ctx.accounts.event;
        event.attendees_count += 1;

        Ok(())
    }




    // Session creation
    pub fn create_session(ctx: Context<CreateSession>, session_id: String, title: String, start_at: i64, end_at: i64) -> Result<()> {
        msg!("Creating a session...");
        let session = &mut ctx.accounts.session;
        session.event = ctx.accounts.event.key();
        session.title = title;
        session.start_at = start_at;
        session.end_at = end_at;
        session.session_id = session_id;


        let event = &mut ctx.accounts.event;
        event.sessions_count += 1;

        Ok(())
    }


    // ClockIn creation
    pub fn create_clockin(ctx: Context<CreateClockin>) -> Result<()> {
        msg!("Creating a clock-in...");
        let registered_attendee = &mut ctx.accounts.registered_attendee;
        let current_session = &mut ctx.accounts.session;
        

        let clockin = Clockin {
            session: current_session.key(),
            is_present: true,
            sign_at: Clock::get()?.unix_timestamp,
        };
        
        registered_attendee.clockin.push(clockin);

        current_session.clockin_count += 1;


       // require!(
       //     (event.attendees.contains(&ctx.accounts.signer.key())),
       //     ClockinError::AttendeeNotRegistered
       // );   

        //require!(
        //    (event.key() = session.event),
        //    ClockinError::EventSessionMismatch
        //);      

        Ok(())
    }    

}

// derive(Accounts)

#[derive(Accounts)]
pub struct CreateRegistry<'info> {
    #[account(init, payer = authority, space = 8 + Registry::INIT_SPACE, seeds = [b"alyra_sign", authority.key().as_ref()], bump)] 
    pub registry: Account<'info, Registry>,
    #[account(mut)] // signer must be mutable
    pub authority: Signer<'info>, 
    pub system_program: Program<'info, System>,
}



#[derive(Accounts)]
#[instruction(event_id: u64, event_code: String, title: String)]  // paramètres en entrée
pub struct CreateEvent<'info> {
    #[account(init, payer = authority, space = 8 + Event::INIT_SPACE, seeds = [b"event", event_id.to_le_bytes().as_ref()], bump)]
    pub event: Account<'info, Event>,
    #[account(
        mut,
        has_one = authority,
        seeds = [b"alyra_sign", authority.key().as_ref()],
        bump
    )]
    pub registry: Account<'info, Registry>,      

    #[account(mut)] // signer must be mutable
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(first_name: String, last_name: String, email: String)]  // paramètres en entrée
pub struct RegisterAttendee<'info> {
    #[account(init, payer = signer, space = 8 + 4388, seeds = [b"registered_attendee", event.key().as_ref(), attendee.key().as_ref(), (event.attendees_count + 1).to_be_bytes().as_ref()], bump)]
    pub registered_attendee: Account<'info, RegisteredAttendee>,
    #[account(mut, seeds = [b"event", event.event_code.as_bytes()], bump)]
    pub event: Account<'info, Event>,
    /// CHECK: the existing account of the attendee
    pub attendee: AccountInfo<'info>,
    #[account(mut)] // signer must be mutable
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(session_id: String, title: String, start_at: i64, end_at: i64)]  // paramètres en entrée
pub struct CreateSession<'info> {
    #[account(init, payer = signer, space = 8 + Session::INIT_SPACE, seeds = [b"session", event.key().as_ref(), session_id.as_bytes()], bump)] 
    pub session: Account<'info, Session>,
    #[account(mut, seeds = [b"event", event.event_code.as_bytes()], bump)]
    pub event: Account<'info, Event>,
    #[account(mut)] // signer must be mutable
    pub signer: Signer<'info>, 
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct CreateClockin<'info> {
    //#[account(init, payer = attendee, space = 8 + Clockin::INIT_SPACE, seeds = [b"clockin", session.key().as_ref(), attendee.key().as_ref()], bump)] 
    //pub clockin: Account<'info, Clockin>,
    #[account(mut, seeds = [b"session", session.event.as_ref(), session.session_id.as_bytes()], bump)]
    pub session: Account<'info, Session>, // modification du nombre de présences
    #[account(mut, seeds = [b"registered_attendee", session.event.as_ref(), attendee.key().as_ref()], bump)]
    pub registered_attendee: Account<'info, RegisteredAttendee>, 
    #[account(mut)] // signer must be mutable
    pub attendee: Signer<'info>,
    pub system_program: Program<'info, System>,
}





// ajouter add_attendee / remove_attendee / update_session / update_event / clock_in

// accounts

#[account]
#[derive(InitSpace)]
pub struct Registry {
    authority: Pubkey,
    events_count: u64,
}


#[account]
#[derive(InitSpace)]
pub struct Event {
    event_id: u64,
    #[max_len(50)]
    title: String, 
    #[max_len(10)]
    event_code: String, 
    authority: Pubkey,
    sessions_count: u64,
    attendees_count: u64,
}


#[account]
//#[derive(InitSpace)]
pub struct RegisteredAttendee {
    attendee_id: u64,       // 8
    attendee: Pubkey,       // 32
    //#[max_len(50)]
    first_name: String,     // 4 + 50
    //#[max_len(50)]
    last_name: String,      // 4 + 50
    //#[max_len(100)]
    email: String,          // 4 + 100
    event: Pubkey,          // 32
    //#[max_len(100, 41)]
    clockin: Vec<Clockin>,  // 4 + (100 * 41) = 4104  --> total : 4388
}

#[account]
#[derive(InitSpace)]
pub struct Session {
    event: Pubkey,
    #[max_len(10)]
    session_id: String,        // id de la session
    #[max_len(50)]
    title: String,
    start_at: i64,  // timestamp in seconds
    end_at: i64,    // timestamp in seconds
    clockin_count: u64, //nb de participants 
}



//#[account]
//#[derive(InitSpace)]
//pub struct Clockin {
//    session: Pubkey,
//    attendee: Pubkey,
//    is_present: bool,
//    sign_at: i64,
//}


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Clockin {
    session: Pubkey,  // 32
    is_present: bool, // 1
    sign_at: i64, // 8
}

// errors

//#[error_code]
//pub enum ClockinError {
//    #[msg("You are not a registered attendee for this event.")]
//    AttendeeNotRegistered,
//    #[msg("Session's event mismatch.")]
//    EventSessionMismatch,
//}

#[error_code]
pub enum SecurityError {
   #[msg("You are not a registered attendee for this event.")]
   AttendeeNotRegistered,
   #[msg("You are not the expected authority.")]
   WrongAuthority,
}