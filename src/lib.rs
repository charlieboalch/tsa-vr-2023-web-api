use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::web;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::{from_reader, from_str, to_string};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimalData {
    pub animals: Vec<Animal>,
    pub pagination: Pagination,
    pub timestamp: Option<u64>
}

impl AnimalData {
    pub fn set_timestamp(&mut self) {
        self.timestamp = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Animal {
    pub id: Option<i64>,
    #[serde(rename = "organization_id")]
    pub organization_id: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub species: Option<String>,
    pub breeds: Option<Breeds>,
    pub colors: Option<Colors>,
    pub age: Option<String>,
    pub gender: Option<String>,
    pub size: Option<String>,
    pub coat: Option<String>,
    pub attributes: Option<Attributes>,
    pub environment: Option<Environment>,
    pub tags: Option<Vec<String>>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "organization_animal_id")]
    pub organization_animal_id: Option<String>,
    pub photos: Option<Vec<Photo>>,
    #[serde(rename = "primary_photo_cropped")]
    pub primary_photo_cropped: Option<PrimaryPhotoCropped>,
    pub status: Option<String>,
    #[serde(rename = "status_changed_at")]
    pub status_changed_at: Option<String>,
    #[serde(rename = "published_at")]
    pub published_at: Option<String>,
    pub distance: Option<f64>,
    pub contact: Option<Contact>,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Breeds {
    pub primary: Option<String>,
    pub secondary: Option<String>,
    pub mixed: Option<bool>,
    pub unknown: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Colors {
    pub primary: Option<String>,
    pub secondary: Option<String>,
    pub tertiary: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(rename = "spayed_neutered")]
    pub spayed_neutered: Option<bool>,
    #[serde(rename = "house_trained")]
    pub house_trained: Option<bool>,
    pub declawed: Option<bool>,
    #[serde(rename = "special_needs")]
    pub special_needs: Option<bool>,
    #[serde(rename = "shots_current")]
    pub shots_current: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    pub children: Option<bool>,
    pub dogs: Option<bool>,
    pub cats: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
    pub full: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryPhotoCropped {
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
    pub full: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<Address>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postcode: Option<String>,
    pub country: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: Option<SelfField>,
    #[serde(rename = "type")]
    pub type_field: Option<Type>,
    pub organization: Option<Organization>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfField {
    pub href: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub href: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub href: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    #[serde(rename = "count_per_page")]
    pub count_per_page: i64,
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(rename = "current_page")]
    pub current_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
}

#[derive(Deserialize, Hash)]
pub struct AnimalOptions {
    species: String,
    size: String,
    gender: String,
    age: String,
    good_with_kids: String,
    good_with_animals: String,
    house_trained: String,
    location: u32
}


// Check if the token has expired yet
// If so, refresh it
// If not, go ahead and make the request
pub async fn get_and_cache(options: AnimalOptions, state: &web::Data<State>) -> Result<AnimalData, Box<dyn Error>> {
    let res = check_cache(&options).await;
    if res.is_some() {
        let mut data = res.unwrap();
        create_short_url(&mut data).await.ok();
        let timestamp = data.timestamp;
        if timestamp.is_some() && timestamp.unwrap() + 43200 > SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() {
            return Ok(data);
        }
    }
    refresh_token(&state).await;

    let request = reqwest::Client::builder()
        .build()?;

    let resp = request.get(format!("https://api.petfinder.com/v2/animals?type={}&size={}&gender={}&age={}&good_with_children={}&good_with_dogs={}&good_with_cats={}&house_trained={}&location={}&limit=100",
            &options.species,
            &options.size,
            &options.gender,
            &options.age,
            &options.good_with_kids,
            &options.good_with_animals,
            &options.good_with_animals,
            &options.house_trained,
            &options.location
        ))
        .bearer_auth(&*state.access_token.lock().unwrap())
        .send()
        .await?;

    let text = &resp.text().await?;

    let mut animal_data: AnimalData = from_str(&text)?;
    animal_data.set_timestamp();
    cache_data(&options, &animal_data).await?;
    create_short_url(&mut animal_data).await.ok();

    Ok(animal_data)
}

async fn create_short_url(animal_data: &mut AnimalData) -> Result<(), Box<dyn Error>> {
    if !Path::new("./cache/url-table.json").exists() {
        let mut file = File::create("./cache/url-table.json")?;
        let table = UrlTable::new();
        file.write_all(to_string(&table)?.as_ref())?;
    }

    let contents = fs::read_to_string("./cache/url-table.json")?;
    let mut table: UrlTable = from_str(&contents)?;
    // now add a url shortener
    for animal in &mut animal_data.animals {
        let animal_url = animal.url.clone().unwrap_or(String::from("unknown"));
        let id = animal.id.clone().unwrap_or(-1);
        let org_id = animal.organization_id.clone().unwrap_or(String::from("unknown"));

        let mut flag = false;
        for values in &table.urls {
            if values.1 == &animal_url {
                animal.url = Some(format!("phqsh.tech/vr/{}", values.0));
                flag = true;
                break;
            }
        }

        //if not found
        if !flag {
            // format- id-orgid
            // example- 65087362-GA575
            let url = &format!("{}-{}",
                               id,
                               org_id);

            println!("{} -> {}", url, animal_url);

            table.urls.insert(url.clone(), animal_url);
            animal.url = Some(String::from(format!("phqsh.tech/vr/{}", url)));
        }
    }

    let mut file = File::create("./cache/url-table.json")?;
    file.write_all(to_string(&table)?.as_ref())?;
    Ok(())
}

async fn refresh_token(state: &web::Data<State>) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    if since_the_epoch.as_secs() < *state.access_token_expiration.lock().unwrap() {
        return;
    }

    let client = reqwest::Client::new();

    let resp = client.post("https://api.petfinder.com/v2/oauth2/token")
        .body(to_string(&AuthData {
            grant_type: "client_credentials",
            client_id: &**state.client_id.lock().unwrap(),
            client_secret: &**state.client_secret.lock().unwrap(),
        }).unwrap())
        .send()
        .await
        .unwrap();

    let text = resp.text().await.unwrap();

    let token_response: TokenResponse = from_str(&text).expect("Failed to parse token response");
    let new_expiration = since_the_epoch.as_secs() + token_response.expires_in;

    let mut expiration = state.access_token_expiration.lock().unwrap();
    *expiration = new_expiration;
    let mut access_token = state.access_token.lock().unwrap();
    *access_token = token_response.access_token;
}

async fn cache_data(options: &AnimalOptions, animal_data: &AnimalData) -> Result<(), Box<dyn Error>> {
    if !Path::new("./cache").exists() {
        fs::create_dir("./cache")?;
    }

    let mut hasher = DefaultHasher::new();
    options.hash(&mut hasher);
    let hash = hasher.finish();
    let mut file = File::create(format!("./cache/{}.json", hash))?;

    let json = to_string(&animal_data)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

async fn check_cache(options: &AnimalOptions) -> Option<AnimalData> {
    let mut hasher = DefaultHasher::new();
    options.hash(&mut hasher);
    let hash = hasher.finish();

    let file = File::open(format!("./cache/{}.json", hash)).ok();
    match file {
        Some(path) => {
            let animal_data: Option<AnimalData> = from_reader(path).ok();
            animal_data
        },
        None => return None
    }
}

pub async fn look_up_url(path: String) -> Result<Option<String>, Box<dyn Error>> {
    let contents = fs::read_to_string("./cache/url-table.json").unwrap();
    let table: UrlTable = from_str(&contents).unwrap();

    for values in &table.urls {
        if values.0 == &path {
            return Ok(Some(values.1.clone()));
        }
    }

    return Ok(None);
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "expires_in")]
    pub expires_in: u64,
    #[serde(rename = "access_token")]
    pub access_token: String,
}

pub struct State {
    pub requests: Mutex<u32>,
    pub access_token: Mutex<String>,
    pub access_token_expiration: Mutex<u64>,
    pub client_id: Mutex<String>,
    pub client_secret: Mutex<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthData<'a> {
    pub grant_type: &'a str,
    pub client_id: &'a str,
    pub client_secret: &'a str,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrlTable {
    pub urls: HashMap<String, String>,
}

impl UrlTable {
    pub fn new() -> UrlTable {
        UrlTable {
            urls: HashMap::new(),
        }
    }
}