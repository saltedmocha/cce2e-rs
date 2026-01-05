use chrono::{DateTime, Local};
use uuid::Uuid;

pub struct User {
    user_id: Uuid,
    username: String,
    contacts: Vec<Contact>,
}

pub struct Contact {
    contact_id: Uuid,
    username: String,
    added_date: DateTime<Local>,
    // in case of where someone want to lock a specific contact chat
    // making it unaccessible without a code access
    locked: bool,
}

impl User {
    pub fn new(uid: Option<Uuid>, uname: Option<String>) -> Self {
        if uid.is_some() && uname.is_some() {
            let name: String = uname.unwrap_or_else(generate_username);
            let id: Uuid = uid.unwrap_or_else(Uuid::new_v4);

            return User {
                user_id: id,
                username: name,
                contacts: Vec::with_capacity(255),
            };
        }

        if let Some(id) = uid {
            return User {
                user_id: id,
                username: generate_username(),
                contacts: Vec::with_capacity(255),
            };
        }

        if let Some(name) = uname {
            return User {
                user_id: Uuid::new_v4(),
                username: name,
                contacts: Vec::with_capacity(255),
            };
        }

        User {
            user_id: Uuid::new_v4(),
            username: generate_username(),
            contacts: Vec::with_capacity(255),
        }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn get_contact(&self) -> &Vec<Contact> {
        &self.contacts
    }

    pub fn add_contact(&mut self, contact: Contact) -> &Self {
        self.contacts.push(contact);
        self
    }
}

impl Contact {
    pub fn get_id(&self) -> &Uuid {
        &self.contact_id
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_added_date(&self) -> &DateTime<Local> {
        &self.added_date
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn add_add_date(&mut self) -> &Self {
        self.added_date = Local::now();

        self
    }

    pub fn lock_contact(&mut self) -> &Self {
        if self.is_locked() {
            return self;
        }

        self.locked = true;
        self
    }

    pub fn new(uname: Option<String>) -> Self {
        if let Some(name) = uname {
            return Self {
                contact_id: Uuid::new_v4(),
                username: name,
                added_date: Local::now(),
                locked: false,
            };
        }

        Self {
            contact_id: Uuid::new_v4(),
            username: generate_username(),
            added_date: Local::now(),
            locked: false,
        }
    }
}

fn generate_username() -> String {
    let suffix: String = {
        use rand::prelude::*;

        let mut rng: ThreadRng = rand::rng();
        let rannum: u64 = ThreadRng::next_u64(&mut rng);

        rannum.to_string()
    };

    let username: String = format!("user-{}", suffix);

    username
}
