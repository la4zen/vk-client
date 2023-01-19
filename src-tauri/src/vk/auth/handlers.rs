use core::time;

#[tauri::command]
pub async fn vk_authorize(app : tauri::AppHandle) {
    #[cfg(debug_assertions)]
    let login_url = reqwest::Url::parse_with_params("https://oauth.vk.com/authorize",
      &[("client_id", "2685278"), ("scope", "1073737727"), ("revoke", "1"), ("response_type","token")]).unwrap();
    let window = tauri::WindowBuilder::new(
      &app,
      "login_window",
      tauri::WindowUrl::External(
        login_url,
      )
    ).center().title("Авторизация").build().unwrap();
    std::thread::spawn(move || {
      loop {
        window.eval("if (window.location.href.includes('access_token')) window.location.href = `http://localhost:27834/logged_in/${window.location.href.slice(32)}`; else console.log('unlogged')").unwrap();
        let monitor = window.current_monitor();
        match monitor {
          Ok(_) => {}
          Err(_) => {
            break;
          }
        }
        std::thread::sleep(time::Duration::from_millis(200));
      };
    });
}