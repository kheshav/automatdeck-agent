use std::{env, panic::{self, PanicInfo}, collections::HashMap};
use backtrace::Backtrace;
use crate::feedback;

#[macro_export]
macro_rules! format_err {
    ($($arg:tt)*) => {
        $crate::error::Error::Other(format!($($arg)*))
    };
}

pub fn create_hook<F>(data: Option<HashMap<&'static str, &'static str>>, f:F)
    where F: 'static + Fn(Option<::std::path::PathBuf>,String) -> Result<(), Box<dyn std::error::Error>> + Send + Sync{

   match ::std::env::var("RUST_BACKTRACE") {
        Err(_) => {

            let data = data.unwrap_or({
                let mut data = HashMap::new();
                data.insert("%NAME%", env!("CARGO_PKG_NAME"));
                data.insert("%GITHUB%", env!("CARGO_PKG_REPOSITORY"));
                data
            });

            panic::set_hook(Box::new(move |info: &PanicInfo| {

                let template = r#"
Well, this is embarrassing...

%NAME% had a problem and crashed. To help us diagnose the problem, you can send us a crash report along with related log file.

We have generated a report file at "%PATH%". Submit an issue with the subject of "%NAME% Crash Report"
 and include the report as an attachment.

We take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.

Thank you kindly!
"#;
                
                let mut text = String::from(template);

                for (k, v) in &data {
                    text = text.replace(k, v);
                }

                let path = if text.contains("%PATH%") {
                    
                    let tmp = env::temp_dir().join(format!("report-ad-agent-{}.log", ::uuid::Uuid::new_v4().to_hyphenated().to_string()));
                    text = text.replace("%PATH%", tmp.to_string_lossy().as_ref());
                    Some(tmp)
                } else {
                    None
                };

                //println!("{}", text);
                feedback::format_display(&text);


                let mut payload = String::new();

                let os = if cfg!(target_os = "windows") {
                    "Windows"
                } else if cfg!(target_os = "linux") {
                    "Linux"
                } else if cfg!(target_os = "macos") {
                    "Mac OS"
                } else if cfg!(target_os = "android") {
                    "Android"
                } else {
                    "Unknown"
                };

                payload.push_str(&format!("Name: {}\n", env!("CARGO_PKG_NAME")));
                payload.push_str(&format!("Version: {}\n", env!("CARGO_PKG_VERSION")));
                payload.push_str(&format!("Operating System: {}\n", os));

                if let Some(inner) = info.payload().downcast_ref::<&str>() {
                    payload.push_str(&format!("Cause: {}.\n", &inner));
                }

                if let Some(inner) = info.payload().downcast_ref::<String>() {
                    payload.push_str(&format!("Cause: {}.\n", inner));
                }

                match info.location() {
                    Some(location) => payload.push_str(&format!(
                        "Panic occurred in file '{}' at line {}\n",
                        location.file(),
                        location.line()
                    )),
                    None => payload.push_str("Panic location unknown.\n"),
                };

                payload.push_str(&format!("{:#?}\n", Backtrace::new()));

                log::error!("\n{}",payload);
                log::error!("{}",text);
                f(path, payload).expect("Error generating report")
            }));
        }
        Ok(_) => {}
    }; 

}
