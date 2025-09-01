use cached::stores::DiskCache;
use loading::Loading;
use reqwest::{header::CACHE_CONTROL, Error, Response};
use bincode::{config, Decode, Encode};
use std::time::{Duration, SystemTime};
#[derive(Encode, Decode, Debug)]
struct CacheValue {
  data: String,
  expiration: SystemTime,
}

pub struct HttpClient {
  cache: DiskCache<String, Vec<u8>>,
}

impl HttpClient {
  pub fn new(cache_name: &str, cache_dir: &str) -> Self {
    let cache: DiskCache<String, Vec<u8>> = DiskCache::new(cache_name)
      .set_disk_directory(cache_dir)
      .build()
      .unwrap();
    HttpClient { cache }
  }

  pub async fn fetch_data(&self, url: &str) -> Result<String, Error> {
    let loading = Loading::default();
    let config = config::standard();
    if let Ok(Some(entry)) = self.cache.connection().get(url) {
      let cached_value:(CacheValue, usize) = bincode::decode_from_slice(&entry, config).unwrap();
      if cached_value.0.expiration > SystemTime::now() {
        // Return cached value if not expired
        loading.success(format!("{} icon fetched from cache", url));
        loading.end();
        return Ok(cached_value.0.data);
      } else {
        // Remove expired cache
        self.cache.connection().remove(url).unwrap();
      }
    }
    loading.text(format!("Fetching {} icon from network...", url));
    let result = reqwest::get(url).await;
    match result {
      Ok(response) => {
        if response.status().is_success() {
          let cache_duration = get_cache_duration(&response).unwrap_or(Duration::from_secs(60));
          let text = response.text().await?;
          loading.success(format!("{} icon fetched from network", url));
          loading.end();
          let cache_value = CacheValue {
            data: text,
            expiration: SystemTime::now() + cache_duration,
          };
          let serialized_data = bincode::encode_to_vec(&cache_value, config).unwrap();
          self
            .cache
            .connection()
            .insert(url.to_string(), serialized_data)
            .unwrap();
          return Ok(cache_value.data);
        } else {
          loading.fail(format!("{} icon fetch err: {:?}", url, response.status()));
          loading.end();
          return Err(response.error_for_status().unwrap_err());
        }
      }
      Err(e) => {
        loading.fail(format!("{} icon fetch err: {:?}", url, e));
        loading.end();
        return Err(e);
      }
    }
  }
}

fn get_cache_duration(response: &Response) -> Option<Duration> {
  if let Some(cache_control) = response.headers().get(CACHE_CONTROL) {
    if let Ok(cache_control_str) = cache_control.to_str() {
      for directive in cache_control_str.split(',') {
        let directive = directive.trim();
        if directive.starts_with("max-age=") {
          if let Ok(seconds) = directive[8..].parse::<u64>() {
            return Some(Duration::from_secs(seconds));
          }
        }
      }
    }
  }
  None
}
