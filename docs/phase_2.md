## 第二阶段：支持多用户开发分析设计
### Task 数据结构修改
1. 增加 user_id 字段

|字段名|字段类型|字段含义说明|默认值|备注|
|-----|------|-----|---|----|
|user_id|i32|用户ID| ||

### 增加 User 数据结构
|字段名|字段类型|字段含义说明|默认值|备注|
|-----|------|-----|---|----|
|id|i32|id| |主键，自增长|
|username|String|用户名| |不允许为空|
|email|String|邮箱| |不允许为空|
|password|String|密码|不允许为空| |
|created_at|datatime|创建时间| |不允许为空|
|updated_at|datatime|修改时间| |不允许为空|


### 实现思路
**实现思路：** 
- 用户登录后，在 cache.json 文件中存储登录用户信息，包括 email、id 等，用户退出登录，清空cache.json 文件用户登录信息；
- 用户登录后，才可对 task 进行增、改、查等操作；

1. 修改 src/services/task_service.rs 文件中 add_task, finish_task, get_tasks, get_unfinished_tasks 几个方法，校验用户是否登录，未登录返回提示信息；
```Rust
fn get_current_user() -> LoginInfo {
    let cache_path = Path::new(constants::CACHE_FILE);
    let string_data = fs::read_to_string(&cache_path).expect("Unable to read file");
    let mut login_info = LoginInfo::new();
    if fs::metadata(&cache_path).unwrap().len() != 0 {
        login_info = serde_json::from_str(&string_data).expect("Unable get json data");
    }

    login_info
}
```
3. 增加 src/models/user.rs 文件，定义 User 与 LoginInfoDTO 数据结构
```Rust
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct LoginInfoDTO {
    pub id: i32,
    pub username: String,
    pub email: String,
}

impl LoginInfoDTO {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
```
4. 增加 src/services/account_service.rs 文件，定义 login 与 logout 方法
```Rust
pub fn login(email: &str, password: &str) -> Result<(), Box<dyn Error>>{
    let cache_path = Path::new(constants::CACHE_FILE);
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&cache_path);

    let string_data = fs::read_to_string(&cache_path).expect("Unable to read file");
    let mut login_info = LoginInfo::new();
    if fs::metadata(&cache_path).unwrap().len() != 0 {
        login_info = serde_json::from_str(&string_data)?;
    }
    login_info.email = email.to_string();
    login_info.id = user.clone().id;
    login_info.name = user.clone().name;
    let cache_json = serde_json::to_string(&login_info)?;

    fs::write(&cache_path, &cache_json).expect("Unable write to file");

    println!("login success.");

    Ok(())
}

pub fn logout() -> Result<(), Box<dyn Error>>{
    let cache_path = Path::new(constants::CACHE_FILE);
    let login_info = LoginInfo::new();
    let cache_json = serde_json::to_string(&login_info)?;

    fs::write(&cache_path, &cache_json).expect("Unable write to file");

    println!("logout success.");

    Ok(())
}
```