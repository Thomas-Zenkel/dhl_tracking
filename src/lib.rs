use std::error::Error;

static BASE_URL_PRODUCTION: &str =
    "https://cig.dhl.de/services/production/rest/sendungsverfolgung?xml=";
static BASE_URL_SANDBOX: &str = "https://cig.dhl.de/services/sandbox/rest/sendungsverfolgung?xml=";

static SANDBOX_XML_PARAM: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?> <data appname="zt12345" language-code="{language_code}" password="geheim" piece-code="00340434161094022115" request="d-get-piece-detail"/>"#;
static PRODUCTION_XML_PARAM: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?> <data appname="{zt_kennung}" language-code="{language_code}" password="{passwd_zt_kennung}" piece-code="{sendungsnummer}" request="d-get-piece-detail"/>"#;
/// Builds Sendungsverfolgung Struct for Sandbox
/// ```
/// use dhl_tracking::SendungsverfolgungBuilder;
///
/// fn main() {
///    let sv = SendungsverfolgungBuilder::new()
///        .sandbox(true)
///        .passwd_entwicklerportal("your login-password entwicklerportal".to_string())
///        .entwickler_id("EntwicklerID from Konto".to_owned())
///        .build()
///        .unwrap();
///    println!("{:?}", sv.get_piece_detail("00340434161094022115").unwrap());
/// }    
/// ```
///
/// Builds Sendungsverfolgung Struct for Production
/// ```
/// use dhl_tracking::SendungsverfolgungBuilder;
///
/// let sendungsverfolgung = SendungsverfolgungBuilder::new()
/// .zt_kennung("ztxxxxx".to_owned())
/// .passwd_zt_kennung("your password".to_owned())
/// .app_token("your token".to_owned())
/// .app_id("your app id".to_owned())
/// .sandbox(false)
/// .build()
/// .unwrap();
///
/// let delivery_data = sendungsverfolgung
/// .get_piece_detail("00300000000000000000")
/// .unwrap();
/// ```
pub struct SendungsverfolgungBuilder {
    zt_kennung: Option<String>,
    passwd_zt_kennung: Option<String>,
    entwickler_id: Option<String>,
    passwd_entwicklerportal: Option<String>,
    language_code: Option<String>,
    sandbox: Option<bool>,
    app_id: Option<String>,
    app_token: Option<String>,
}

impl Default for SendungsverfolgungBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl SendungsverfolgungBuilder {
    pub fn new() -> SendungsverfolgungBuilder {
        SendungsverfolgungBuilder {
            zt_kennung: None,
            passwd_zt_kennung: None,
            entwickler_id: None,
            passwd_entwicklerportal: None,
            language_code: Some("de".to_string()),
            sandbox: None,
            app_id: None,
            app_token: None,
        }
    }

    /// Zu finden im Entwicklerportal in den Kontoeinstellungen.
    pub fn entwickler_id(mut self, entwickler_id: String) -> SendungsverfolgungBuilder {
        self.entwickler_id = Some(entwickler_id);
        self
    }
    /// Passwort mit dem man sich im Entwicklerportal einloggt.
    pub fn passwd_entwicklerportal(
        mut self,
        passwd_entwicklerportal: String,
    ) -> SendungsverfolgungBuilder {
        self.passwd_entwicklerportal = Some(passwd_entwicklerportal);
        self
    }
    /// Erhält man von seinem Vertriebsmitarbeiter
    pub fn zt_kennung(mut self, zt_kennung: String) -> SendungsverfolgungBuilder {
        self.zt_kennung = Some(zt_kennung);
        self
    }
    /// Das Passwort das man zur ZT-Kennung erhalten hat.
    pub fn passwd_zt_kennung(mut self, passwd_zt_kennung: String) -> SendungsverfolgungBuilder {
        self.passwd_zt_kennung = Some(passwd_zt_kennung);
        self
    }
    /// Sandbox Modus aktivieren.
    pub fn sandbox(mut self, sandbox: bool) -> SendungsverfolgungBuilder {
        self.sandbox = Some(sandbox);
        self
    }
    /// Sprache in der die Antwort zurückgegeben werden soll.
    /// Mögliche Werte: de und en.
    pub fn language_code(mut self, language_code: String) -> SendungsverfolgungBuilder {
        self.language_code = Some(language_code);
        self
    }
    /// App Token zu App ID. Findet sich im Entwicklerportal unter Paket APIs -> Freigabe und Betrieb -> Details -> Token anzeigen.
    pub fn app_token(mut self, app_token: String) -> SendungsverfolgungBuilder {
        self.app_token = Some(app_token);
        self
    }
    /// App ID. Findet sich im Entwicklerportal unter Paket APIs -> Freigabe und Betrieb.
    pub fn app_id(mut self, app_id: String) -> SendungsverfolgungBuilder {
        self.app_id = Some(app_id);
        self
    }

    /// Ezeugt neue Instanz von Sendungsverfolgung oder gibt Fehler zurück.
    pub fn build(self) -> Result<Sendungsverfolgung, Box<dyn Error>> {
        let sandbox = self.sandbox.unwrap_or(true); //If not set, use sandbox
        let entwickler_id = match self.entwickler_id {
            Some(entwickler_id) => Some(entwickler_id),
            None => {
                if sandbox {
                    //Sandbox needs entwickler_id
                    return Err("Entwickler ID not set. Needet in sandbox mode.".into());
                } else {
                    None
                }
            }
        };
        let zt_kennung = match self.zt_kennung {
            Some(u) => Some(u),
            None => {
                if !sandbox {
                    //Production needs zt_kennung
                    Err("Can't build Sendungsverfolgung, zt_kennung not set. Needet in production mode.")?;
                }
                None
            }
        };
        let passwd_zt_kennung = match self.passwd_zt_kennung {
            Some(p) => Some(p),
            None => {
                if !sandbox {
                    return Err("Can't build Sendungsverfolgung, passwd_zt_kennung not set. Needet in production mode.".into());
                }
                None
            }
        };
        let passwd_entwicklerportal = match self.passwd_entwicklerportal {
            Some(p) => Some(p),
            None => {
                if sandbox {
                    return Err(
                        "Can't build Sendungsverfolgung, passwd_entwicklerportal not set. Needet in sandbox mode".into(),
                    );
                }
                None
            }
        };
        let app_token = match self.app_token {
            Some(l) => Some(l),
            None => {
                if !sandbox {
                    return Err("Can't build Sendungsverfolgung, app_token not set. Needet in production mode.".into());
                }
                None
            }
        };
        let app_id = match self.app_id {
            Some(l) => Some(l),
            None => {
                if !sandbox {
                    return Err("Can't build Sendungsverfolgung, app_id not set. Needet in production mode.".into());
                }
                None
            }
        };
        let language_code = match self.language_code {
            Some(l) => l,
            None => "de".to_string(),
        };
        Ok(Sendungsverfolgung {
            zt_kennung,
            passwd_zt_kennung,
            entwickler_id,
            passwd_entwicklerportal,
            language_code,
            sandbox,
            app_id,
            app_token,
        })
    }
}

/// Builds Sendungsverfolgung Struct for Sandbox
/// ```
/// use dhl_tracking::SendungsverfolgungBuilder;
///
/// fn main() {
///    let sv = SendungsverfolgungBuilder::new()
///        .sandbox(true)
///        .passwd_entwicklerportal("your login-password entwicklerportal".to_string())
///        .entwickler_id("EntwicklerID from Konto".to_owned())
///        .build()
///        .unwrap();
///    println!("{:?}", sv.get_piece_detail("00340434161094022115").unwrap());
/// }    
/// ```
///
/// Builds Sendungsverfolgung Struct for Production
/// ```
/// use dhl_tracking::SendungsverfolgungBuilder;
///
/// let sendungsverfolgung = SendungsverfolgungBuilder::new()
/// .zt_kennung("ztxxxxx".to_owned())
/// .passwd_zt_kennung("your password".to_owned())
/// .app_token("your token".to_owned())
/// .app_id("your app id".to_owned())
/// .sandbox(false)
/// .build()
/// .unwrap();
///
/// let delivery_data = sendungsverfolgung
/// .get_piece_detail("00300000000000000000")
/// .unwrap();
/// ```
#[derive(Debug)]
pub struct Sendungsverfolgung {
    zt_kennung: Option<String>,
    passwd_zt_kennung: Option<String>,
    entwickler_id: Option<String>,
    passwd_entwicklerportal: Option<String>,
    language_code: String,
    sandbox: bool,
    app_id: Option<String>,
    app_token: Option<String>,
}
impl Sendungsverfolgung {
    /// Retrieves shipment number data from DHL and returns either the body of the response or an HTTP error.
    /// Ruft Daten zur Sendungsnummer von DHL ab und gibt entweder den Body der Antwort zurück oder einen HTTP Fehler.
    pub fn get_piece_detail(
        &self,
        sendungsnummer: &str,
    ) -> Result<std::string::String, Box<dyn Error>> {
        let mut xml_param = {
            if self.sandbox {
                SANDBOX_XML_PARAM.to_string()
            } else {
                PRODUCTION_XML_PARAM.to_string()
            }
        };
        xml_param = xml_param.replace("{language_code}", &self.language_code);

        if !self.sandbox {
            xml_param = xml_param.replace("{sendungsnummer}", sendungsnummer);
            xml_param = xml_param.replace(
                "{zt_kennung}",
                self.zt_kennung
                    .as_ref()
                    .ok_or("zt_kennung not set in production mode.")?,
            );
            xml_param = xml_param.replace(
                "{passwd_zt_kennung}",
                self.passwd_zt_kennung
                    .as_ref()
                    .ok_or("passwd_zt_kennung not set in production mode.")?,
            );
        }
        let url = {
            if self.sandbox {
                BASE_URL_SANDBOX.to_string() + &xml_param
            } else {
                BASE_URL_PRODUCTION.to_string() + &xml_param
            }
        };

        let url = url::Url::parse(&url)?;
        let auth_user = {
            if self.sandbox {
                self.entwickler_id
                    .as_ref()
                    .ok_or("entwickler_id not set in sandbox mode.")?
            } else {
                self.app_id
                    .as_ref()
                    .ok_or("app_id not set in production mode.")?
            }
        };
        let auth_password = {
            if self.sandbox {
                self.passwd_entwicklerportal
                    .as_ref()
                    .ok_or("passwd_entwicklerportal not set sandbox mode.")?
            } else {
                self.app_token
                    .as_ref()
                    .ok_or("app_token not set in production mode.")?
            }
        };

        let client = reqwest::blocking::Client::new();
        let responce = client
            .get(url)
            .basic_auth(auth_user, Some(auth_password))
            .body("")
            .send()?;
        Ok(responce.text()?)
    }
}
