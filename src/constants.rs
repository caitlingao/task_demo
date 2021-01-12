// constants
pub const SINGULAR_PLURAL_THRESHOLD: i32 = 1;
pub const TASK_ID_INCREMENT_THRESHOLD: i32 = 1;

// file path
pub const TMP_DIR: &str = "tmp";
pub const DOWNLOAD_DIR: &str = "tmp/download";
pub const USER_FILE: &str = ".todo-config.json";
pub const TASKS_FILE: &str = "tmp/tasks.json";
pub const CACHE_FILE: &str = "tmp/cache.json";

// message
// login logout
pub const LOGIN_SUCCESS: &str = "Login success.";
pub const LOGIN_FAILED: &str = "User does not exist or wrong password.";
pub const ASK_FOR_LOGIN: &str = "Please login first.";
pub const LOGOUT_SUCCESS: &str = "Logout success.";

// file
pub const UNABLE_TO_READ_FILE: &str = "Unable to read file.";
pub const UNABLE_WRITE_TO_FILE: &str = "Unable write to file.";
pub const FILE_NOT_EXIST: &str = "File does not exist.";
pub const GET_FILE_DATA_WRONG: &str = "Get data from file wrong.";
// task
pub const TASK_DOES_NOT_EXIST: &str = "Task does not exist.";
